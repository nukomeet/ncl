#[macro_use] extern crate nom;

pub use value::Value;
pub use object::Object;

use std::str::{self, FromStr};

use nom::{
    digit,
    eof,
    space,
    not_line_ending,
    line_ending,
    IResult,
    is_alphabetic,
    is_alphanumeric,
};

pub mod value;
pub mod object;

pub type Key = String;
type Entry = (Key, Value);

named!(comment,
       preceded!(tag!("#"), not_line_ending));

named!(blank,
       chain!(many0!(terminated!(
               many0!(alt!(comment | space)),
               line_ending)),
               || { &b""[..] }));

named!(boolean<Value>,
       map!(
           map_res!(
               map_res!(
                   alt!(tag!("true") | tag!("false")),
                   str::from_utf8),
                   <bool as FromStr>::from_str),
                   From::from));

named!(number<Value>,
       map!(
           map_res!(
               map_res!(
                   digit,
                   str::from_utf8),
                   <i64 as FromStr>::from_str),
                   From::from));

named!(string<Value>,
       map!(
           map_res!(
               delimited!(tag!("\""), take_until!("\""), tag!("\"")),
               str::from_utf8),
               From::from));

named!(entries<Object>,
       map!(many0!(entry), From::from));

named!(object_begin,
       chain!(space? ~ tag!("{") ~ alt!(blank | space)?, || { &b""[..] }));

named!(object_end,
       chain!(space? ~ tag!("}"), || { &b""[..] }));

named!(object<Value>,
       map!(delimited!(object_begin, entries, object_end),
       From::from));

named!(value<Value>,
       delimited!(opt!(space), alt!(number | boolean | string), opt!(space)));

fn keyable<'a>(input: &'a [u8]) -> IResult<'a, &'a [u8], &[u8]> {
    if input.len() > 0 && !is_alphabetic(input[0]) {
        return IResult::Error(nom::Err::Position(666, input));
    }
    for idx in 1..input.len() {
        if !is_alphanumeric(input[idx]) && input[idx] != b'_' && input[idx] != b'-' {
            return IResult::Done(&input[idx..], &input[0..idx]);
        }
    }
    IResult::Done(b"", input)
}

named!(key<Key>,
       map_res!(
           chain!(key: keyable ~
                  space?,
                  || { str::from_utf8(key).unwrap() }),
                  FromStr::from_str));

named!(entry<Entry>,
       alt!(
           chain!(space? ~
                  key: key ~
                  space? ~
                  value: object ~
                  blank?,
                  || { (key, value) }) |
           chain!(space? ~
                  key: key ~
                  tag!("=") ~
                  value: value ~
                  blank?,
                  || { (key, value) })));

named!(root<Object>,
       terminated!(delimited!(opt!(blank), entries, many0!(alt!(space | line_ending | comment))), eof));

#[derive(Debug)]
pub enum Error {
    ParserFailed(String),
    Incomplete
}

pub fn parse<T: AsRef<[u8]>>(input: T) -> Result<Object, Error> {
    match root(input.as_ref()) {
        IResult::Done(_, object) => Ok(object),
        IResult::Error(err) => Err(Error::ParserFailed(format!("{:?}", err))),
        IResult::Incomplete(_) => Err(Error::Incomplete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = parse("str = \"test\"\nnum = 42\nbool = true\nobj { num = 666 }").unwrap();

        assert_eq!(data.get("str"),   Some(&Value::Str("test".to_string())));
        assert_eq!(data.get("num"),   Some(&Value::Num(42)));
        assert_eq!(data.get("bool"),  Some(&Value::Bool(true)));

        let obj = match data.get("obj").unwrap() {
            &Value::Object(ref obj) => obj,
            _ => { assert!(false); unreachable!() }
        };

        assert_eq!(obj.get("num"), Some(&Value::Num(666)));
    }

    #[test]
    fn it_fails() {
        let data = parse("str = \"test\"\nnum = 42\nbool = true\nobj = { num = 666");

        assert!(data.is_err(), "data is: {:?}", data);
    }
}
