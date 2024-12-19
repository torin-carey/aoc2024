use aoc::prelude::*;

fn count_tokens(input: &str, patterns: &[&str]) -> usize {
    let mut counts = vec![0usize; input.len() + 1];
    counts[0] = 1;
    for idx in 0..input.len() {
        if counts[idx] == 0 {
            continue;
        }
        for pattern in patterns {
            if input[idx..].starts_with(*pattern) {
                counts[idx + pattern.len()] += counts[idx];
            }
        }
    }
    counts[input.len()]
}

#[main]
fn day18(inp: &'static str) {
    let (i, patterns) = nom_err(terminated(
        separated_list1(tag(", "), alpha1),
        pair(line_ending, line_ending),
    )(inp))
    .unwrap();
    let mut inputs = iterator(i, terminated(alpha1, line_ending));
    let mut part1 = 0;
    let mut part2 = 0;
    for input in &mut inputs {
        let count = count_tokens(input, &patterns);
        if count > 0 {
            part1 += 1;
            part2 += count;
        }
    }
    assert!(nom_err(inputs.finish()).unwrap().0.is_empty());
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
