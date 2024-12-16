use aoc::prelude::*;

#[main]
fn day12(inp: &'static str) -> Result<()> {
    let (_, map) = nom_err(Map::<char>::parse(inp))?;

    let mut remaining: HashSet<Coords> = map.iter().map(|(c, _)| c).collect();

    let mut part1 = 0;
    let mut part2 = 0;

    while let Some(point) = remaining.iter().copied().next() {
        remaining.remove(&point);
        let ch = map[point];
        let mut edge = HashSet::new();
        let mut region = HashSet::new();
        let mut scan = HashSet::new();
        scan.insert(point);
        region.insert(point);
        while let Some(point) = scan.iter().copied().next() {
            scan.remove(&point);
            region.insert(point);
            remaining.remove(&point);
            for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
                let Some(neigh) = map.add(point, dir) else {
                    edge.insert((point, dir));
                    continue;
                };
                if map[neigh] == ch && !region.contains(&neigh) {
                    scan.insert(neigh);
                    region.insert(neigh);
                    remaining.remove(&neigh);
                } else if map[neigh] != ch {
                    edge.insert((point, dir));
                }
            }
        }
        part1 += region.len() * edge.len();

        let mut sides = 0;
        while let Some((coord, dir)) = edge.iter().copied().next() {
            edge.remove(&(coord, dir));
            sides += 1;
            for trans_dir in [Dir::E, Dir::W] {
                let mut pos = coord;
                loop {
                    let Some(new_pos) = map.add(pos, dir + trans_dir) else {
                        break;
                    };
                    pos = new_pos;
                    if !edge.remove(&(pos, dir)) {
                        break;
                    }
                }
            }
        }
        part2 += region.len() * sides;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
