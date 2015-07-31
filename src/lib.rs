#[macro_use] extern crate nom;

pub use value::Value;
pub use object::Object;

use std::str::{self, FromStr};

use nom::{multispace, digit, alphanumeric, IResult};

pub mod value;
pub mod object;

pub type Key = String;

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

named!(object<Object>,
       map!(many0!(entry), From::from));

named!(object_val<Value>,
       map!(delimited!(tag!("{"), object, tag!("}")),
       From::from));

named!(value<Value>,
       delimited!(opt!(multispace), alt!(number | boolean | string | object_val), opt!(multispace)));

named!(key<Key>,
       map_res!(
           map_res!(
               delimited!(opt!(multispace), alphanumeric, opt!(multispace)),
               str::from_utf8),
               FromStr::from_str));

named!(entry<(Key, Value)>,
chain!(key: key ~
       tag!("=") ~
       value: value,
       || { (key, value) }));

pub fn parse<T: AsRef<[u8]>>(input: T) -> Result<Object, ()> {
    let object = match object(input.as_ref()) {
        IResult::Done(_, object) => object,
        _ => return Err(())
    };

    Ok(object)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = parse("str = \"test\"\nnum = 42\nbool = true\nobj = { num = 666 }").unwrap();

        assert_eq!(data.get("str"), Some(&Value::Str("test".to_string())));
        assert_eq!(data.get("num"), Some(&Value::Num(42)));
        assert_eq!(data.get("bool"), Some(&Value::Bool(true)));

        let obj = match data.get("obj").unwrap() {
            &Value::Object(ref obj) => obj,
            _ => { assert!(false); unreachable!() }
        };

        assert_eq!(obj.get("num"), Some(&Value::Num(666)));
    }
}
