use std::io::prelude::*;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

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

#[allow(unused)]
fn main() {
    // game loop
    loop {
        let mut input_line = io::stdin().lock().lines().next().unwrap().unwrap();
        let inputs = input_line
            .split(" ")
            .map(|e| parse_input!(e, i32))
            .collect::<Vec<_>>();

        let curr_pos = Pos {
            x: inputs[0],
            y: inputs[1],
        };
        let next_checkpoint = NextCheckPoint::from_slice(&inputs[2..=5]);

        let mut input_line = io::stdin().lock().lines().next().unwrap().unwrap();
        let inputs = input_line
            .split(" ")
            .map(|e| parse_input!(e, i32))
            .collect::<Vec<_>>();

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
