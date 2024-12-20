use aoc::prelude::*;

#[derive(Copy, Clone, Debug, ParseTile, DisplayTile, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    #[tile('.')]
    Safe,
    #[tile('#')]
    Corrupt,
}

fn gen_map(len: usize, input: &[Coords]) -> Map<Tile> {
    let mut mem = Map::<Tile>::new(len, len, Tile::Safe);
    for pos in input {
        mem[*pos] = Tile::Corrupt;
    }
    mem
}

fn find_shortest(len: usize, input: &[Coords]) -> Option<usize> {
    let mem = gen_map(len, input);

    let start = (0, 0);
    let h = |p: &Coords| (len - p.0) + (len - p.1);
    let edges = |p: &Coords| {
        mem.neigh(*p, true, false)
            .into_iter()
            .filter(|(c, _)| mem[*c] == Tile::Safe)
            .map(move |(c, _)| (c, 1))
    };
    let end = |p: &Coords| *p == (len - 1, len - 1);

    let astar = aoc::astar::AStar::run(start, h, edges, end);

    astar.triggered_end.map(|e| astar.g_map[&e])
}

#[main]
fn day18(inp: &'static str) {
    let (_, coords) = nom_err(separated_list1(
        line_ending,
        map(separated_pair(nom_u8, nom_char(','), nom_u8), |(x, y)| {
            (x as usize, y as usize)
        }),
    )(inp))
    .unwrap();

    let len = 71;

    let part1 = find_shortest(len, &coords[..1024]).unwrap();
    println!("Part 1: {part1}");

    assert!(find_shortest(len, &coords).is_none());
    let mut a = 1024;
    let mut b = coords.len();
    while a + 1 < b {
        let x = (a + b) / 2;
        if find_shortest(len, &coords[..x]).is_some() {
            a = x;
        } else {
            b = x;
        }
    }
    let (x, y) = coords[a];
    println!("Part 2: {x},{y}");
}
