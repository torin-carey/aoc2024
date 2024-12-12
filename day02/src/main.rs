use aoc::prelude::*;

fn parse_row(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(
        space1,
        nom_u32
    )(i)
}

fn is_safe_mono(row: &[u32], inc: bool) -> bool {
    for idx in 0..row.len()-1 {
        if inc && row[idx+1] > row[idx] && row[idx+1] <= row[idx]+3 {
            continue
        }
        if !inc && row[idx+1] < row[idx] && row[idx+1]+3 >= row[idx] {
            continue
        }
        return false
    }
    return true
}

fn is_safe(row: &[u32]) -> bool {
    is_safe_mono(row, true) || is_safe_mono(row, false)
}

fn is_safe_mono_skip(row: &[u32], inc: bool, skip: usize) -> bool {
    let skip_fn = |idx| if idx >= skip { idx+1 } else { idx };
    for idx in 0..row.len()-2 {
        let (i1, i2) = (skip_fn(idx), skip_fn(idx+1));
        if inc && row[i2] > row[i1] && row[i2] <= row[i1]+3 {
            continue
        }
        if !inc && row[i2] < row[i1] && row[i2]+3 >= row[i1] {
            continue
        }
        return false
    }
    return true
}

fn is_safe_skip(row: &[u32], skip: usize) -> bool {
    is_safe_mono_skip(row, true, skip) || is_safe_mono_skip(row, false, skip)
}

#[main]
fn day2(inp: &str) -> Result<()> {
    let (i, rows) = many1(terminated(parse_row, line_ending))(inp).unwrap();

    let mut safe = 0usize;
    for mut row in rows {
        if (0..row.len()).map(|i| is_safe_skip(&row, i)).any(|b| b) {
            safe += 1;
        }
    }

    dbg!(safe);

    Ok(())
}
