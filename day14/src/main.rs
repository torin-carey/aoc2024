use aoc::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

const WIDTH: i32 = 101;
const WCOEFF: i32 = 51;
const HEIGHT: i32 = 103;
const HCOEFF: i32 = -50;

impl Robot {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (_, px, _, py, _, vx, _, vy, _)) = tuple((
            tag("p="),
            nom_i32,
            tag(","),
            nom_i32,
            tag(" v="),
            nom_i32,
            tag(","),
            nom_i32,
            line_ending,
        ))(i)?;
        Ok((
            i,
            Robot {
                pos: (px, py),
                vel: (vx, vy),
            },
        ))
    }

    fn advance(self, count: i32) -> Robot {
        let x = (self.pos.0 + (self.vel.0 * (count % WIDTH))) % WIDTH;
        let y = (self.pos.1 + (self.vel.1 * (count % HEIGHT))) % HEIGHT;
        Robot {
            pos: ((x + WIDTH) % WIDTH, (y + HEIGHT) % HEIGHT),
            vel: self.vel,
        }
    }

    fn quad(self) -> Option<usize> {
        const HW: i32 = WIDTH / 2;
        const HWP1: i32 = HW + 1;
        const HH: i32 = HEIGHT / 2;
        const HHP1: i32 = HH + 1;
        Some(match self.pos {
            (0..HW, 0..HH) => 0,
            (HWP1..WIDTH, 0..HH) => 1,
            (0..HW, HHP1..HEIGHT) => 2,
            (HWP1..WIDTH, HHP1..HEIGHT) => 3,
            (HW, ..) | (.., HH) | _ => None?,
        })
    }
}

trait IteratorF64Ext: Iterator<Item = f64> + Clone {
    fn mean(self) -> Option<f64> {
        let mut sum = 0.0;
        let mut count = 0;
        for f in self {
            sum += f;
            count += 1;
        }
        if count != 0 {
            Some(sum / count as f64)
        } else {
            None
        }
    }

    fn variance(self) -> Option<f64> {
        let mean = self.clone().mean()?;
        self.map(|f| (f - mean).powf(2.0)).mean()
    }
}

impl<I: Iterator<Item = f64> + Clone> IteratorF64Ext for I {}

fn solve(x: i32, y: i32) -> i32 {
    let m = WIDTH * HEIGHT;
    (((y * WIDTH * WCOEFF + x * HEIGHT * HCOEFF) % m) + m) % m
}

#[main]
fn day14(inp: &'static str) {
    let (_, robots) = many1(Robot::parse)(inp).unwrap();

    let part1: usize = {
        let mut counts = [0; 4];
        for idx in robots.iter().flat_map(|r| r.advance(100).quad()) {
            counts[idx] += 1;
        }
        counts.into_iter().sum()
    };

    let points_at = |t: i32| robots.iter().map(move |r| r.advance(t).pos);
    let (_, x_time) = (0..WIDTH)
        .map(|t| (points_at(t).map(|(x, _)| x as f64).variance().unwrap(), t))
        .reduce(|(s1, t1), (s2, t2)| if s1 < s2 { (s1, t1) } else { (s2, t2) })
        .unwrap();
    let (_, y_time) = (0..HEIGHT)
        .map(|t| (points_at(t).map(|(_, y)| y as f64).variance().unwrap(), t))
        .reduce(|(s1, t1), (s2, t2)| if s1 < s2 { (s1, t1) } else { (s2, t2) })
        .unwrap();

    let part2 = solve(x_time, y_time);

    let mut map = Map::<char>::new(WIDTH as usize, HEIGHT as usize, '.');
    for robot in robots.iter().map(|r| r.advance(part2)) {
        map[(robot.pos.0 as usize, robot.pos.1 as usize)] = '#';
    }
    println!("{map}");

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
