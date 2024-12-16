use aoc::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Ant(char),
}

impl ParseTile for Tile {
    fn from_char(ch: char) -> Option<Self> {
        if ch == '.' {
            Some(Tile::Empty)
        } else if ch.is_alphanumeric() {
            Some(Tile::Ant(ch))
        } else {
            None
        }
    }
}

impl DisplayTile for Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Ant(ch) => ch,
        }
    }
}

fn antinodes_part1(set: &mut HashSet<Coords>, map: &Map<Tile>, a: Coords, b: Coords) {
    let dx = b.0.wrapping_sub(a.0);
    let dy = b.1.wrapping_sub(a.1);
    let c1 = (a.0.wrapping_sub(dx), a.1.wrapping_sub(dy));
    let c2 = (b.0.wrapping_add(dx), b.1.wrapping_add(dy));
    if map.valid(c1) {
        set.insert(c1);
    }
    if map.valid(c2) {
        set.insert(c2);
    }
}

fn reduce_d(dx: isize, dy: isize) -> (isize, isize) {
    if dx == 0 {
        return (0, 1);
    } else if dy == 0 {
        return (1, 0);
    }
    let c = num::integer::gcd(dx, dy);
    (dx / c, dy / c)
}

fn antinodes_part2(set: &mut HashSet<Coords>, map: &Map<Tile>, a: Coords, b: Coords) {
    let dx = b.0 as isize - a.0 as isize;
    let dy = b.1 as isize - a.1 as isize;
    let (dx, dy) = reduce_d(dx, dy);

    let (x, y) = (a.0 as isize, a.1 as isize);
    for idx in 0.. {
        let c = ((x - idx * dx) as usize, (y - idx * dy) as usize);
        if map.valid(c) {
            set.insert(c);
        } else {
            break;
        }
    }
    for idx in 1.. {
        let c = ((x + idx * dx) as usize, (y + idx * dy) as usize);
        if map.valid(c) {
            set.insert(c);
        } else {
            break;
        }
    }
}

fn find_antinodes<F>(map: &Map<Tile>, antinodes: F) -> HashSet<Coords>
where
    F: Fn(&mut HashSet<Coords>, &Map<Tile>, Coords, Coords),
{
    let mut ant: HashMap<char, Vec<Coords>> = HashMap::new();
    for (coord, tile) in map {
        if let Tile::Ant(ch) = tile {
            ant.entry(*ch).or_default().push(coord);
        }
    }

    let mut anti = HashSet::new();
    for (_, col) in &ant {
        for (a, b) in (0..col.len() - 1).flat_map(|x| (x + 1..col.len()).map(move |y| (x, y))) {
            let (ca, cb) = (col[a], col[b]);
            antinodes(&mut anti, &map, ca, cb);
        }
    }

    anti
}

#[main]
fn day8(inp: &'static str) -> Result<()> {
    let (i, map) = nom_err(Map::<Tile>::parse(inp))?;
    nom_err(eof(i))?;

    let part1 = find_antinodes(&map, antinodes_part1).len();
    let part2 = find_antinodes(&map, antinodes_part2).len();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
