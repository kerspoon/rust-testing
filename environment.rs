
extern crate collections;
use collections::HashMap;
use std::gc::Gc;

enum Value {
  Nil,
  Symbol(~str),
  String(~str),
  Number(int),
  Pair(~Value, ~Value),
  Lambda(~Value, ~Value)
}

struct Env {
  elms : HashMap<~str,Gc<Value>>,
  parent : Option<~Env>
}

fn envGet(env: &mut Env, name: ~str) -> Gc<Value> {

}

fn envAdd(env: &mut Env, key: ~str, val: Gc<Value>) {

}

// I'll probably need garbage collection here for the returned value
fn envNewFrame(env: &mut Env) -> Env {

}

fn main() {

}