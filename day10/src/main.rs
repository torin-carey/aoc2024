use aoc::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Tile(u8);

impl ParseTile for Tile {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '0'..='9' => Some(Tile(ch as u8 - '0' as u8)),
            _ => None,
        }
    }
}

fn reachable(map: &Map<Tile>, head: Coords) -> (usize, usize) {
    let mut queue = VecDeque::new();
    let mut part1 = HashSet::new();
    let mut part2 = 0;
    queue.push_back(head);
    while let Some(coord) = queue.pop_front() {
        for (n, _) in map.neigh(coord, true, false) {
            if map[n].0 != map[coord].0 + 1 {
                continue;
            }
            if map[n] == Tile(9) {
                part1.insert(n);
                part2 += 1;
            } else {
                queue.push_back(n);
            }
        }
    }
    (part1.len(), part2)
}

#[main]
fn day10(inp: &'static str) -> Result<()> {
    let (_, map) = nom_err(Map::<Tile>::parse(inp))?;

    let (mut part1, mut part2) = (0, 0);
    for (coord, tile) in map.iter() {
        if *tile != Tile(0) {
            continue;
        }

        let (p1, p2) = reachable(&map, coord);
        part1 += p1;
        part2 += p2;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
