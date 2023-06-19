use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::value,
    sequence::{delimited, preceded},
    IResult,
};

pub fn parse_line(input: &str) -> Result<(String, bool), Box<dyn Error + '_>> {
    let (entry, _) = remove_point(&input)?;
    let (rest, status) = get_between_brackets(entry)?;
    let (name, _) = remove_leading_space(rest)?;
    Ok((name.to_owned(), status))
}

fn remove_leading_space(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

fn remove_point(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag("- "))(input)
}

fn get_between_brackets(input: &str) -> IResult<&str, bool> {
    let (remaing, status) = delimited(tag("["), is_x, tag("]"))(input)?;

    Ok((remaing, status))
}

fn is_x(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("x")), value(false, tag(" "))))(input)
}
