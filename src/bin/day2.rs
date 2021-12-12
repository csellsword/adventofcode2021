use std::fs;
use std::env;
use std::io::{self, BufRead};

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn parse_direction(raw_line: &String) -> Direction {
    let split_line: Vec<&str> = raw_line.split(' ').collect();
    match split_line[0] {
        "forward" => Direction::Forward(split_line[1].parse::<i32>().unwrap()),
        "down" => Direction::Down(split_line[1].parse::<i32>().unwrap()),
        "up" => Direction::Up(split_line[1].parse::<i32>().unwrap()),
        word => {
            panic!("Unknown command {}", word);
        }
    }
}

struct Submarine {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine{
            aim: 0,
            horizontal: 0,
            depth:0
        }
    }

    fn thrust(&mut self, amt: i32){
        self.horizontal += amt;
        self.depth += self.aim * amt;
    }

    fn adjust(&mut self, amt: i32){
        self.aim += amt;
    }

    fn execute(&mut self, instructions: &Vec<Direction>) {
        for ins in instructions {
            match ins {
                Direction::Forward(amt) => self.thrust(*amt),
                Direction::Down(amt) => self.adjust(*amt),
                Direction::Up(amt) => self.adjust(-amt),
            }
        }
        println!("Part Two: {}", self.horizontal * self.depth);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(file);
    let mut directions: Vec<Direction> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => directions.push(parse_direction(&text)),
            _ => panic!("Read failed for line, {:?}", line),
        }
    }

    let mut depth: i32 = 0;
    let mut dist: i32 = 0;
    for ins in &directions {
        match ins {
            Direction::Forward(num) => dist = dist + num,
            Direction::Down(num) => depth = depth + num,
            Direction::Up(num) => depth = depth - num,
        }
    }

    println!("Part One: {}", depth * dist);

    let mut sub = Submarine::new();
    sub.execute(&directions);

    Ok(())
}