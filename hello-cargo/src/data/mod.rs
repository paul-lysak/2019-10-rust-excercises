pub use std::fmt;

#[derive(Clone, Copy, Debug, Default)]
pub struct Foo<AType: fmt::Display> {
    pub a: AType,
    _version: usize
}

impl<AType : fmt::Display> fmt::Display for Foo<AType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.a)
    }
}

impl<AType : fmt::Display + Default> Foo<AType> {
    pub fn new(a: AType) -> Self {
        Foo { a: a, ..Default::default() }
    }
}

