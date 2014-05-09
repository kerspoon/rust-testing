
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

fn test_basic() {
  let mut env = Env {
    frame: HashMap::<~str,Gc<Value>>::new(),
    parent: None
  };

  let num = Gc::new(Number(4));
  envAdd(&mut env, ~"hi", num.clone());
  let num2 = envGet(&mut env, ~"hi");
  assert!(num.ptr_eq(&num2));
}


// I'll probably need garbage collection here for the returned value
fn envNewFrame(env: &mut Gc<Env>) -> Gc<Env> {
  Gc::new(Env {
    frame: HashMap::<~str,Gc<Value>>::new(),
    parent: Some(env.clone())
  })
}

fn test_new_frame() {
  let mut env = Gc::new(Env {
    frame: HashMap::<~str,Gc<Value>>::new(),
    parent: None
  });

  let newEnv = envNewFrame(&mut env);
  let newEnvBorrow = newEnv.borrow();

  match newEnvBorrow.parent {
    Some(thing) => {
      assert!(env.ptr_eq(& thing));
    }
    _ => {
      fail!(~"expected parent to exist");
    }
  }
}

fn main() {
  test_basic();
  test_new_frame();
}
