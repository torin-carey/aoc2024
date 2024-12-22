use aoc::prelude::*;

const MOD: u64 = 16777216;

fn function(secret: u64) -> u64 {
    let secret = ((secret * 64) ^ secret) % MOD;
    let secret = ((secret / 32) ^ secret) % MOD;
    ((secret * 2048) ^ secret) % MOD
}

#[derive(Clone, Debug)]
struct Secrets(u64);

impl Iterator for Secrets {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.0 = function(self.0);
        Some(self.0)
    }
}

struct Windows(i8, i8, i8, i8, u64);

impl Windows {
    fn new(start: u64) -> Self {
        let a = function(start);
        let b = function(a);
        let c = function(b);
        Self(
            0,
            (a % 10) as i8 - (start % 10) as i8,
            (b % 10) as i8 - (a % 10) as i8,
            (c % 10) as i8 - (b % 10) as i8,
            c,
        )
    }
}

impl Iterator for Windows {
    type Item = ([i8; 4], usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.1;
        self.1 = self.2;
        self.2 = self.3;
        let x = function(self.4);
        self.3 = (x % 10) as i8 - (self.4 % 10) as i8;
        self.4 = x;
        Some(([self.0, self.1, self.2, self.3], (self.4 % 10) as usize))
    }
}

#[main]
fn day22(inp: &'static str) {
    let (_, secrets) = nom_err(terminated(many1(terminated(nom_u64, line_ending)), eof)(
        inp,
    ))
    .unwrap();
    let mut part1 = 0;
    let mut part2_map = HashMap::<[i8; 4], usize>::new();
    for secret in secrets {
        part1 += Secrets(secret).nth(2000).unwrap();

        let mut seen = HashSet::new();
        for (seq, price) in Windows::new(secret).take(2000 - 4) {
            if seen.insert(seq) {
                *part2_map.entry(seq).or_default() += price;
            }
        }
    }
    let part2 = part2_map.values().max().copied().unwrap();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
