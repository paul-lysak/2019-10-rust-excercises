use std::fmt;

struct Foo {
  a: usize,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.a)
  }
}

fn say_hello<T: fmt::Display>(who: T) {
  println!("Hello {}", who);
}

fn main() {
  let a = Foo { a: 42 };
  say_hello("world");
  say_hello(&a);
  say_hello(a);
}

