use std::{cell::RefCell, thread::LocalKey};


struct Foo;

impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(1);
    }
}
fn main() {
    Foo::FOO.with(|x| println!("{:?}", x));
    let b = Bar::constructor();
    b.foo.with(|x| println!("{:?}", x));
}


thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);
}

struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}

impl Bar {
    fn constructor() -> Self {
        Self { foo: &FOO }
    }
}