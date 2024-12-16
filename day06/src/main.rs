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

fn part1(mut pos: Coords, map: &mut Map<Tile>) -> Result<usize> {
    let mut dir = Dir::N;

    loop {
        map[pos] = Tile::Hit;
        let Some(to) = map.add(pos, dir) else { break };
        if map[to] == Tile::Wall {
            dir = dir + Dir::E;
        } else {
            pos = to;
        }
    }

    Ok(map.iter().filter(|(_, t)| **t == Tile::Hit).count())
}

fn does_loop(mut pos: Coords, map: &Map<Tile>, set: &mut HashSet<(Coords, Dir)>) -> bool {
    set.clear();
    let mut dir = Dir::N;
    while set.insert((pos, dir)) {
        let Some(to) = map.add(pos, dir) else {
            return false;
        };
        if map[to] == Tile::Wall {
            dir = dir + Dir::E;
        } else {
            pos = to;
        }
    }
    true
}

fn part2(start: Coords, map: &mut Map<Tile>) -> Result<usize> {
    let mut set = HashSet::new();
    let mut count = 0;
    map[start] = Tile::Start;
    for idx in 0..map.width() * map.height() {
        let coord = map.coords(idx);
        if map[coord] != Tile::Hit {
            continue;
        }
        map[coord] = Tile::Wall;
        if does_loop(start, &map, &mut set) {
            count += 1;
        }
        map[coord] = Tile::Hit;
    }
    Ok(count)
}

#[main]
fn day6(inp: &'static str) -> Result<()> {
    let (_, mut map) = nom_err(Map::<Tile>::parse(inp))?;
    let (start, _) = map
        .iter()
        .filter(|(_, t)| **t == Tile::Start)
        .next()
        .unwrap();

    dbg!(part1(start, &mut map)?);
    dbg!(part2(start, &mut map)?);

    Ok(())
}
