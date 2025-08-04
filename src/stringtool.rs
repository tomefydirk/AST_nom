use nom::branch::alt;
use nom::error::{Error, ErrorKind};
use nom::{IResult, Parser};

pub struct Tokeniser {

}

pub fn my_tag(input: String, pat: String) -> IResult<String, String> {
    if input.starts_with(&pat) {
        let rest = input[pat.len()..].to_string();
        Ok((rest, pat))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

pub fn my_alt(input: String, patterns: Vec<String>) -> IResult<String, String> {
    for pat in patterns {
        match my_tag(input.clone(), pat.clone()) {
            Ok(ok) => return Ok(ok),
            Err(_) => continue,
        }
    }

    Err(nom::Err::Error(nom::error::Error::new(input, ErrorKind::Alt)))
}



impl Parser<String> for Tokeniser {
    type Output = Token;

    type Error = Error<String>;

    fn process<OM: nom::OutputMode>(
        &mut self,
        input: String,
    ) -> nom::PResult<OM, String, Self::Output, Self::Error> {
        todo!()
    }
}
//todo()!
pub enum Token {
    Number(f64),
    Other(String),
}

#[cfg(test)]
mod test {
    use crate::stringtool::my_tag;

    #[test]
    fn test1() {
        println!("HERE");
        let a = String::from("))12");
        let b = String::from("))");

        let result = my_tag(a, b);

        println!("{:?}",result);
    }
}
