use tokio::io::{BufReader, AsyncBufReadExt, BufWriter, AsyncWriteExt};
use tokio::process::Command;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut test_case_reader = BufReader::new(tokio::io::stdin()).lines();

    let mut geodsolve_proc = Command::new("bin/times_2")
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
        //eprintln!("read test_case_line: {}", test_case_line);
        geodsolve_input.write_all(test_case_line.as_bytes()).await.expect("write failed");
        geodsolve_input.write_all("\n".as_bytes()).await.expect("write2 failed");
        geodsolve_input.flush().await.expect("flush failed");

        match geodsolve_output.next_line().await {
            Err(e) => panic!("err: {:?}", e),
            Ok(None) => panic!("geodsolve should have output after giving it input"),
            Ok(Some(geodsolve_output_line)) => {
                compare(test_case_line, geodsolve_output_line);
            }
        }
    }

    eprintln!("done");
    Ok(())
}

fn compare(test_case: String, geodsolve_output: String) {
    eprintln!("test_case: {} -> geodsolve_output_line: {}", test_case, geodsolve_output);
}

