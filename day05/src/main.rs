use aoc::prelude::*;

fn parse_ordering(i: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many1(terminated(
        separated_pair(nom_u32, nom_char('|'), nom_u32),
        line_ending,
    ))(i)
}

fn in_order(row: &[u32], ords: &[(u32, u32)]) -> bool {
    let idx: HashMap<_, _> = row.into_iter().enumerate().map(|(i, k)| (k, i)).collect();
    ords.into_iter().all(|(a, b)| {
        if let (Some(ia), Some(ib)) = (idx.get(a), idx.get(b)) {
            *ia < *ib
        } else {
            true
        }
    })
}

fn order(row: &mut [u32], ords: &[(u32, u32)]) {
    let set: HashSet<_> = row.into_iter().collect();
    let ords: Vec<(u32, u32)> = ords
        .into_iter()
        .filter(|(a, b)| set.contains(a) && set.contains(b))
        .copied()
        .collect();

    let find = |row: &[u32], val| {
        row.into_iter()
            .enumerate()
            .filter(|(_, x)| **x == val)
            .next()
            .unwrap()
            .0
    };

    loop {
        let mut did_swap = false;
        for &(a, b) in &ords {
            let (i1, i2) = (find(&row, a), find(&row, b));
            if i1 > i2 {
                row.swap(i1, i2);
                did_swap = true;
            }
        }
        if !did_swap {
            break;
        }
    }
}

#[main]
fn day5(inp: &'static str) -> Result<()> {
    let (i, ord) = terminated(parse_ordering, line_ending)(inp)?;

    let mut rows = iterator(
        i,
        terminated(
            separated_list1(nom_char::<_, nom::error::Error<_>>(','), nom_u32),
            line_ending,
        ),
    );

    let (mut part1, mut part2) = (0, 0);
    for mut row in &mut rows {
        if in_order(&row, &ord) {
            part1 += row[row.len() / 2];
        } else {
            order(&mut row, &ord);
            part2 += row[row.len() / 2];
        }
    }
    dbg!(part1, part2);

    Ok(())
}
