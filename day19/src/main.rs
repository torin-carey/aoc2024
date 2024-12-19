use aoc::prelude::*;

fn count_tokens(input: &str, patterns: &[&str]) -> usize {
    let mut map = BTreeMap::<usize, usize>::new();
    map.insert(0, 1);
    while let Some((len, count)) = map.pop_first() {
        if len == input.len() {
            return count;
        }
        let rem = &input[len..];
        for pattern in patterns {
            if rem.starts_with(*pattern) {
                *(map.entry(len + pattern.len()).or_default()) += count;
            }
        }
    }
    0
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
