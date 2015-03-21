#![allow(unused)]

pub trait TailRec: Sized {
    #[inline]
    fn tailrec<B, F>(self, f: F) -> B where F: Fn(Self) -> Result<Self, B> {
        let mut res = f(self);
        loop { match res {
            Ok (v) => { res = f(v); }
            Err(e) => { return e }
        }}
    }
}

impl<A> TailRec for A {}
