pub trait TailRec: Sized {
    #[inline]
    fn rec<Output, F>(self, iterate: F) -> Output where F: Fn(Self) -> Result<Output, Self> {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }

    #[inline]
    fn rec_mut<Output, F>(self, mut iterate: F) -> Output where F: FnMut(Self) -> Result<Output, Self> {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }
}

impl<State> TailRec for State {}
