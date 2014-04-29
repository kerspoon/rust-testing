
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

//println!("bbb {}", strEvalInNewEnv(~"hello"));
fn strEvalInNewEnv(text: ~str) -> ~str {
  let parsed : Option<Value> = from_str(text);
  match parsed {
    None => {
      ~"error parsing"
    },
    Some(val) => {
      let program = Gc::new(val);
      let mut env = HashMap::<~str,Gc<Value>>::new();
      let res = evalLisp(program, &mut env);
      res.borrow().to_str()
    }
  }
}

//println!("aaa {}", evalInNewEnv(Gc::new(Symbol(~"hello"))));
fn evalInNewEnv(program: Gc<Value>) -> ~str {
  let mut env = HashMap::<~str,Gc<Value>>::new();
  let res = evalLisp(program, &mut env);
  res.borrow().to_str()
}

// runTest(~"()", ~"nil");
fn runTest(input : ~str, expected: ~str) {
  println!("testing {}", input);
  let output = strEvalInNewEnv(input);
  assert_eq!(expected, output);
}

fn main() {
  runTest(~"()", ~"nil");
}

// put food in fridge
