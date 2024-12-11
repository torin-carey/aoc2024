use aoc::prelude::*;

fn even_split(i: u64) -> Option<(u64, u64)> {
    let format = format!("{i}");
    if format.len() % 2 == 0 {
        let len = format.len() / 2;
        Some((format[..len].parse().unwrap(), format[len..].parse().unwrap()))
    } else {
        None
    }
}

fn blink(rocks: &mut HashMap<u64, usize>, count: usize) -> usize {
    let mut tmp = HashMap::new();
    for _ in 0..count {
        tmp.clear();
        for (rock, c) in &*rocks {
            if *rock == 0 {
                *tmp.entry(1).or_default() += *c;
            } else if let Some((a, b)) = even_split(*rock) {
                *tmp.entry(a).or_default() += *c;
                *tmp.entry(b).or_default() += *c;
            } else {
                *tmp.entry(rock*2024).or_default() += *c;
            }
        }
        std::mem::swap(rocks, &mut tmp);
    }
    rocks.iter().fold(0, |s, (_, c)| s + c)
}

#[main]
fn day11(inp: &'static str) -> Result<()> {
    let (_, rocks) = nom_err(separated_list1(tag(" "), nom_u64)(inp))?;
    let mut rocks: HashMap<u64, usize> = rocks.into_iter().map(|x| (x, 1)).collect();

    let part1 = blink(&mut rocks, 25);
    let part2 = blink(&mut rocks, 50);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
