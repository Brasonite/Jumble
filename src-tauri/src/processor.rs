use nom::{
    IResult, Parser, branch::alt, bytes::complete::take_while1, combinator::map, multi::many0,
};

#[derive(Debug, Clone)]
pub enum Segment {
    Fixed(String),
    Flexible(String),
}

pub fn parse_fixed(input: &str) -> IResult<&str, Segment> {
    map(take_while1(|c: char| !c.is_alphabetic()), |x: &str| {
        Segment::Fixed(x.to_string())
    })
    .parse(input)
}

pub fn parse_flexible(input: &str) -> IResult<&str, Segment> {
    map(take_while1(|c: char| c.is_alphabetic()), |x: &str| {
        Segment::Flexible(x.to_string())
    })
    .parse(input)
}

pub fn parse_segment(input: &str) -> IResult<&str, Segment> {
    alt((parse_fixed, parse_flexible)).parse(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Segment>> {
    many0(parse_segment).parse(input)
}
