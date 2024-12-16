use aoc::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Token {
    Mul(u64, u64),
    Do(bool),
    Char,
}

impl Token {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(
                tuple((tag("mul("), nom_u64, tag(","), nom_u64, tag(")"))),
                |(_, a, _, b, _)| Token::Mul(a, b),
            ),
            value(Token::Do(true), tag("do()")),
            value(Token::Do(false), tag("don't()")),
            value(Token::Char, anychar),
        ))(i)
    }
}

#[main]
fn day3(inp: &'static str) -> Result<()> {
    let mut acc = 0;
    let mut enabled = true;

    let mut tokens = iterator(inp, Token::parse);
    for token in &mut tokens {
        match token {
            Token::Mul(a, b) => {
                if enabled {
                    acc += a * b;
                }
            }
            Token::Do(en) => enabled = en,
            _ => {}
        }
    }
    tokens.finish()?;
    dbg!(acc);

    Ok(())
}
