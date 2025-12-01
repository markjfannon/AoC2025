use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Unable to read the file!");
    let lines = lines.split("\n");

    let rotations: Vec<Rotation> = lines
        .map(|line| Rotation::from_str(line))
        .collect();

    println!("{}", rotations.len());

    let mut lock = Lock::new();

    for rotation in rotations {
        lock.inc(rotation);
    }

    println!("First password should be {}", lock.zero_count);
}



#[derive(Debug)]
enum Direction {
    L, R
}
struct Rotation {
    direction: Direction,
    distance: i64
}

struct Lock {
    position: i64,
    zero_count: u32
}

impl Lock {
    pub fn new() -> Lock {
        Lock { position : 50, zero_count : 0}
    }

    fn inc(&mut self, rot: Rotation) {
        let rem: i64 = rot.distance % 100;

        match rot.direction {
            Direction::R => self.position += rem,
            Direction::L => self.position += -rem
        }

        self.position = self.position.rem_euclid(100);

        if self.position == 0 {
            self.zero_count += 1;
        }
    }
}

impl Rotation {
    pub fn from_str(input: &str) -> Rotation {
        let mut chars = input.chars();
        let dir = chars.next().unwrap();
        let num = chars.as_str();

        let direction = match dir {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!()
        };

        let distance: i64 = num.parse().unwrap();

        Rotation { direction, distance }
    }
}