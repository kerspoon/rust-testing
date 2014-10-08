
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate collections;
use std::from_str::FromStr;
use std::fmt;
use std::string;

pub enum Value {
  Nil,
  True,
  False,
  Symbol(string::String),
  String(string::String),
  Number(f64),
  Pair(Box<Value>, Box<Value>),
  Lambda(Box<Value>, Box<Value>)
}

// http://static.rust-lang.org/doc/0.10/std/from_str/trait.FromStr.html
// https://github.com/rust-lang/rust/blob/master/src/libsyntax/crateid.rs
impl FromStr for Value {

    fn from_str(s: &str) -> Option<Value> {

        let starts_with_number = regex!(r"^-|\d");

        if s == "nil" {
            Some(Nil)
        } else if s == "true" {
            Some(True)
        } else if s == "false" {
            Some(False)
        } else if starts_with_number.is_match(s) {
            match from_str(s) {
                Some(num) => Some(Number(num)),
                _ => None
            }
        } else if s.starts_with("\"") {
            Some(String(s[1..-1].to_string()))
        } else if s.len() != 0 {
            Some(Symbol(s.to_string()))
        } else {
            None
        }
    }
}

// https://github.com/mozilla/rust/blob/master/src/test/run-pass/ifmt.rs
impl fmt::Show for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Nil =>  write!(f, "nil"),
            True =>  write!(f, "true"),
            False =>  write!(f, "false"),
            Symbol(ref x) => write!(f, "{}", x),
            String(ref x) => write!(f, "\"{}\"", x.replace("\"", "\\\"")),
            Number(x) => write!(f, "{}", x),
            _ => fail!("not implemented")
        }
    }
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world");
}

// ------------------------------------------------------------------------- //
// TESTS
// ------------------------------------------------------------------------- //

#[test]
fn nil_to_string() {
    assert_eq!("nil", Nil.to_string().as_slice());
}

#[test]
fn true_to_string() {
    assert_eq!("true", True.to_string().as_slice());
}

#[test]
fn false_to_string() {
    assert_eq!("false", False.to_string().as_slice());
}

#[test]
fn symbol_to_string() {
    assert_eq!("a123", Symbol("a123".to_string()).to_string().as_slice());
    assert_eq!("hi", Symbol("hi".to_string()).to_string().as_slice());
}

#[test]
fn string_to_string() {
    assert_eq!("\"a123\"", String("a123".to_string()).to_string().as_slice());
    assert_eq!("\"hi\"", String("hi".to_string()).to_string().as_slice());
    assert_eq!("\"this has a \\\" in the middle of it\"", String("this has a \" in the middle of it".to_string()).to_string().as_slice());
}

#[test]
fn number_to_string() {
    assert_eq!("123", Number(123.0).to_string().as_slice());
    assert_eq!("-5", Number(-5.0).to_string().as_slice());
    assert_eq!("0.5", Number(0.5).to_string().as_slice());
    assert_eq!("-0.01", Number(-0.01).to_string().as_slice());
}

#[test]
fn empty_from_string() {
    let tmp: Option<Value> = from_str("");
    match tmp {
        Some(_) => assert!(false),
        _ => assert!(true)
    }
}

#[test]
fn nil_from_string() {
    let tmp: Option<Value> = from_str("nil");
    match tmp {
        Some(Nil) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn true_from_string() {
    let tmp: Option<Value> = from_str("true");
    match tmp {
        Some(True) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn false_from_string() {
    let tmp: Option<Value> = from_str("false");
    match tmp {
        Some(False) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn symbol_from_string() {
    let tmp: Option<Value> = from_str("hello");
    match tmp {
        Some(Symbol(ref x)) => assert_eq!(x.as_slice(), "hello"),
        _ => assert!(false)
    }
}

#[test]
fn number_from_string() {
    {
        let tmp: Option<Value> = from_str("0");
        match tmp {
            Some(Number(x)) => assert_eq!(x, 0.0),
            _ => assert!(false)
        }
    }
    {
        let tmp: Option<Value> = from_str("1");
        match tmp {
            Some(Number(x)) => assert_eq!(x, 1.0),
            _ => assert!(false)
        }
    }
    {
        let tmp: Option<Value> = from_str("-9.01");
        match tmp {
            Some(Number(x)) => assert_eq!(x, -9.01),
            _ => assert!(false)
        }
    }
}
