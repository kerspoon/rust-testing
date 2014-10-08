
extern crate collections;
use collections::HashMap;
use std::rc::Rc;

fn test_get() {
  let mut map = HashMap::new();
  map.insert(~"foo", 1);
  let r = *map.get(&~"foo");
  assert_eq!(r, 1);
}

enum LispVal {
  LispSymbol(~str),
  LispNumber(int),
}

fn test_get_enum() {
  let mut map = HashMap::<~str,LispVal>::new();
  println!("test_get_enum: {:?}", map);
  map.insert(~"foo", LispNumber(4));
  let ref r = map.get(&~"foo");
  println!("  : {:?}", r);
}

fn test_get_rc() {
  let x = Rc::new(5);
  let y = ~"foo";
  let mut map = HashMap::<~str,Rc<int>>::new();
  map.insert(y, x.clone());
  println!("test_get_rc: {:?}", map);
}

fn test_get_rc_enum() {
  // create a new reference counted object.
  let x = Rc::new(LispNumber(5));
  // create an owned pointer to the string
  let y = ~"foo";
  // create a changeable dictionary from strings to reference counted objects
  let mut map = HashMap::<~str,Rc<LispVal>>::new();
  // add a clone of our object
  map.insert(y, x.clone());
  // mostly here to let me know the type which I have added in above
  println!("test_get_rc_enum: {:?}", map);
  // get a reference to the object from the map
  // we need to pass in a reference to an object
  // ~ is used to create the object
  // & is used to get its ref.
  let ref r = map.get(&~"foo");

  // here we match on a dereferenced version of the object
  // To dereference means to borrow the pointer.
  // I did it because I can't work out the syntax for matching a
  // Rc object. (something like &Rc::<LispNumber(n)>)
  // anyway that give us a reference anyway.
  let answer = match r.deref() {
    // Note the we print out a line and then return the number
    &LispNumber(n) => {
      println!("  : {:?}", n);
      n
    },
    // we could either match LispSymbol or wildcard here
    _ => {
      println!("  : error");
      0
    }
  };
  // this works! (in my version)
  // rustc 0.11-pre-nightly (412a18f 2014-04-20 00:31:34 -0700)
  // host: i686-pc-mingw32

  assert_eq!(answer, 5);
}

// next up testing it with an owned pointer inside the object.


// the SecVal inside the pairs have to be references as otherwise the size
// would be infinite (it would be twice its own size).

enum SecVal {
  SecNumber(int),
  SecString(~str),
  SecPair(~SecVal, ~SecVal)
}

fn test_get_rc_enum_own() {
  // we don't need garbage collection for the inner objects
  // they can quite happily be owned by the SecPair.
  let tmp = SecPair(~SecNumber(4), ~SecString(~"hi"));
  let x = Rc::new(tmp);

  let mut map = HashMap::<~str,Rc<SecVal>>::new();
  map.insert(~"foo", x.clone());
  println!("test_get_rc_enum: {:?}", map);
  let ref r = map.get(&~"foo");

  let answer = match r.deref() {
    &SecPair(~SecNumber(n), _) => {
      println!("  : {:?}", n);
      n
    },
    _ => {
      println!("  : error");
      0
    }
  };

  assert_eq!(answer, 4);
}


fn main() {
  test_get();
  test_get_enum();
  test_get_rc();
  test_get_rc_enum();
  test_get_rc_enum_own();
  println!("end");
}

