#![allow(unused)]

extern crate tailrec;
use tailrec::TailRec;

struct Acc { res: u64, lim: u64 }

fn iterate(n: u64) {
    n.tailrec(|acc| match acc {
        0 => { Err(()) }
        k => { println!("{:0>5}: <loop>", k - 1); Ok(k - 1) }
    })
}

#[inline]
fn incr(n: u64, m: u64) -> u64 {
    Acc { res: 1u64, lim: m }.tailrec(|xs| {
        match xs {
            Acc { res, lim: 0u64 } => { Err(res) }
            Acc { res, lim  } => {
                let (lhs, rhs) = (res + 1, lim - 1);
                println!("{:0>5}, {:0>5}", lhs, rhs);
                Ok(Acc { res: lhs, lim: rhs })
            }
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
fn even(n: u64) -> Result<Result<u64, u64>, bool> {
    if n == 0 {
        Err(true)
    } else {
        println!("{:0>5}: even", n);
        Ok(Err(n - 1))
    }
}

#[inline]
fn odd(n: u64) -> Result<Result<u64, u64>, bool> {
    if n == 0 {
        Err(false)
    } else {
        println!("{}: odd", n);
        Ok(Ok(n - 1))
    }
}

fn main() {
    println!("{:0>5}", incr(0u64, ::std::u16::MAX as u64));
    mutual(::std::u16::MAX as u64);
    iterate(::std::u16::MAX as u64);
}
