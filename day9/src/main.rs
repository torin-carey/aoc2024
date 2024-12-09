use aoc::prelude::*;

use std::iter::repeat;

fn char_num(ch: char) -> Option<usize> {
    match ch {
        '0'..='9' => Some(ch as usize - '0' as usize),
        '\n' => None,
        _ => panic!("unhandled char {ch}")
    }
}

fn part1(inp: &str) -> usize {
    let mut disk: Vec<Option<u16>> = Vec::new();
    let mut id = 0u16;
    let mut file = true;
    for ch in inp.chars() {
        let Some(len) = char_num(ch) else { break };
        disk.extend(repeat(if file {
            id += 1;
            Some(id-1)
        } else {
            None
        }).take(len));
        file = !file;
    }

    let iter1 = disk.iter().copied().enumerate();
    let mut iter2 = iter1.clone().rev().flat_map(|(i, v)| v.map(|x| (i, x)));
    let mut checksum = 0;
    let mut last_idx = disk.len();
    for (idx, val) in iter1 {
        if idx >= last_idx {
            break
        }
        let id = match val {
            Some(v) => v,
            None => {
                let (idx2, val2) = iter2.next().expect("no more back elements");
                last_idx = idx2;
                if idx2 < idx {
                    break
                }
                val2
            }
        };

        checksum += idx * id as usize;
    }

    checksum
}

fn part2(inp: &str) -> usize {
    //                    pos    len
    let mut files:  Vec<(usize, usize)> = Vec::new();
    let mut spaces: Vec<(usize, usize)> = Vec::new();

    let mut pos = 0;
    let mut file = true;
    for ch in inp.chars() {
        let Some(len) = char_num(ch) else { break };
        if file {
            files.push((pos, len));
        } else {
            spaces.push((pos, len));
        }
        pos += len;
        file = !file;
    }

'file_loop:
    for (idx, file) in files.iter_mut().enumerate().rev() {
        for (space_idx, space) in spaces.iter_mut().enumerate() {
            if idx > space_idx && space.1 >= file.1 {
                // We should also merge the spaces around files original
                // position, but that's not necessary for the correct
                // solution.
                file.0 = space.0;
                space.1 -= file.1;
                space.0 += file.1;
                continue 'file_loop
            }
        }
    }

    let mut checksum = 0;
    for (idx, file) in files.iter().copied().enumerate() {
        for i in 0..file.1 {
            checksum += (file.0 + i) * idx;
        }
    }
    checksum
}

#[main]
fn day9(inp: &'static str) -> Result<()> {
    dbg!(part1(inp));
    dbg!(part2(inp));
    Ok(())
}
