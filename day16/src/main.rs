use aoc::prelude::*;

#[derive(Copy, Clone, Debug, ParseTile, DisplayTile, PartialEq, Eq)]
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

fn smallest_elem<T: Copy, F: Fn(T) -> usize>(set: &HashSet<T>, f: F) -> Option<T> {
    let mut lowest = usize::MAX;
    let mut val = None;
    for p in set {
        let m = f(*p);
        if m < lowest {
            lowest = m;
            val = Some(*p);
        }
    }
    val
}

fn build_path(end: Coords, from: HashMap<Point, SmallVec<[Point; 1]>>) -> HashSet<Coords> {
    let mut set = HashSet::new();
    let mut check = Vec::new();
    check.push((end, Dir::N));
    check.push((end, Dir::E));
    check.push((end, Dir::S));
    check.push((end, Dir::W));
    while let Some(point) = check.pop() {
        set.insert(point.0);
        if let Some(froms) = from.get(&point) {
            for f in froms {
                check.push(*f);
            }
        }
    }
    set
}

fn shortest_paths(map: &Map<Tile>, start: Point, end: Coords) -> Option<(HashSet<Coords>, usize)> {
    let mut open = HashSet::<Point>::new();
    let mut gmap = HashMap::<Point, usize>::new();
    let mut fmap = HashMap::<Point, usize>::new();
    let mut camefrom = HashMap::<Point, SmallVec<[Point; 1]>>::new();
    let h = |p: Point| heuristic(p, end);
    open.insert(start);
    gmap.insert(start, 0);
    fmap.insert(start, h(start));

    while let Some(p) = smallest_elem(&open, |p| fmap[&p]) {
        if p.0 == end {
            return Some((build_path(end, camefrom), gmap[&p]));
        }
        open.remove(&p);
        let gp = gmap[&p];
        let mut neighs = SmallVec::<[(Point, usize); 4]>::new();
        let forwards = p.1.add_coords(p.0, 1);
        if map[forwards] == Tile::Space {
            neighs.push(((forwards, p.1), gp + 1));
        }
        neighs.push(((p.0, p.1 + Dir::E), gp + 1000));
        neighs.push(((p.0, p.1 + Dir::S), gp + 2000));
        neighs.push(((p.0, p.1 + Dir::W), gp + 1000));

        for (neigh, g) in neighs {
            let currentg = *gmap.get(&neigh).unwrap_or(&usize::MAX);
            if g < currentg {
                gmap.insert(neigh, g);
                fmap.insert(neigh, g + h(neigh));
                open.insert(neigh);
                camefrom.insert(neigh, [p].into());
            } else if g == currentg {
                camefrom.get_mut(&neigh).unwrap().push(p);
            }
        }
    }
    None
}

#[main]
fn day16(inp: &'static str) -> Result<()> {
    let (_, mut map) = nom_err(Map::<Tile>::parse(inp))?;

    let (start, _) = map
        .iter()
        .filter(|(c, t)| **t == Tile::Start)
        .next()
        .expect("expected start tile");
    let (end, _) = map
        .iter()
        .filter(|(c, t)| **t == Tile::End)
        .next()
        .expect("expected end tile");
    map[start] = Tile::Space;
    map[end] = Tile::Space;

    println!("{map}");

    println!("{}", heuristic((start, Dir::E), end));

    let (points, cost) = shortest_paths(&map, (start, Dir::E), end).unwrap();

    println!("Part 1: {cost}");

    for point in &points {
        map[*point] = Tile::Short;
    }
    println!("{map}");
    println!("Part 2: {}", points.len());

    Ok(())
}
