use aoc::prelude::*;

#[derive(Copy, Clone, Debug, ParseTile, PartialEq, Eq)]
enum Tile {
    #[tile('X')]
    X,
    #[tile('M')]
    M,
    #[tile('A')]
    A,
    #[tile('S')]
    S,
}

fn part1(map: &Map<Tile>) -> Result<usize> {
    let mut count = 0;
    for (cx, tile) in map {
        if tile != &Tile::X {
            continue
        }

        for (cm, dir) in map.neigh_diag(cx) {
            if map[cm] != Tile::M {
                continue
            }
            let Some(ca) = map.add_diag(cm, dir) else {
                continue
            };
            let Some(cs) = map.add_diag(ca, dir) else {
                continue
            };
            if map[ca] == Tile::A && map[cs] == Tile::S {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn part2(map: &Map<Tile>) -> Result<usize> {
    let mut count = 0;
    for (coord, tile) in map {
        if tile != &Tile::A {
            continue
        }
        let neigh: SmallVec<[_; 4]> = map.neigh_diag_only(coord)
                .into_iter().map(|(c, _)| map[c]).collect();
        if neigh.len() != 4 {
            continue
        }
        let xmas = match (neigh[0], neigh[1], neigh[2], neigh[3]) {
            (Tile::M, Tile::M, Tile::S, Tile::S) => true,
            (Tile::M, Tile::S, Tile::M, Tile::S) => true,
            (Tile::S, Tile::S, Tile::M, Tile::M) => true,
            (Tile::S, Tile::M, Tile::S, Tile::M) => true,
            _ => false
        };
        if xmas {
            count += 1;
        }
    }
    Ok(count)
}

#[main]
fn day4(inp: &'static str) -> Result<()> {
    let (_, map): (_, Map<Tile>) = Map::parse::<_, nom::error::Error<_>>(inp)?;

    dbg!(part1(&map)?);
    dbg!(part2(&map)?);

    Ok(())
}
