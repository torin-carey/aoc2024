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
    #[tile('O')]
    Short,
}

type Point = (Coords, Dir);

fn coords_diff(point: Point, end: Coords) -> (usize, usize, Dir) {
    if point.0 .0 >= end.0 && point.0 .1 >= end.1 {
        (point.0 .0 - end.0, point.0 .1 - end.1, point.1)
    } else if point.0 .0 >= end.0 && point.0 .1 < end.1 {
        (point.0 .0 - end.0, end.1 - point.0 .1, point.1 + Dir::E)
    } else if point.0 .0 < end.0 && point.0 .1 < end.1 {
        (end.0 - point.0 .0, end.1 - point.0 .1, point.1 + Dir::S)
    } else {
        (end.0 - point.0 .0, point.0 .1 - end.1, point.1 + Dir::W)
    }
}

/// Computes the cost of getting from point to end, assuming there are no
/// walls in the path.
///
/// This is a lower bound for the true cost where walls are considered.
fn heuristic(point: Point, end: Coords) -> usize {
    let (x, y, d) = coords_diff(point, end);
    if x == 0 && y == 0 {
        0
    } else if x == 0 {
        match d {
            Dir::N => y,
            Dir::E | Dir::W => 1000 + y,
            Dir::S => 2000 + y,
            _ => unreachable!(),
        }
    } else if y == 0 {
        match d {
            Dir::W => y,
            Dir::N | Dir::S => 1000 + y,
            Dir::E => 2000 + y,
            _ => unreachable!(),
        }
    } else {
        match d {
            Dir::N | Dir::W => 1000 + x + y,
            Dir::S | Dir::E => 2000 + x + y,
            _ => unreachable!(),
        }
    }
}

#[main]
fn day16(inp: &'static str) {
    let (_, mut map) = nom_err(Map::<Tile>::parse(inp)).unwrap();

    let (start, _) = map
        .iter()
        .filter(|(_, t)| **t == Tile::Start)
        .next()
        .expect("expected start tile");
    let (end, _) = map
        .iter()
        .filter(|(_, t)| **t == Tile::End)
        .next()
        .expect("expected end tile");
    map[start] = Tile::Space;
    map[end] = Tile::Space;

    println!("{map}\n");

    let astar = aoc::astar::AStar::run(
        (start, Dir::E),
        |p| heuristic(*p, end),
        |p| {
            let mut neighs = SmallVec::<[(Point, usize); 4]>::new();
            let forwards = p.1.add_coords(p.0, 1);
            if map[forwards] == Tile::Space {
                neighs.push(((forwards, p.1), 1));
            }
            neighs.push(((p.0, p.1 + Dir::E), 1000));
            neighs.push(((p.0, p.1 + Dir::S), 2000));
            neighs.push(((p.0, p.1 + Dir::W), 1000));
            neighs.into_iter()
        },
        |p| p.0 == end,
    )
    .expect("no path from start to end");

    let cost = [Dir::N, Dir::E, Dir::S, Dir::W]
        .into_iter()
        .flat_map(|dir| astar.g_map.get(&(end, dir)).copied())
        .min()
        .unwrap();

    let points: HashSet<_> = astar
        .shortest_paths_nodes(
            [Dir::N, Dir::E, Dir::S, Dir::W]
                .into_iter()
                .map(|dir| (end, dir)),
        )
        .into_iter()
        .map(|(p, _)| p)
        .collect();

    for point in &points {
        map[*point] = Tile::Short;
    }
    println!("{map}\n");

    println!("Part 1: {cost}");
    println!("Part 2: {}", points.len());
}
