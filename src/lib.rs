pub trait TailRec: Sized {
    #[inline]
    fn tailrec<B, F>(self, iterate: F) -> B where F: Fn(Self) -> Result<B, Self> {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }
}

impl<A> TailRec for A {}
