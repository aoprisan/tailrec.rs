pub trait TailRec: Sized {
    #[inline]
    fn tailrec<Output, F>(self, iterate: F) -> Output where F: Fn(Self) -> Result<Output, Self> {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }
}

impl<A> TailRec for A {}
