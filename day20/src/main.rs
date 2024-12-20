use aoc::prelude::*;

#[derive(Copy, Clone, Debug, ParseTile, DisplayTile, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    #[tile('#')]
    Wall,
    #[tile('.')]
    Space,
    #[tile('S')]
    Start,
    #[tile('E')]
    End,
}

#[main]
fn day20(inp: &'static str) {
    let (_, map) = nom_err(terminated(Map::<Tile>::parse, eof)(inp)).unwrap();
    let (end, _) = map.iter().filter(|(_, t)| **t == Tile::End).next().unwrap();

    let astar = aoc::astar::AStar::run(
        end,
        |_| 1,
        |p| {
            map.neigh(*p, true, false)
                .into_iter()
                .filter(|(n, _)| map[*n] != Tile::Wall)
                .map(|(n, _)| (n, 1))
        },
        |_| false, // search all paths
    );

    let mut part1 = 0;
    for (pos, _) in map.iter().filter(|(_, t)| **t == Tile::Wall) {
        let neighs: SmallVec<[Coords; 4]> = map
            .neigh(pos, true, false)
            .into_iter()
            .filter(|(p, _)| map[*p] != Tile::Wall)
            .map(|(p, _)| p)
            .collect();
        if neighs.len() < 2 {
            continue;
        }
        for (a, b) in (0..(neighs.len() - 1))
            .flat_map(|i| (i..neighs.len()).map(move |j| (i, j)))
            .map(|(a, b)| (neighs[a], neighs[b]))
        {
            let Some(g_a) = astar.g_map.get(&a).copied() else {
                continue;
            };
            let Some(g_b) = astar.g_map.get(&b).copied() else {
                continue;
            };
            let g_small = g_a.min(g_b);
            let g_big = g_a.max(g_b);
            if g_big > g_small + 101 {
                part1 += 1;
            }
        }
    }
    println!("Part 1: {part1}");

    let mut part2 = 0;
    let max_cost = astar.g_map.values().copied().max().unwrap();
    let mut costs: Vec<SmallVec<[Coords; 1]>> = vec![SmallVec::new(); max_cost + 1];
    for (pos, cost) in astar.g_map.iter() {
        costs[*cost].push(*pos);
    }
    for lower in 0..max_cost {
        if costs[lower].is_empty() {
            continue;
        }
        for upper in (lower..=max_cost).rev() {
            if costs[upper].is_empty() {
                continue;
            }
            let base_save = upper - lower;
            if base_save < 100 {
                break;
            }
            for a in &costs[lower] {
                for b in &costs[upper] {
                    let dist = a.0.max(b.0) - a.0.min(b.0) + a.1.max(b.1) - a.1.min(b.1);
                    if dist > 20 {
                        continue;
                    }
                    let save = base_save - dist;
                    if save >= 100 {
                        part2 += 1;
                    }
                }
            }
        }
    }
    println!("Part 2: {part2}");
}
