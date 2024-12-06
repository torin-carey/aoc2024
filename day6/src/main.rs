use aoc::prelude::*;

#[derive(Copy, Clone, Debug, DisplayTile, ParseTile, PartialEq, Eq)]
enum Tile {
    #[tile('.')]
    Empty,
    #[tile('#')]
    Wall,
    #[tile('^')]
    Start,
    #[tile('X')]
    Hit,
}

fn part1(mut pos: Coords, mut map: Map<Tile>) -> Result<usize> {
    let mut dir = Dir::U;

    loop {
        map[pos] = Tile::Hit;
        let Some(to) = map.add(pos, dir) else { break };
        if map[to] == Tile::Wall {
            dir = dir + Dir::R;
        } else {
            pos = to;
        }
    }

    Ok(map.iter().filter(|(_, t)| **t == Tile::Hit).count())
}

fn does_loop(mut pos: Coords, map: &Map<Tile>, set: &mut HashSet<(Coords, Dir)>) -> bool {
    set.clear();
    let mut dir = Dir::U;
    while set.insert((pos, dir)) {
        let Some(to) = map.add(pos, dir) else { return false };
        if map[to] == Tile::Wall {
            dir = dir + Dir::R;
        } else {
            pos = to;
        }
    }
    true
}

fn part2(start: Coords, mut map: Map<Tile>) -> Result<usize> {
    let mut set = HashSet::new();
    let mut count = 0;
    for idx in 0..map.width()*map.height() {
        let coord = map.coords(idx);
        if map[coord] != Tile::Empty {
            continue
        }
        map[coord] = Tile::Wall;
        if does_loop(start, &map, &mut set) {
            count += 1;
        }
        map[coord] = Tile::Empty;
    }
    Ok(count)
}

#[main]
fn day6(inp: &'static str) -> Result<()> {
    let (_, map) = Map::<Tile>::parse::<_, nom::error::Error<_>>(inp)?;
    let (start, _) = map.iter().filter(|(_, t)| **t == Tile::Start).next().unwrap();

    dbg!(part1(start, map.clone()));
    dbg!(part2(start, map.clone()));

    Ok(())
}
