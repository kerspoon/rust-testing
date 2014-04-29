
extern crate collections;
use collections::HashMap;
use std::gc::Gc;
use std::from_str::FromStr;
use std::fmt;

enum Value {
  Nil,
  Symbol(~str),
  String(~str),
  Number(int),
  Pair(~Value, ~Value),
  Lambda(~Value, ~Value)
}

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

fn gen_symbol(map : &mut HashMap<~str,Gc<Value>>, name: ~str) -> Gc<Value>{
  if !map.contains_key(&name) {
    let sym = Symbol(name.clone());
    let obj = Gc::new(sym);
    map.insert(name.clone(), obj);
  }

  map.get(&name).clone()
}

fn evalLisp(this: Gc<Value>, env: &mut HashMap<~str,Gc<Value>>) -> Gc<Value> {

  match this.borrow() {
    &Symbol(ref s) => {
      // TODO: look up in environment.
      this.clone()
    },
    _ => gen_symbol(env, ~"nil")
  }
}

fn main() {
  let program = Gc::new(Symbol(~"hello"));
  let mut env = HashMap::<~str,Gc<Value>>::new();
  env.insert(~"null", Gc::new(Number(44)));

  gen_symbol(&mut env, ~"nil");

  let res = evalLisp(program, &mut env);

  println!("res {}", res.borrow());
}

// put food in fridge
