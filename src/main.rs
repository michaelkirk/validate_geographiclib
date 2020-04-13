use tokio::io::{BufReader, AsyncBufReadExt, BufWriter, AsyncWriteExt};
use tokio;
use tokio::process::Command;

// use std::io::{self, Read, BufReader as StdBufReader, BufRead};
// use std::io::LineWriter;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut geodsolve_proc = Command::new("bin/times_2")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn command");

    let geodsolve_stdin = geodsolve_proc.stdin.take()
        .expect("child did not have a handle to stdin");

    let mut geodsolve_writer = BufWriter::new(geodsolve_stdin);

    let geodsolve_stdout = geodsolve_proc.stdout.take()
        .expect("child did not have a handle to stdout");

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    // let foo = tokio::spawn(async {
    //     println!("spawned");
    //
    //     // let status = geodsolve_proc.await
    //     //     .expect("geodsolve_proc process encountered an error");
    //     //
    //     // println!("geodsolve_proc status was: {}", status);
    //
    //     let mut geodsolve_reader = BufReader::new(geodsolve_stdout).lines();
    //     // println!("wrote test_case_line: {}\nReading next line from geodsolve...", test_case_line);
    //     while let Ok(Some(geodsolve_output_line)) = geodsolve_reader.next_line().await {
    //         println!("geodsolve_output_line: {}", geodsolve_output_line);
    //     }
    //
    //     println!("done reading output from GeodSolve");
    // });

    let mut geodsolve_reader = BufReader::new(geodsolve_stdout).lines();
    let mut test_case_reader = BufReader::new(tokio::io::stdin()).lines();
    while let Some(test_case_line) = test_case_reader.next_line().await? {
        println!("read test_case_line: {}", test_case_line);
        geodsolve_writer.write_all(test_case_line.as_bytes()).await.expect("write failed");
        geodsolve_writer.write_all("\n".as_bytes()).await.expect("write2 failed");
        geodsolve_writer.flush().await.expect("flush failed");

        if let Ok(Some(geodsolve_output_line)) = geodsolve_reader.next_line().await {
            println!("geodsolve_output_line: {}", geodsolve_output_line);
        } else {
            println!("geodsolve output err or none");
        }
    }
    // drop(geodsolve_writer);
    println!("end of input");


    // println!("awaiting foo");
    // foo.await;
    // println!("awaited foo");

    Ok(())
}

