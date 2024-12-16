use aoc::prelude::*;

mod part1;
mod part2;

fn expand_map2(map: &Map<part1::Tile>) -> Map<part2::Tile> {
    let mut buf = Vec::with_capacity(map.tiles.len() * 2);
    for tile in &map.tiles {
        buf.extend(match *tile {
            part1::Tile::Wall => [part2::Tile::Wall, part2::Tile::Wall],
            part1::Tile::Box => [part2::Tile::BoxL, part2::Tile::BoxR],
            part1::Tile::Space => [part2::Tile::Space, part2::Tile::Space],
            part1::Tile::Start => [part2::Tile::Start, part2::Tile::Space],
        })
    }
    Map::from_buf(map.width() * 2, buf)
}

#[main]
fn day13(inp: &'static str) -> Result<()> {
    let (_, (map, moves)) = nom_err(separated_pair(
        Map::<part1::Tile>::parse,
        line_ending,
        many1(preceded(
            opt(line_ending),
            alt((
                value(Dir::N, nom_char('^')),
                value(Dir::E, nom_char('>')),
                value(Dir::S, nom_char('v')),
                value(Dir::W, nom_char('<')),
            )),
        )),
    )(inp))?;

    let map2 = expand_map2(&map);
    let p1 = part1::part1(map, &moves)?;
    println!("Part 1: {p1}\n");

    let p2 = part2::part2(map2, &moves)?;
    println!("Part 2: {p2}");

    Ok(())
}
