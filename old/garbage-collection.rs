
use std::rc::Rc;

/*
 http://pcwalton.github.io/blog/2013/06/02/removing-garbage-collection-from-the-rust-language/

 > In practice, the rule has been that programmers should use ~ to allocate in
 > all circumstances except when they have no way of knowing precisely when the
 > object in question should be freed.

 https://github.com/mozilla/rust/blob/master/src/test/compile-fail/no_share-rc.rs
 https://github.com/mozilla/rust/blob/master/src/libstd/gc.rs
*/

fn test_simple_clone() {
  let x = Rc::new(5);
  let y = x.clone();
  assert_eq!(*x, 5);
  assert_eq!(*y, 5);
}

fn main() {
  test_simple_clone();
  println!("end");
}
