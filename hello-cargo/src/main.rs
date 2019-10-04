mod data;

use data::*;

fn say_hello<T: fmt::Display + fmt::Debug>(who: T) {
    //    println!("Hello {}", who);
    //    println!("Hello {:?}", who);
    dbg!("Hello {}", who);
}

fn main() {
    let a = Foo::new(42);
//    let defaultA: Foo = Default::default();
    say_hello("world");
    say_hello(&a);
    say_hello(a);
//    say_hello(defaultA);
}
