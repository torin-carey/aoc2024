use aoc::prelude::*;

#[main]
fn day1(inp: &str) -> Result<()> {
    let parse_line = |i| -> IResult<&str, (u32, u32)> {
        terminated(separated_pair(nom_u32, space1, nom_u32), line_ending)(i)
    };

    let mut iter = iterator(inp, parse_line);

    let mut list = Vec::new();
    let mut count: HashMap<u32, u32> = HashMap::new();

    for (i1, i2) in &mut iter {
        list.push(i1);
        let set = count.entry(i2).or_default();
        *set = *set + 1;
    }

    let mut res = 0u32;
    for i in list {
        let cnt = *count.get(&i).unwrap_or(&0);
        res += i * cnt;
    }

    dbg!(res);

    Ok(())
}
