use std::{
    fs::File,
    io::{BufRead, BufReader},
};


#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

impl Direction {
    fn from_str(s: &str) -> Self {
        let (d, n) = s.split_at(1);
        match d {
            "L" => Self::Left(i32::from_str_radix(n, 10).unwrap()),
            "R" => Self::Right(i32::from_str_radix(n, 10).unwrap()),
            _ => panic!("Unknown direction {d}"),
        }
    }

    fn rotate(&self, dial: i32) -> i32 {
        match self {
            Self::Left(n) => dial - n,
            Self::Right(n) => dial + n,

        }
    } 
}

pub fn run() {
    let file = File::open("src/bin/day1/part1.txt").unwrap();
    let reader = BufReader::new(&file);

    let directions: Vec<Direction> = reader
        .lines()
        .map(|l| Direction::from_str(&l.unwrap()))
        .collect();

    let dial = directions.iter().fold(55i32, |acc, d| d.rotate(acc));

    println!("{:?}", dial);
}
