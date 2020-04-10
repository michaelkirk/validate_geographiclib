use std::io::{ self, BufRead };

fn main() -> io::Result<()> {
    let mut i = 0;
    for line in io::stdin().lock().lines() {
        i += 1;
        println!("line {}: {}", i, line.ok().unwrap());
    }
    Ok(())
}

