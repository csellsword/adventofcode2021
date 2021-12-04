use std::fs;
use std::env;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut prev = match lines.next() {
        Some(res) => {
            let dummy = res?;
            dummy.parse::<u64>().unwrap()
        }
        None => {
            println!("Nothing");
            return Ok(())
        },
    };

    let mut count = 0;
    while let Some(line) = lines.next() {
        let curr = line.unwrap().parse::<u64>().unwrap();
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    println!("{}", count);

    Ok(())
}
