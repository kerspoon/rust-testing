
extern crate collections;
use collections::HashMap;
use std::rc::Rc;

// std::to_str[src] The ToStr trait for converting to strings

enum LispVal {
  LispSymbol(~str),
  LispNumber(int),
  LispString(~str),
  LispPair(~LispVal, ~LispVal),
  LispLambda(~LispVal, ~LispVal)
  /*LispSpecial(Builtin),*/
}

fn test_get(){
  let mut map = HashMap::<~str, uint>::new();
  map.insert(~"foo", 1);

  let r = *map.get(&~"foo");

  map.insert(~"bar", 2); // works!
}

/*
fn tmp(map : &HashMap<~str, LispVal>, name: ~str) -> &~LispVal {

  let ref r = *map.get(&name);
  //Use this like let foo = box(GC) Bar::new(...);
  r
}*/

/*

// we clearly need shared pointers for this, bring on memory management.

fn gen_symbol(map : HashMap<~str, LispVal>, name: str) -> &LispVal {
  if !map.contains_key(name) {
    map.insert(name, LispSymbol(name));
  }

  // fn get<'a>(&'a self, k: &K) -> &'a V
  map.get(&name)
}
*/
/*
fn gen_symbol(name: ~str) -> LispVal {
  map.find_or_insert(name, LispSymbol(name));
}*/

#[test]
fn test_gen_sym() {
  let mut map = HashMap::<~str, LispVal>::new();
  map.insert(~"null", LispSymbol(~"null"));
  assert_eq!(map.len(), 1);

  // map.insert(~"null", ~gen_symbol(map, ~"null"));
  // assert_eq!(gen_symbol(map, ~"hi"), gen_symbol(map, ~"hi"));
}

#[test]
fn test_get_sym() {
  let mut map = HashMap::<~str, LispVal>::new();
  map.insert(~"null", LispSymbol(~"null"));
  map.get(&~"null"); // the memory location of the heap allocated string.

  // map.insert(~"null", ~gen_symbol(map, ~"null"));
  // assert_eq!(gen_symbol(map, ~"hi"), gen_symbol(map, ~"hi"));
}

#[test]
fn test_three() {
  let mut map = HashMap::<~str, ~LispVal>::new();
  map.insert(~"null", ~LispNumber(44));
  tmp(map, ~"null");

  // map.insert(~"null", ~gen_symbol(map, ~"null"));
  // assert_eq!(gen_symbol(map, ~"hi"), gen_symbol(map, ~"hi"));
}

/*
impl LispVal {
  fn eval(&self, env: ) -> f64 {
    match *self {

    }
  }
}
*/


/*
struct LispSymbol { name: string }
struct LispNumber { value: float }
struct LispString { name: string }
struct LispPair { name: string }
LispPair car cdr
LispLambda argnames body
LispSpecial(name, func)
*/
