use aoc::prelude::*;
use fraction::prelude::*;

#[derive(Debug)]
struct Crane {
    a: (u64, u64),
    b: (u64, u64),
    p: (u64, u64),
}

impl Crane {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (_, ax, _, ay, _)) = tuple((
            tag("Button A: X+"),
            nom_u64,
            tag(", Y+"),
            nom_u64,
            line_ending,
        ))(i)?;
        let (i, (_, bx, _, by, _)) = tuple((
            tag("Button B: X+"),
            nom_u64,
            tag(", Y+"),
            nom_u64,
            line_ending,
        ))(i)?;
        let (i, (_, px, _, py, _)) =
            tuple((tag("Prize: X="), nom_u64, tag(", Y="), nom_u64, line_ending))(i)?;
        Ok((
            i,
            Crane {
                a: (ax, ay),
                b: (bx, by),
                p: (px, py),
            },
        ))
    }

    fn matrix(&self) -> Option<[Fraction; 4]> {
        let a: [Fraction; 4] = [
            self.a.0.into(),
            self.a.1.into(),
            self.b.0.into(),
            self.b.1.into(),
        ];
        let det = (a[0] * a[3]) - (a[1] * a[2]);
        if det == 0.into() {
            None
        } else {
            Some([a[3] / det, -a[1] / det, -a[2] / det, a[0] / det])
        }
    }

    fn solve(&self) -> Option<(u64, u64)> {
        // This doesn't account for the possibility of linearly dependent
        // vectors which might have a solution, but the test input does not
        // seem to contain any...
        let m = self.matrix().unwrap();
        let (px, py) = (Fraction::from(self.p.0), Fraction::from(self.p.1));
        let (cx, cy) = (m[0] * px + m[2] * py, m[1] * px + m[3] * py);
        if cx.denom() == Some(&1) && cy.denom() == Some(&1) {
            Some((cx.numer().copied().unwrap(), cy.numer().copied().unwrap()))
        } else {
            None
        }
    }
}

const PART2_OFFSET: u64 = 10000000000000;

#[main]
fn day13(inp: &'static str) -> Result<()> {
    let (_, cranes) = separated_list1(line_ending, Crane::parse)(inp)?;
    let (mut part1, mut part2) = (0, 0);
    for mut crane in cranes {
        if let Some((pa, pb)) = crane.solve() {
            part1 += 3 * pa + pb;
        }
        crane.p.0 += PART2_OFFSET;
        crane.p.1 += PART2_OFFSET;
        if let Some((pa, pb)) = crane.solve() {
            part2 += 3 * pa + pb;
        }
    }
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
