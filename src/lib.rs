pub trait TailRec<Output, F> {
    #[inline]
    fn rec(self, iterate: F) -> Output where
        Self: Sized, F: Fn(Self) -> Result<Output, Self>
    {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }

    #[inline]
    fn rec_ref<'a>(&'a self, iterate: F) -> Output where
        F: for<'r> Fn(&'r Self) -> Result<Output, &'r Self>
    {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }

    #[inline]
    fn rec_mut(self, mut iterate: F) -> Output where
        Self: Sized, F: FnMut(Self) -> Result<Output, Self>
    {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }

    #[inline]
    fn rec_ref_mut<'a>(&'a mut self, mut iterate: F) -> Output where
        F: for<'r> FnMut(&'r mut Self) -> Result<Output, &'r mut Self>
    {
        let mut state = iterate(self);
        loop { match state {
            Ok (done) => { return done }
            Err(more) => { state = iterate(more); }
        }}
    }
}

impl<Output, F, State> TailRec<Output, F> for State {}
