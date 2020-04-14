use tokio::io::{BufReader, AsyncBufReadExt, BufWriter, AsyncWriteExt};
use tokio::process::Command;
use std::process::Stdio;

use std::env;
use std::fmt;

mod geod_error;

#[derive(Debug)]
enum Error {
    ArgumentError
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    eprintln!("args: {:?}", args);

    match args.len() {
        2 => {
            let calcs: Vec<Calculation> = vec![Calculation::DirectP1ToP2, Calculation::DirectP2ToP1];
            run(&args[1], &calcs).await
        }
        _ => {
            usage();
            Err(Error::ArgumentError.into())
        }
    }
}

fn usage() {
    println!("USAGE:
validate_geodsolve <path-string>

WHERE:
    path-string: path to a binary like GeodSolve
");
}

async fn run(bin_name: &str, calcs: &Vec<Calculation>) -> Result<(), Box<dyn std::error::Error>> {
    let mut test_case_reader = BufReader::new(tokio::io::stdin()).lines();
    let geod = Geodesic::wgs84();

    if calcs.contains(&Calculation::DirectP1ToP2) || calcs.contains(&Calculation::DirectP2ToP1) {
        let mut geodsolve_proc = Command::new(bin_name)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to spawn command");

        let mut geodsolve_writer = BufWriter::new(
            geodsolve_proc.stdin.take().expect("geodsolve did not have a handle to stdin")
        );

        let mut geodsolve_reader = BufReader::new(
            geodsolve_proc.stdout.take().expect("geodsolve did not have a handle to stdout")
        ).lines();

        let mut max_position_error: Option<f64> = Option::None;
        let mut max_azi_error: Option<f64> = Option::None;

        while let Some(test_case_line) = test_case_reader.next_line().await? {
            let test_case_fields = test_case_line.split(" ").map(|s| s.parse::<f64>().unwrap()).collect();

            if calcs.contains(&Calculation::DirectP1ToP2) {
                let input = format_input(&test_case_fields, Calculation::DirectP1ToP2);
                geodsolve_writer.write_all(input.as_bytes()).await.expect("write failed");
                geodsolve_writer.flush().await.expect("flush failed");

                let direct_error: DirectError = match geodsolve_reader.next_line().await {
                    Err(e) => panic!("err: {:?}", e),
                    Ok(None) => panic!("geodsolve should have output after giving it input"),
                    Ok(Some(geodsolve_output_line)) => {
                        let output_fields: Vec<f64> = geodsolve_output_line.split(" ").map(|s| s.parse::<f64>().unwrap()).collect();
                        DirectError::new(
                            &geod,
                            output_fields[0],
                            output_fields[1],
                            output_fields[2],
                            test_case_fields[3],
                            test_case_fields[4],
                            test_case_fields[5],
                        )
                    }
                };
                max_position_error = Some(
                    match max_position_error {
                        None => direct_error.position_error,
                        Some(prev_max) => prev_max.max(direct_error.position_error)
                    }
                );
                max_azi_error = Some(
                    match max_azi_error {
                        None => direct_error.azi_error,
                        Some(prev_max) => prev_max.max(direct_error.azi_error)
                    }
                );
            }

            if calcs.contains(&Calculation::DirectP2ToP1) {
                let input = format_input(&test_case_fields, Calculation::DirectP2ToP1);
                geodsolve_writer.write_all(input.as_bytes()).await.expect("write failed");
                geodsolve_writer.flush().await.expect("flush failed");

                let direct_error: DirectError = match geodsolve_reader.next_line().await {
                    Err(e) => panic!("err: {:?}", e),
                    Ok(None) => panic!("geodsolve should have output after giving it input"),
                    Ok(Some(geodsolve_output_line)) => {
                        let output_fields: Vec<f64> = geodsolve_output_line.split(" ").map(|s| s.parse::<f64>().unwrap()).collect();
                        DirectError::new(
                            &geod,
                            output_fields[0],
                            output_fields[1],
                            output_fields[2],
                            test_case_fields[0],
                            test_case_fields[1],
                            test_case_fields[2],
                        )
                    }
                };
                max_position_error = Some(
                    match max_position_error {
                        None => direct_error.position_error,
                        Some(prev_max) => prev_max.max(direct_error.position_error)
                    }
                );
                max_azi_error = Some(
                    match max_azi_error {
                        None => direct_error.azi_error,
                        Some(prev_max) => prev_max.max(direct_error.azi_error)
                    }
                );
            }
        }

        let mult: f64 = 1.0e9;
        println!("{} {}", max_position_error.unwrap() * mult, max_azi_error.unwrap() * mult);
    }

    eprintln!("done");
    Ok(())
}

use geographiclib_rs::Geodesic;
use geod_error::DirectError;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Calculation {
    DirectP1ToP2,
    DirectP2ToP1,
}

fn format_input(fields: &Vec<f64>, calc: Calculation) -> String {
    match calc {
        Calculation::DirectP1ToP2 => {
            format!("{} {} {} {}\n", fields[0], fields[1], fields[2], fields[6])
        }
        Calculation::DirectP2ToP1 => {
            format!("{} {} {} {}\n", fields[3], fields[4], fields[5], -fields[6])
        }
    }
}

