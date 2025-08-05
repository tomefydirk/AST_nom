use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::{IResult, Parser};

/*
Explication :
    un "token" peut etre un "float"(f64) ou ('-','+','*','/','(',')','ln','V') appelé "other_token"


    space0 est utilisé ici par convention
*/
pub fn scan_float(input: &str) -> IResult<&str, Token> {
    let (rest, first_part) = digit1(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    if point.is_some() {
        let (rest3, second_part) = digit1(rest2)?;
        Ok((
            rest3,
            Token::Number(format!("{first_part}.{second_part}").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })?),
        ))
    } else {
        Ok((
            rest,
            Token::Number(format!("{first_part}.0").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })?),
        ))
    }
}

pub fn tag_other_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag("-"),
        tag("+"),
        tag("*"),
        tag("/"),
        tag("("),
        tag(")"),
        tag("ln"),
        tag("V"),
        space0,
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(mut input: &str) -> IResult<&str, Token> {
    input = input.trim();
    alt((scan_float, tag_other_token)).parse(input)
}

//Enum pour les token :
#[derive(Debug)]
pub enum Token<'a> {
    Number(f64),
    Other(&'a str),
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
