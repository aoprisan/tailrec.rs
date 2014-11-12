pub type Lazy<'a, X> = Box<FnOnce<(), X> + 'a>;
pub fn map<'a, X, Y, F:'a>(m: Lazy<'a, X>, f: F) -> Lazy<'a, Y>
    where
        F: FnOnce(X) -> Y,
{
    box move |:| f(m.call_once(()))
}
free_monad!(Trampoline, Lazy, map, [])

impl<'a, X> Trampoline<'a, X> {
    pub fn run(self) -> X {
        self.go(|&:sbmx: Lazy<'a, Box<_>>| *sbmx.call_once(()))
    }
}

#[inline(always)]
pub fn done<'a, X>(a: X) -> Trampoline<'a, X> {
    Pure(a).wrap()
}

#[inline(always)]
pub fn more<'a, X>(ma:Lazy<'a, Trampoline<'a, X>>) -> Trampoline<'a, X> {
    Roll(map(ma, |:tx: Trampoline<'a, _>| box tx.proj())).wrap()
}

#[cfg(test)]
mod tests {
    extern crate num;

    use self::num::BigInt;
    use super::{
        Trampoline,
        done,
        more,
    };

    fn factorial<'a>(n:uint, acc:BigInt) -> Trampoline<'a, BigInt> {
        if n <= 2 {
            done(acc)
        } else {
            let nb: BigInt = FromPrimitive::from_uint(n).unwrap();
            more(box move |:| {
                factorial(n - 1, nb * acc)
            })
        }
    }

    #[test]
    fn welp() {
        let acc: BigInt = FromPrimitive::from_uint(1u).unwrap();
        println!("{}", factorial(1500, acc).run())
    }
}
