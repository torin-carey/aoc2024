use aoc::prelude::*;

#[derive(Copy, Clone, Debug, ParseTile, DisplayTile, PartialEq, Eq)]
pub enum Tile {
    #[tile('#')]
    Wall,
    #[tile('O')]
    Box,
    #[tile('.')]
    Space,
    #[tile('@')]
    Start,
}

fn xray_box_space(map: &Map<Tile>, mut coord: Coords, dir: Dir) -> Option<Coords> {
    while map[coord] == Tile::Box {
        let Some(next) = map.add(coord, dir) else { None? };
        coord = next;
    }
    if map[coord] == Tile::Space {
        Some(coord)
    } else {
        None
    }
}

pub fn part1(mut map: Map<Tile>, moves: &[Dir]) -> Result<usize> {
    let Some(mut pos) = map.iter()
        .filter(|(c, t)| **t == Tile::Start)
        .map(|(c, _)| c)
        .next() else
    {
        Err(anyhow!("no start point"))?
    };
    println!("{map}\n");
    map[pos] = Tile::Space;

    for m in moves.iter().copied() {
        let Some(next) = map.add(pos, m) else { continue };
        match map[next] {
            Tile::Space => pos = next,
            Tile::Wall => {},
            Tile::Box => {
                if let Some(end) = xray_box_space(&map, next, m) {
                    map[end] = Tile::Box;
                    map[next] = Tile::Space;
                    pos = next;
                }
            },
            Tile::Start => Err(anyhow!("multiple start positions"))?,
        }
    }
    map[pos] = Tile::Start;

    println!("{map}");

    let mut part1 = 0;
    for (coord, tile) in map.iter() {
        if *tile == Tile::Box {
            part1 += coord.0 + (100*coord.1);
        }
    }

    Ok(part1)
}
