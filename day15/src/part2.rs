use aoc::prelude::*;

#[derive(Copy, Clone, Debug, DisplayTile, PartialEq, Eq)]
pub enum Tile {
    #[tile('#')]
    Wall,
    #[tile('[')]
    BoxL,
    #[tile(']')]
    BoxR,
    #[tile('.')]
    Space,
    #[tile('@')]
    Start,
}

fn do_push(map: &Map<Tile>, changes: &mut Vec<Coords>, pos: Coords, dir: Dir) -> Option<()> {
    match dir {
        Dir::N|Dir::S => {
            let l = dir.add_coords(pos, 1);
            let r = (l.0+1, l.1);
            match (map[l], map[r]) {
                (Tile::Wall, _) | (_, Tile::Wall) => None?,
                (Tile::BoxL, Tile::BoxR) => {
                    changes.push(l);
                    do_push(map, changes, l, dir)?;
                },
                (Tile::BoxR, Tile::BoxL) => {
                    changes.push((l.0-1, l.1));
                    changes.push(r);
                    do_push(map, changes, (l.0-1, l.1), dir)?;
                    do_push(map, changes, r, dir)?;
                },
                (Tile::BoxR, Tile::Space) => {
                    changes.push((l.0-1, l.1));
                    do_push(map, changes, (l.0-1, l.1), dir)?;
                },
                (Tile::Space, Tile::BoxL) => {
                    changes.push(r);
                    do_push(map, changes, r, dir)?;
                },
                (Tile::Space, Tile::Space) => {},
                _ => unreachable!()
            }
        },
        Dir::E|Dir::W => {
            let target = match dir {
                Dir::E => (pos.0 + 2, pos.1),
                Dir::W => (pos.0 - 1, pos.1),
                _ => unreachable!()
            };
            match map[target] {
                Tile::Wall => None?,
                Tile::BoxL => {
                    changes.push(target);
                    do_push(map, changes, target, dir)?;
                },
                Tile::BoxR => {
                    let next = (target.0-1, target.1);
                    changes.push(next);
                    do_push(map, changes, next, dir)?;
                },
                Tile::Space => {},
                Tile::Start => unreachable!()
            }
        },
        _ => unreachable!()
    }
    Some(())
}

fn push_to(map: &mut Map<Tile>, pos: Coords, dir: Dir) -> bool {
    let mut changes = Vec::new();
    let next = match map[pos] {
        Tile::BoxL => pos,
        Tile::BoxR => (pos.0-1, pos.1),
        Tile::Space => return true,
        Tile::Wall => return false,
        _ => unreachable!()
    };
    changes.push(next);
    let Some(()) = do_push(&*map, &mut changes, next, dir) else { return false };
    while let Some(change) = changes.pop() {
        map[change] = Tile::Space;
        map[(change.0+1, change.1)] = Tile::Space;
        let new = dir.add_coords(change, 1);
        map[new] = Tile::BoxL;
        map[(new.0+1, new.1)] = Tile::BoxR;
    }
    true
}

pub fn part2(mut map: Map<Tile>, moves: &[Dir]) -> Result<usize> {
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
        if push_to(&mut map, next, m) {
            pos = next;
        }
    }
    map[pos] = Tile::Start;

    println!("{map}");

    let mut part1 = 0;
    for (coord, tile) in map.iter() {
        if *tile == Tile::BoxL {
            part1 += coord.0 + (100*coord.1);
        }
    }

    Ok(part1)
}
