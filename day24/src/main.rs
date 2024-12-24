use aoc::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Gate::And, tag(" AND ")),
            value(Gate::Or, tag(" OR ")),
            value(Gate::Xor, tag(" XOR ")),
        ))(i)
    }

    fn eval(&self, a: bool, b: bool) -> bool {
        match *self {
            Gate::And => a && b,
            Gate::Or => a || b,
            Gate::Xor => a != b,
        }
    }
}

const N: usize = 45;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Op<'a>([&'a str; 2], Gate);

impl<'a> Op<'a> {
    fn new(mut operands: [&'a str; 2], gate: Gate) -> Self {
        operands.sort();
        Op(operands, gate)
    }
}

struct Code<'a> {
    canonical: HashMap<Op<'a>, &'a str>,
    gates: HashMap<&'a str, Op<'a>>,
    names: HashSet<&'a str>,
    swaps: Vec<[&'a str; 2]>,
}

impl<'a> Code<'a> {
    fn swap(&mut self, a: &'a str, b: &'a str) {
        let a_op = self.gates[a];
        let b_op = self.gates[b];
        self.canonical.insert(a_op, b);
        self.canonical.insert(b_op, a);
        self.gates.insert(b, a_op);
        self.gates.insert(a, b_op);
        self.swaps.push([a, b]);
    }

    fn get_register(&self, ch: char, idx: usize) -> Option<&'a str> {
        let s = format!("{ch}{idx:02}");
        self.names.get(s.as_str()).copied()
    }

    fn find_and_swap(&mut self, a: &'a str, b: &'a str, gate: Gate) -> &'a str {
        if let Some(name) = self.canonical.get(&Op::new([a, b], gate)).copied() {
            name
        } else {
            let mut to_swap = None;
            for (name, t) in &self.gates {
                let (x, y, g) = (t.0[0], t.0[1], t.1);
                if gate != g {
                    continue;
                }
                if a == x {
                    to_swap = Some((*name, b, y));
                } else if a == y {
                    to_swap = Some((*name, b, x));
                } else if b == x {
                    to_swap = Some((*name, a, y));
                } else if b == y {
                    to_swap = Some((*name, a, x));
                } else {
                    continue;
                }
                break;
            }
            let (name, x, y) = to_swap.expect("no candidate");
            self.swap(x, y);
            name
        }
    }

    fn find_relabels(&mut self) {
        let mut carry: Option<&'a str> = None;
        for i in 0..N {
            let Some(xi) = self.get_register('x', i) else { break };
            let Some(yi) = self.get_register('y', i) else { break };

            let ei = self.canonical[&Op::new([xi, yi], Gate::Xor)];
            let ai = self.canonical[&Op::new([xi, yi], Gate::And)];

            if let Some(c) = carry {
                let eic = self.find_and_swap(ei, c, Gate::And);
                carry = Some(self.find_and_swap(eic, ai, Gate::Or));
                Op::new([ei, c], Gate::Xor);
            } else {
                assert!(i == 0);
                carry = Some(ai);
            }
        }
        assert_eq!(carry, Some("z45"));
    }
}

impl<'a> From<&[(&'a str, &'a str, Gate, &'a str)]> for Code<'a> {
    fn from(v: &[(&'a str, &'a str, Gate, &'a str)]) -> Self {
        let mut code = Code {
            canonical: HashMap::with_capacity(v.len()),
            gates: HashMap::with_capacity(v.len()),
            names: HashSet::new(),
            swaps: Vec::new(),
        };
        for (a, b, g, o) in v.into_iter().copied() {
            let op = Op::new([a, b], g);
            code.canonical.insert(op, o);
            code.gates.insert(o, op);
            code.names.insert(o);
            code.names.insert(a);
            code.names.insert(b);
        }
        code
    }
}

#[derive(Debug)]
struct Inputs<'a> {
    initial: HashMap<&'a str, bool>,
    gates: Vec<(&'a str, &'a str, Gate, &'a str)>,
}

impl<'a> Inputs<'a> {
    fn parse(i: &'a str) -> IResult<&'a str, Self> {
        let (i, initial) = many1(
            terminated(
                separated_pair(alphanumeric1, tag(": "), alt((
                    value(true, nom_char('1')),
                    value(false, nom_char('0')),
                ))),
                line_ending
            )
        )(i)?;
        let (i, _) = line_ending(i)?;
        let (i, gates) = many1(
            map(
                tuple((
                    alphanumeric1,
                    Gate::parse,
                    alphanumeric1,
                    tag(" -> "),
                    alphanumeric1,
                    line_ending,
                )),
                |(a, g, b, _, o, _)| (a, b, g, o)
            )
        )(i)?;
        Ok((i, Self { initial: initial.into_iter().collect(), gates }))
    }
}

fn eval<'a>(inputs: &Inputs<'a>) -> HashMap<&'a str, bool> {
    let mut state = inputs.initial.clone();
    let mut gates: HashMap<_, _> = inputs.gates.iter()
        .copied()
        .map(|(a, b, g, o)| (o, (a, b, g)))
        .collect();

    while !gates.is_empty() {
        let mut to_remove = Vec::new();
        for (out, (a, b, gate)) in &gates {
            let Some(a_val) = state.get(a) else { continue };
            let Some(b_val) = state.get(b) else { continue };
            state.insert(*out, gate.eval(*a_val, *b_val));
            to_remove.push(*out);
        }
        for out in to_remove {
            gates.remove(out);
        }
    }

    state
}

fn register(state: &HashMap<&str, bool>, reg: char) -> u64 {
    let mut res = 0;
    for (out, set) in state {
        let Ok((_, shift)) = nom_err(delimited(nom_char(reg), nom_u64, eof)(*out)) else { continue };
        if *set {
            res |= 1 << shift;
        }
    }
    res
}

#[main]
fn day24(i: &'static str) -> Result<()> {
    let (_, inputs) = terminated(Inputs::parse, eof)(i)?;
    let evaluated = eval(&inputs);

    println!("Part 1: {}", register(&evaluated, 'z'));

    let mut code: Code = inputs.gates.as_slice().into();

    code.find_relabels();
    let mut swaps: Vec<_> = code.swaps.iter().copied().flatten().collect();
    swaps.sort();
    swaps.dedup();
    println!("Part 2: {}", swaps.join(","));

    Ok(())
}
