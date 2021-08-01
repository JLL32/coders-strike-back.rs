use std::io;
use std::io::{prelude::*, Stdin};

struct NextCheckPoint {
    pos: Pos,
    dist: i32,
    angle: i32,
}

struct Pos {
    x: i32,
    y: i32,
}

impl NextCheckPoint {
    fn from_slice(slice: &[i32]) -> NextCheckPoint {
        NextCheckPoint {
            pos: Pos {
                x: slice[0],
                y: slice[1],
            },
            dist: slice[2],
            angle: slice[3],
        }
    }
}

fn get_inputs(stdin: Stdin) -> Vec<i32> {
    stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|e| e.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

#[allow(unused)]
fn main() {
    // game loop
    loop {
        let inputs = get_inputs(io::stdin());

        let curr_pos = Pos {
            x: inputs[0],
            y: inputs[1],
        };

        let next_checkpoint = NextCheckPoint::from_slice(&inputs[2..=5]);

        let inputs = get_inputs(io::stdin());

        let opponent_pos = Pos {
            x: inputs[0],
            y: inputs[1],
        };

        let thrust = if next_checkpoint.angle > 90 || next_checkpoint.angle < -90 {
            "0"
        } else {
            "BOOST"
        };

        println!(
            "{x} {y} {thrust}",
            x = next_checkpoint.pos.x,
            y = next_checkpoint.pos.y,
            thrust = thrust
        );
    }
}
