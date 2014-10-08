
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

// -------------------- Environment -------------------- //

struct Env {
  frame : HashMap<~str,Gc<Value>>,
  parent : Option<Gc<Env>>
}

// get from the environment a value.
// if it is not in this frame then look in parent
// if there is no parent then return Nil.
// should this be Nil or error in some other way.
fn envGet(env: & Env, key: ~str) -> Gc<Value> {
  if env.frame.contains_key(&key) {
    env.frame.get(&key).clone()
  } else {
    match env.parent {
      None => Gc::new(Nil),
      Some(ref parent) => {
        let tmp = parent.borrow();
        envGet(tmp, key)
      }
    }
  }
}

fn envAdd(env: &mut Env, key: ~str, val: Gc<Value>) {
  env.frame.insert(key.clone(), val.clone());
}

fn envNewFrame(env: &mut Gc<Env>) -> Gc<Env> {
  Gc::new(Env {
    frame: HashMap::<~str,Gc<Value>>::new(),
    parent: Some(env.clone())
  })
}

fn envCreate(symbols : &mut HashMap<~str,Gc<Value>>) -> Gc<Env> {
  let mut env = Gc::new(Env {
    frame: HashMap::<~str,Gc<Value>>::new(),
    parent: None
  });

  let &mut borrow = env.borrow();

  envAdd(& borrow, ~"nil", Gc::new(Nil));
  envAdd(& borrow, ~"true", gen_symbol(symbols, ~"true"));
  envAdd(& borrow, ~"false", gen_symbol(symbols, ~"false"));

  // the next task is to work out how to add things to my env.

  env
}

// -------------------- Symbol Table -------------------- //

fn gen_symbol(symbols : &mut HashMap<~str,Gc<Value>>, name: ~str) -> Gc<Value>{
  if !symbols.contains_key(&name) {
    let sym = Symbol(name.clone());
    let obj = Gc::new(sym);
    symbols.insert(name.clone(), obj);
  }

  symbols.get(&name).clone()
}

// -------------------- Eval -------------------- //

fn evalLisp(this: Gc<Value>, env: &mut Gc<Env>, symbols: &mut HashMap<~str,Gc<Value>>) -> Gc<Value> {

  match this.borrow() {
    &Symbol(ref s) => {
      envGet(env.borrow(), s.clone())
      //this.clone()
    },
    _ => gen_symbol(symbols, ~"nil")
  }
}

// -------------------- Test Helpers -------------------- //

//println!("bbb {}", strEvalInNewEnv(~"hello"));
fn strEvalInNewEnv(text: ~str) -> ~str {
  let parsed : Option<Value> = from_str(text);
  match parsed {
    None => {
      ~"error parsing"
    },
    Some(val) => {
      let program = Gc::new(val);
      let mut env = Gc::new(Env {
        frame: HashMap::<~str,Gc<Value>>::new(),
        parent: None
      });
      let mut symbols = HashMap::<~str,Gc<Value>>::new();
      let res = evalLisp(program, &mut env, &mut symbols);
      res.borrow().to_str()
    }
  }
}

//println!("aaa {}", evalInNewEnv(Gc::new(Symbol(~"hello"))));
fn evalInNewEnv(program: Gc<Value>) -> ~str {
  let mut env = Gc::new(Env {
    frame: HashMap::<~str,Gc<Value>>::new(),
    parent: None
  });
  let mut symbols = HashMap::<~str,Gc<Value>>::new();
  let res = evalLisp(program, &mut env, &mut symbols);
  res.borrow().to_str()
}

fn test_basic(){
  println!("aaa {}", evalInNewEnv(Gc::new(Symbol(~"hello"))));
}

// runTest(~"()", ~"nil");
fn runTest(input : ~str, expected: ~str) {
  println!("testing {}", input);
  let output = strEvalInNewEnv(input);
  assert_eq!(expected, output);
}

fn main() {
  test_basic();
  runTest(~"()", ~"nil");
}
