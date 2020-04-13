use std::io::{self, BufRead, stdin};
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
    "the quick brown fox jumped over the lazy dog\n";


fn main() -> io::Result<()> {
    // wc()?;
    geodsolve()
}

fn wc() -> io::Result<()> {
    // Spawn the `wc` command
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }

    Ok(())
}

fn geodsolve() -> io::Result<()> {
    // TODO pass as argument
    let geodsolve_bin = "bin/times_2";

    let mut geodsolve_process = Command::new(geodsolve_bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn geodsolve bin");

    let stdin = geodsolve_process.stdin.as_mut().unwrap();
    let mut line_number = 0;
    for line in io::stdin().lock().lines() {
        line_number += 1;
        let line = line.unwrap();
        match stdin.write_all(line.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}",
                               why),
            Ok(_) => println!("sent line to wc"),
        }
        match stdin.write_all("\n".as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}",
                               why),
            Ok(_) => println!("sent line to wc"),
        }
    }

    println!("will wait");
    geodsolve_process.wait();
    println!("did wait");
    // geodsolve_process.stdin.as_mut().unwrap().write_all("\n".as_bytes());

    //}


    //
    //     // Write a string to the `stdin` of `wc`.
    //     //
    //     // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    //     // must have one, we can directly `unwrap` it.
    //     match geodsolve_stdin.write_all(&line.as_bytes()) {
    //         Err(why) => panic!("couldn't write to stdin: {}", why),
    //         Ok(_) => println!("sent line {} to geodsolve", line_number),
    //     }
    // }

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match geodsolve_process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }

    //let _result = child.wait().unwrap();


    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    // let mut s = String::new();
    // match geodsolve_stdout.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read stdout: {}", why),
    //     Ok(_) => print!("responded with:\n{}", s),
    // }

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    // let mut s = String::new();
    // match geodsolve_process.stdout.unwrap().read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read wc stdout: {}",
    //                        why),
    //     Ok(_) => print!("geodsolve responded with:\n{}", s),
    // }
    //
    // println!("output: {}", s);
    // }

    Ok(())
}

