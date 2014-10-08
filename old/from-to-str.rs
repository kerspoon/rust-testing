use std::from_str::FromStr;
use std::fmt;
use std::gc::Gc;

enum Value {
  Nil,
  Symbol(~str),
  String(~str),
  Number(int),
  Pair(~Value, ~Value),
  Lambda(~Value, ~Value)
}

/*
  (define add (lambda (a b) (+ a b)))
  (add 2 3)
*/

/*
// we need the garbage collector here.
impl Gc<Value> {

  fn mmm(&self) {
    println!("xcvbnm");
  }

  fn eval(&self) -> Gc<Value> {
    match self {
      &Symbol(ref s) => {
        // TODO: look up in environment.
        Nil
      },
      &Pair(~Lambda(ref args, ref body), ref cdr) => {
        // TODO: apply.
        Nil
      },
      &Pair(_, _) => {
        fail!(~"cannot apply a non lambda");
      },
      _ => Gc::new(self)
    }
  }
}*/

// http://static.rust-lang.org/doc/0.10/std/from_str/trait.FromStr.html
impl FromStr for Value {
  fn from_str(s: &str) -> Option<Value> {
    if s[0] == '4' as u8 {
      Some(Number(4))
    } else {
      None
    }
  }
}

// https://github.com/mozilla/rust/blob/master/src/test/run-pass/ifmt.rs
impl fmt::Show for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        &Symbol(ref x) => f.buf.write(x.as_bytes()),
        _ => f.buf.write("other".as_bytes())
      }
    }
}

fn main() {

  let x = Symbol(~"hhh");
  println!("{} => {}", x, x.to_str());

  let m : Option<Value> = from_str("hi");
  println!("{} => {}", m, m.to_str());

  println!("End");
}

