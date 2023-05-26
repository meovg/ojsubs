use std::io::{self, BufRead};

// #[derive(Debug)]
enum MoveType {
    Up,
    Down,
    Forward,
}

struct Move(MoveType, i32);

impl Move {
    fn from_input(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(" ").collect();
        Self(
            match tokens[0] {
                "forward" => MoveType::Forward,
                "up" => MoveType::Up,
                "down" => MoveType::Down,
                _ => {
                    panic!("what did you mean by this bruh");
                }
            },
            tokens[1].parse::<i32>().unwrap(),
        )
    }
}

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
    aim: i32,
}

impl Point {
    fn apply(&mut self, m: &Move) {
        // println!("{:?} {}", m.0, m.1);
        match m.0 {
            MoveType::Forward => {
                self.x += m.1;
                self.y += self.aim * m.1
            }
            MoveType::Up => self.aim -= m.1,
            MoveType::Down => self.aim += m.1,
        }
    }
}

fn main() {
    let mut submarine = Point::default();
    for line in io::stdin().lock().lines() {
        let cmd = Move::from_input(&line.unwrap());
        submarine.apply(&cmd);
    }
    println!("{}", submarine.x * submarine.y);
}
