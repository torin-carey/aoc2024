use aoc::prelude::*;
use std::fmt::Write;

#[derive(Clone, Debug)]
struct State {
    a: u64,
    b: u64,
    c: u64,

    ip: usize,
    code: Vec<u8>,
}

impl State {
    fn single_step(&mut self) -> Option<Option<u8>> {
        if self.ip >= self.code.len() {
            None?;
        }
        let (opcode, operand) = (self.code[self.ip], self.code[self.ip + 1]);
        let literal = || match operand {
            0..7 => operand as u64,
            _ => panic!("invalid operand {operand}"),
        };
        let combo = || match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand {operand}"),
        };

        self.ip += 2;
        match opcode {
            0 => self.a >>= combo(),
            1 => self.b ^= literal(),
            2 => self.b = combo() & 0x07,
            3 => {
                if self.a != 0 {
                    self.ip = literal() as usize;
                }
            }
            4 => self.b ^= self.c,
            5 => return Some(Some((combo() & 0x07) as u8)),
            6 => self.b = self.a >> combo(),
            7 => self.c = self.a >> combo(),
            _ => panic!("invalid opcode {opcode}"),
        }
        Some(None)
    }

    fn reset(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
    }

    fn run(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        while let Some(s) = self.single_step() {
            if let Some(val) = s {
                out.push(val);
            }
        }
        out
    }

    fn first_out(&mut self) -> Option<u8> {
        while let Some(s) = self.single_step() {
            if let Some(val) = s {
                return Some(val);
            }
        }
        None
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (a, b, c, code)) = tuple((
            delimited(tag("Register A: "), nom_u64, line_ending),
            delimited(tag("Register B: "), nom_u64, line_ending),
            delimited(tag("Register C: "), nom_u64, pair(line_ending, line_ending)),
            preceded(tag("Program: "), separated_list1(nom_char(','), nom_u8)),
        ))(i)?;
        Ok((
            i,
            State {
                a,
                b,
                c,
                ip: 0,
                code,
            },
        ))
    }
}

fn list_to_str(out: &[u8]) -> String {
    let mut s = String::new();
    out.iter().fold(true, |f, c| {
        if f {
            write!(&mut s, "{c}")
        } else {
            write!(&mut s, ",{c}")
        }
        .unwrap();
        false
    });
    s
}

#[main]
fn day17(inp: &'static str) {
    let (_, mut state) = State::parse(inp).unwrap();

    let part1 = state.clone().run();
    println!("Part 1: {}", list_to_str(&part1));

    let mut reduced = [0u8; 1024];
    for a in 0..1024 {
        state.reset(a);
        reduced[a as usize] = state.first_out().unwrap();
    }

    let mut strands: VecDeque<u64> = VecDeque::new();
    let mut next = VecDeque::new();
    strands.push_back(0);
    for (idx, op) in state.code.iter().rev().copied().enumerate() {
        assert!(strands.len() > 0, "failed at step {idx}");
        while let Some(strand) = strands.pop_front() {
            for x in 0..=0b111 {
                let val = (strand << 3) | (x as u64);
                if reduced[(val as usize) & 1023] == op {
                    next.push_back(val);
                }
            }
        }
        std::mem::swap(&mut next, &mut strands);
        next.clear();
    }
    let part2 = strands.pop_front().unwrap();
    state.reset(part2);
    println!("Run:      {}", list_to_str(&state.run()));
    println!("Expected: {}", list_to_str(&state.code));
    println!("Part 2: {part2}");
}
