use aoc::prelude::*;

fn parse_calib(i: &str) -> IResult<&str, (u64, Vec<u64>)> {
    pair(
        terminated(nom_u64, nom_char(':')),
        many1(preceded(nom_char(' '), nom_u64)),
    )(i)
}

fn concatenate(a: u64, b: u64) -> u64 {
    let fact = match b {
        ..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        1000..10000 => 10000,
        10000..100000 => 100000,
        _ => panic!("concat table too small")
    };
    a*fact + b
}

fn possible(target: u64, operands: &[u64], concat: bool) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((operands[0], &operands[1..]));
    while let Some((cur, rem)) = queue.pop_front() {
        let [next, rest @ ..] = rem else {
            if cur == target {
                return true
            } else {
                continue
            }
        };
        if cur + *next <= target {
            queue.push_back((cur + *next, rest));
        }
        if cur * *next <= target {
            queue.push_back((cur * *next, rest));
        }
        if concat {
            assert!(cur != 0 && *next != 0,
                "concatenation is undefined for zero operands");
            let c = concatenate(cur, *next);
            if c <= target {
                queue.push_back((c, rest));
            }
        }
    }
    false
}

#[main]
fn day7(inp: &'static str) -> Result<()> {
    let mut iter = iterator(inp, terminated(parse_calib, line_ending));
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for (target, operands) in &mut iter {
        if possible(target, &operands, false) {
            part1 += target;
            part2 += target;
        } else if possible(target, &operands, true) {
            part2 += target;
        }
    }
    let (i, _) = iter.finish()?;
    nom_err(eof(i))?;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
