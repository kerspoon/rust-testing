
extern crate collections;
use collections::HashMap;
use std::gc::Gc;



fn add_if_missing(map : &mut HashMap<~str,int>, name: ~str, num: int) {
  if !map.contains_key(&name) {
    map.insert(name, num);
  }
}

fn test_one() {
  let mut map = HashMap::<~str,int>::new();
  map.insert(~"hi", 5);
  add_if_missing(&mut map, ~"foo", 4);
}


enum LispVal {
  LispSymbol(~str),
  LispNumber(int),
  LispString(~str),
  LispPair(~LispVal, ~LispVal),
  LispLambda(~LispVal, ~LispVal)
}

fn add_symbol_if_missing(map : &mut HashMap<~str,Gc<LispVal>>, name: ~str) {
  if !map.contains_key(&name) {
    let sym = LispSymbol(name.clone());
    let obj = Gc::new(sym);
    map.insert(name, obj);
  }
}


fn gen_symbol(map : &mut HashMap<~str,Gc<LispVal>>, name: ~str) -> Gc<LispVal>{
  if !map.contains_key(&name) {
    let sym = LispSymbol(name.clone());
    let obj = Gc::new(sym);
    map.insert(name.clone(), obj);
  }

  // fn get<'a>(&'a self, k: &K) -> &'a V
  map.get(&name).clone()
}


fn main() {
  let mut map = HashMap::<~str,Gc<LispVal>>::new();
  println!("main: {:?}", map);
  test_one();
  add_symbol_if_missing(&mut map, ~"hi");
  add_symbol_if_missing(&mut map, ~"hi");

  assert!(gen_symbol(&mut map, ~"hi").ptr_eq(&gen_symbol(&mut map, ~"hi")));

}
