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
            run(&args[1], Calculation::DirectFromP1).await
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

async fn run(bin_name: &String, calc: Calculation) -> Result<(), Box<dyn std::error::Error>> {
    let mut test_case_reader = BufReader::new(tokio::io::stdin()).lines();

    let mut geodsolve_proc = Command::new(bin_name)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn command");

    let mut geodsolve_input = BufWriter::new(
        geodsolve_proc.stdin.take().expect("geodsolve did not have a handle to stdin")
    );

    let mut geodsolve_output = BufReader::new(
        geodsolve_proc.stdout.take().expect("geodsolve did not have a handle to stdout")
    ).lines();

    while let Some(test_case_line) = test_case_reader.next_line().await? {
        let input = parse_input(&test_case_line, calc);
        geodsolve_input.write_all(input.as_bytes()).await.expect("write failed");
        geodsolve_input.flush().await.expect("flush failed");

        match geodsolve_output.next_line().await {
            Err(e) => panic!("err: {:?}", e),
            Ok(None) => panic!("geodsolve should have output after giving it input"),
            Ok(Some(geodsolve_output_line)) => {
                compare(test_case_line, geodsolve_output_line, calc);
            }
        }
    }

    eprintln!("done");
    Ok(())
}

use geographiclib_rs::Geodesic;
use geod_error::DirectError;

fn compare(test_case: String, geodsolve_output: String, calc: Calculation) {
    let test_case_fields: Vec<f64> = test_case.split(" ").map(|s| s.parse::<f64>().unwrap()).collect();
    let output_fields: Vec<f64> = geodsolve_output.split(" ").map(|s| s.parse::<f64>().unwrap()).collect();

    let geod = Geodesic::wgs84();
    let error = DirectError::new(
        Geodesic::wgs84(),
        output_fields[0],
        output_fields[1],
        output_fields[2],
        test_case_fields[3],
        test_case_fields[4],
        test_case_fields[5],
    );

    println!("error: {:?}", error);
}

#[derive(Clone, Copy)]
enum Calculation {
    DirectFromP1
}

fn parse_input(line: &str, calc: Calculation) -> String {
    match calc {
        Calculation::DirectFromP1 => {
            let fields: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
            format!("{} {} {} {}\n", fields[0], fields[1], fields[2], fields[6])
        }
    }
}

