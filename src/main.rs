use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::Command;

use std::env;
use std::fmt;

mod calculation;
mod geod_error;

#[derive(Debug)]
enum Error {
    ArgumentError,
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
            let calcs: Vec<Calculation> =
                vec![Calculation::DirectP1ToP2, Calculation::DirectP2ToP1, Calculation::Inverse];
            run(&args[1], &calcs).await
        }
        _ => {
            usage();
            Err(Error::ArgumentError.into())
        }
    }
}

fn usage() {
    println!(
        "USAGE:
validate_geodsolve <path-string>

WHERE:
    path-string: path to a binary like GeodSolve
"
    );
}

async fn run(bin_name: &str, calcs: &Vec<Calculation>) -> Result<(), Box<dyn std::error::Error>> {
    let mut test_case_reader = BufReader::new(tokio::io::stdin()).lines();
    let geod = Geodesic::wgs84();

    // Direct Calculation Process

    let mut direct_proc = Command::new(bin_name)
        .arg("-p")
        .arg("10")
        .arg("-f")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn direct calculation command");

    let mut direct_writer = BufWriter::new(
        direct_proc
            .stdin
            .take()
            .expect("geodsolve did not have a handle to stdin"),
    );

    let mut direct_reader = BufReader::new(
        direct_proc
            .stdout
            .take()
            .expect("geodsolve did not have a handle to stdout"),
    )
    .lines();

    let mut max_position_error: Option<DirectError> = Option::None;
    let mut max_azi_error: Option<DirectError> = Option::None;
    let mut max_m12_error: Option<DirectError> = Option::None;

    // Inverse Calculation Process
    let mut inverse_proc = Command::new(bin_name)
        .arg("-i")
        .arg("-p")
        .arg("10")
        .arg("-f")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn indirect calculation command");

    let mut inverse_writer = BufWriter::new(
        inverse_proc
            .stdin
            .take()
            .expect("geodsolve did not have a handle to stdin"),
    );

    let mut inverse_reader = BufReader::new(
        inverse_proc
            .stdout
            .take()
            .expect("geodsolve did not have a handle to stdout"),
    )
    .lines();

    let mut max_distance_error: Option<InverseError> = None;

    let mut line_number = 0;
    while let Some(test_case_line) = test_case_reader.next_line().await? {
        let test_case_fields = test_case_line
            .split(" ")
            .map(|s| s.parse::<f64>().unwrap())
            .collect();

        let calc = Calculation::DirectP1ToP2;
        if calcs.contains(&calc) {
            let input = format_input(&test_case_fields, calc);
            direct_writer
                .write_all(input.as_bytes())
                .await
                .expect("write failed");
            direct_writer.flush().await.expect("flush failed");

            let direct_error: DirectError = match direct_reader.next_line().await {
                Err(e) => panic!("err: {:?}", e),
                Ok(None) => panic!("geodsolve should have output after giving it input"),
                Ok(Some(geodsolve_output_line)) => {
                    let output_fields: Vec<f64> = geodsolve_output_line
                        .split(" ")
                        .map(|s| s.parse::<f64>().unwrap())
                        .collect();
                    DirectError::new(
                        output_fields[3],
                        output_fields[4],
                        output_fields[5],
                        output_fields[8],
                        test_case_fields[3],
                        test_case_fields[4],
                        test_case_fields[5],
                        test_case_fields[8],
                        &geod,
                        line_number,
                        calc,
                    )
                }
            };
            max_position_error = max_error(max_position_error, direct_error, |e| e.position_error);
            max_azi_error = max_error(max_azi_error, direct_error, |e| e.azi_error);
            max_m12_error = max_error(max_m12_error, direct_error, |e| e.m12_error);
        }

        let calc = Calculation::DirectP2ToP1;
        if calcs.contains(&calc) {
            let input = format_input(&test_case_fields, calc);
            direct_writer
                .write_all(input.as_bytes())
                .await
                .expect("write failed");
            direct_writer.flush().await.expect("flush failed");

            let direct_error: DirectError = match direct_reader.next_line().await {
                Err(e) => panic!("err: {:?}", e),
                Ok(None) => panic!("geodsolve should have output after giving it input"),
                Ok(Some(geodsolve_output_line)) => {
                    let output_fields: Vec<f64> = geodsolve_output_line
                        .split(" ")
                        .map(|s| s.parse::<f64>().unwrap())
                        .collect();
                    DirectError::new(
                        output_fields[3],
                        output_fields[4],
                        output_fields[5],
                        output_fields[8],
                        test_case_fields[0],
                        test_case_fields[1],
                        test_case_fields[2],
                        -test_case_fields[8],
                        &geod,
                        line_number,
                        calc,
                    )
                }
            };
            max_position_error = max_error(max_position_error, direct_error, |e| e.position_error);
            max_azi_error = max_error(max_azi_error, direct_error, |e| e.azi_error);
            max_m12_error = max_error(max_m12_error, direct_error, |e| e.m12_error);
        }

        let calc = Calculation::Inverse;
        if calcs.contains(&calc) {
            let input = format_input(&test_case_fields, calc);
            inverse_writer
                .write_all(input.as_bytes())
                .await
                .expect("write failed");
            inverse_writer.flush().await.expect("flush failed");

            let inverse_error: InverseError = match inverse_reader.next_line().await {
                Err(e) => panic!("err: {:?}", e),
                Ok(None) => panic!("geodsolve should have output after giving it input"),
                Ok(Some(geodsolve_output_line)) => {
                    let output_fields: Vec<f64> = geodsolve_output_line
                        .split(" ")
                        .map(|s| s.parse::<f64>().unwrap())
                        .collect();
                    InverseError::new(
                        output_fields[6],
                        test_case_fields[6],
                        &geod,
                        line_number,
                    )
                }
            };
            max_distance_error = max_error(max_distance_error, inverse_error, |e| e.s12_error);
        }

        line_number += 1;
    }

    let mult: f64 = 1.0e9;
    let max_position_error = max_position_error.unwrap();
    let max_azi_error = max_azi_error.unwrap();
    let max_m12_error = max_m12_error.unwrap();

    println!(
        "0 {:.2} {}",
        max_position_error.position_error * mult,
        max_position_error.line_number
    );
    println!(
        "1 {:.2} {}",
        max_azi_error.azi_error * mult,
        max_azi_error.line_number
    );
    println!(
        "2 {:.2} {}",
        max_m12_error.m12_error * mult,
        max_m12_error.line_number
    );

    let max_distance_error = max_distance_error.unwrap();
    println!(
        "3 {:.2} {}",
        max_distance_error.s12_error * mult,
        max_distance_error.line_number
    );

    Ok(())
}

use calculation::Calculation;
use geod_error::{DirectError, InverseError};
use geographiclib_rs::Geodesic;

fn format_input(fields: &Vec<f64>, calc: Calculation) -> String {
    match calc {
        Calculation::DirectP1ToP2 => {
            format!("{} {} {} {}\n", fields[0], fields[1], fields[2], fields[6])
        }
        Calculation::DirectP2ToP1 => {
            format!("{} {} {} {}\n", fields[3], fields[4], fields[5], -fields[6])
        }
        Calculation::Inverse => {
            format!("{} {} {} {}\n", fields[0], fields[1], fields[3], fields[4])
        }
    }
}

fn max_error<T, F>(a: Option<T>, b: T, f: F) -> Option<T>
where
    F: Fn(T) -> f64,
    T: Copy
{
    Some(match a {
        None => b,
        Some(a) => {
            if f(b) > f(a) {
                b
            } else {
                a
            }
        }
    })
}
