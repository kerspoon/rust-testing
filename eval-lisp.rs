
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

fn gen_symbol(map : &mut HashMap<~str,Gc<Value>>, name: ~str) -> Gc<Value>{
  if !map.contains_key(&name) {
    let sym = Symbol(name.clone());
    let obj = Gc::new(sym);
    map.insert(name.clone(), obj);
  }

  map.get(&name).clone()
}

fn evalLisp(this: Gc<Value>, env: &mut HashMap<~str,Gc<Value>>) -> Gc<Value> {
  Gc::new(Symbol(~"nil"))
}

fn main() {
  let program = Gc::new(Symbol(~"hello"));
  let mut env = HashMap::<~str,Gc<Value>>::new();
  env.insert(~"null", Gc::new(Number(44)));

  gen_symbol(&mut env, ~"nil");

  evalLisp(program, &mut env);
}

// put food in fridge
