use tokio::io::{BufReader as TKOBufReader, AsyncBufReadExt};
use tokio::process::Command;

use std::io::{self, Read, BufReader as StdBufReader, BufRead};
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut geodsolve_proc = Command::new("bin/times_2")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn command");


    while true {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
                if n == 0 {
                    println!("EOF");
                    break;
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }


    let geodsolve_stdout = geodsolve_proc.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut geodsolve_reader = TKOBufReader::new(geodsolve_stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async {
        let status = geodsolve_proc.await
            .expect("geodsolve_proc process encountered an error");

        println!("geodsolve_proc status was: {}", status);
    });

    while let Some(line) = geodsolve_reader.next_line().await? {
        println!("Line: {}", line);
    }

    Ok(())
}

