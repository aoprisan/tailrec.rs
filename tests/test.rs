#![allow(unused)]

extern crate tailrec;
use tailrec::TailRec;

struct Acc { res: u64, lim: u64 }

fn iterate(n: u64) {
    n.tailrec(|acc| match acc {
        0 => { Ok(()) }
        k => { println!("{:0>5}: <loop>", k - 1); Err(k - 1) }
    })
}

#[inline]
fn incr(n: u64, m: u64) -> u64 {
    Acc { res: 1u64, lim: m }.tailrec(|acc| match acc {
        Acc { res, lim: 0u64 } => { Ok(res) }
        Acc { res, lim  } => {
            let (lhs, rhs) = (res + 1, lim - 1);
            println!("{:0>5}, {:0>5}", lhs, rhs);
            Err(Acc { res: lhs, lim: rhs })
        }
    })
}

#[inline]
fn mutual(n: u64) -> bool {
    Ok(n).tailrec(|acc| match acc {
        Ok(v) => { even(v) }
        Err(e) => { odd(e) }
    })
}

#[inline]
fn even(n: u64) -> Result<bool, Result<u64, u64>> { match n {
    0 => { Ok(true) }
    k => {
        println!("{:0>5}: odd", k);
        Err(Err(k - 1))
    }
}}

#[inline]
fn odd(n: u64) -> Result<bool, Result<u64, u64>> { match n {
    0 => { Ok(false) }
    k => {
        println!("{:0>5}: even", n);
        Err(Ok(n - 1))
    }
}}

#[test]
fn test() {
    println!("{:0>5}", incr(0u64, ::std::u16::MAX as u64));
    mutual(::std::u16::MAX as u64);
    iterate(::std::u16::MAX as u64);
}
