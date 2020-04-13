use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut geodsolve_cmd = Command::new("bin/times_2");

    // Specify that we want the command's standard output piped back to us.
    // By default, standard input/output/error will be inherited from the
    // current process (for example, this means that standard input will
    // come from the keyboard and standard output/error will go directly to
    // the terminal if this process is invoked from the command line).
    geodsolve_cmd.stdout(Stdio::piped());

    let mut geodsolve_proc = geodsolve_cmd.spawn()
        .expect("failed to spawn command");

    let geodsolve_stdout = geodsolve_proc.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut geodsolve_reader = BufReader::new(geodsolve_stdout).lines();

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

