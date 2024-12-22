use aoc::prelude::*;
use nom::character::complete::digit0;

fn from_keypad(ch: char) -> (i8, i8) {
    match ch {
        '0' => (1, 0),
        'A' => (2, 0),
        '1' => (0, 1),
        '2' => (1, 1),
        '3' => (2, 1),
        '4' => (0, 2),
        '5' => (1, 2),
        '6' => (2, 2),
        '7' => (0, 3),
        '8' => (1, 3),
        '9' => (2, 3),
        _ => panic!("unknown keypad button: {ch:?}"),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Atom {
    diff: (i8, i8),
    hole: (i8, i8),
    count: u8,
}

impl Atom {
    fn from_num(i: &str) -> Vec<Atom> {
        let mut into = Vec::new();
        let mut pos = (2, 0);
        for ch in i.chars().chain(['A']) {
            let next = from_keypad(ch);
            into.push(Atom {
                diff: (next.0 - pos.0, next.1 - pos.1),
                hole: (-pos.0, -pos.1),
                count: 1,
            });
            pos = next;
        }
        into
    }

    fn layer(&self) -> SmallVec<[Vec<Atom>; 2]> {
        assert!(self.count > 0);
        let mut vec = SmallVec::new();
        match self.diff {
            (x @ 1.., 0) => vec.push(vec![
                Atom {
                    diff: (0, -1),
                    hole: (-2, 0),
                    count: x as u8,
                },
                Atom {
                    diff: (0, 1),
                    hole: (-2, 1),
                    count: self.count,
                },
            ]),
            (x @ ..0, 0) => vec.push(vec![
                Atom {
                    diff: (-2, -1),
                    hole: (-2, 0),
                    count: (-x) as u8,
                },
                Atom {
                    diff: (2, 1),
                    hole: (0, 1),
                    count: self.count,
                },
            ]),
            (0, y @ 1..) => vec.push(vec![
                Atom {
                    diff: (-1, 0),
                    hole: (-2, 0),
                    count: y as u8,
                },
                Atom {
                    diff: (1, 0),
                    hole: (-1, 0),
                    count: self.count,
                },
            ]),
            (0, y @ ..0) => vec.push(vec![
                Atom {
                    diff: (-1, -1),
                    hole: (-2, 0),
                    count: (-y) as u8,
                },
                Atom {
                    diff: (1, 1),
                    hole: (-1, 1),
                    count: self.count,
                },
            ]),
            (x @ 1.., y @ 1..) => {
                if (self.hole.0 - x, self.hole.1) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (0, -1),
                            hole: (-2, 0),
                            count: x as u8,
                        },
                        Atom {
                            diff: (-1, 1),
                            hole: (-2, 1),
                            count: y as u8,
                        },
                        Atom {
                            diff: (1, 0),
                            hole: (-1, 0),
                            count: self.count,
                        },
                    ]);
                }
                if (self.hole.0, self.hole.1 - y) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (-1, 0),
                            hole: (-2, 0),
                            count: y as u8,
                        },
                        Atom {
                            diff: (1, -1),
                            hole: (-1, 0),
                            count: x as u8,
                        },
                        Atom {
                            diff: (0, 1),
                            hole: (-2, 1),
                            count: self.count,
                        },
                    ]);
                }
            }
            (x @ ..0, y @ 1..) => {
                if (self.hole.0 - x, self.hole.1) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (-2, -1),
                            hole: (-2, 0),
                            count: (-x) as u8,
                        },
                        Atom {
                            diff: (1, 1),
                            hole: (0, 1),
                            count: y as u8,
                        },
                        Atom {
                            diff: (1, 0),
                            hole: (-1, 0),
                            count: self.count,
                        },
                    ]);
                }
                if (self.hole.0, self.hole.1 - y) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (-1, 0),
                            hole: (-2, 0),
                            count: y as u8,
                        },
                        Atom {
                            diff: (-1, -1),
                            hole: (-1, 0),
                            count: (-x) as u8,
                        },
                        Atom {
                            diff: (2, 1),
                            hole: (0, 1),
                            count: self.count,
                        },
                    ]);
                }
            }
            (x @ 1.., y @ ..0) => {
                if (self.hole.0 - x, self.hole.1) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (0, -1),
                            hole: (-2, 0),
                            count: x as u8,
                        },
                        Atom {
                            diff: (-1, 0),
                            hole: (-2, 1),
                            count: (-y) as u8,
                        },
                        Atom {
                            diff: (1, 1),
                            hole: (-1, 1),
                            count: self.count,
                        },
                    ]);
                }
                if (self.hole.0, self.hole.1 - y) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (-1, -1),
                            hole: (-2, 0),
                            count: (-y) as u8,
                        },
                        Atom {
                            diff: (1, 0),
                            hole: (-1, 1),
                            count: x as u8,
                        },
                        Atom {
                            diff: (0, 1),
                            hole: (-2, 1),
                            count: self.count,
                        },
                    ]);
                }
            }
            (x @ ..0, y @ ..0) => {
                if (self.hole.0 - x, self.hole.1) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (-2, -1),
                            hole: (-2, 0),
                            count: (-x) as u8,
                        },
                        Atom {
                            diff: (1, 0),
                            hole: (0, 1),
                            count: (-y) as u8,
                        },
                        Atom {
                            diff: (1, 1),
                            hole: (-1, 1),
                            count: self.count,
                        },
                    ]);
                }
                if (self.hole.0, self.hole.1 - y) != (0, 0) {
                    vec.push(vec![
                        Atom {
                            diff: (-1, -1),
                            hole: (-2, 0),
                            count: (-y) as u8,
                        },
                        Atom {
                            diff: (-1, 0),
                            hole: (-1, 1),
                            count: (-x) as u8,
                        },
                        Atom {
                            diff: (2, 1),
                            hole: (0, 1),
                            count: self.count,
                        },
                    ]);
                }
            }
            (0, 0) => vec.push(vec![Atom {
                diff: (0, 0),
                hole: (-2, 0),
                count: self.count,
            }]),
        }
        vec
    }

    fn cost(&self) -> usize {
        self.diff.0.abs() as usize + self.diff.1.abs() as usize + self.count as usize
    }

    fn cheapest(&self, n: usize) -> usize {
        if n == 0 {
            self.cost()
        } else {
            let possible = self.layer();
            possible
                .into_iter()
                .map(|l| l.into_iter().map(|a| a.cheapest(n - 1)).sum())
                .min()
                .unwrap()
        }
    }

    fn cheapest_seq(&self) -> Vec<Atom> {
        let possible = self.layer();
        let mut best = 0;
        let mut dup = true;
        let mut level = 0;
        while dup {
            level += 1;
            let mut cost = usize::MAX;
            for (idx, map) in possible.iter().enumerate() {
                let c: usize = map.iter().map(|a| a.cheapest(level)).sum();
                if c < cost {
                    cost = c;
                    best = idx;
                    dup = false;
                } else if c == cost {
                    dup = true;
                }
            }
        }
        possible.into_iter().nth(best).unwrap()
    }
}

fn cheapest(atoms: &[Atom], n: usize) -> usize {
    let mut cache = HashMap::new();
    let mut counts = HashMap::<Atom, usize>::new();
    let mut next = HashMap::<Atom, usize>::new();
    for atom in atoms {
        *counts.entry(*atom).or_default() += 1;
    }
    for _ in 0..n {
        next.clear();
        for (atom, count) in counts.iter() {
            for new in cache.entry(*atom).or_insert_with(|| atom.cheapest_seq()) {
                *next.entry(*new).or_default() += *count;
            }
        }
        std::mem::swap(&mut counts, &mut next);
    }
    let mut cost = 0;
    for (atom, count) in counts {
        cost += atom.cost() * count;
    }
    cost
}

#[main]
fn day21(inp: &'static str) {
    let (_, inputs) = nom_err(terminated(
        many1(terminated(terminated(digit0, nom_char('A')), line_ending)),
        eof,
    )(inp))
    .unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for input in inputs {
        let int: usize = input.parse().unwrap();
        let atoms = Atom::from_num(input);
        part1 += int * cheapest(&atoms, 2);
        part2 += int * cheapest(&atoms, 25);
    }
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
