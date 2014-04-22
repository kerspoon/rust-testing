
extern crate collections;
use collections::HashMap;
use std::rc::Rc;

enum LispVal {
  LispSymbol(~str),
  LispNumber(int),
  LispString(~str),
  LispPair(~LispVal, ~LispVal),
  LispLambda(~LispVal, ~LispVal)
}

fn add_symbol_if_missing(mut map : HashMap<~str,Rc<LispVal>>, name: ~str) {
  if !map.contains_key(&name) {
    let sym = LispSymbol(name.clone());
    let obj = Rc::new(sym);
    map.insert(name, obj);
  }
}

fn main() {
  let map = HashMap::new();
  add_symbol_if_missing(map, ~"hi");
}
