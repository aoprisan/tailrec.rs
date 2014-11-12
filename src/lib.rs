#![crate_name="tailrec"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/tailrec.rs/doc/tailrec/")]

#![feature(overloaded_calls)]
#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(link, plugin)]
extern crate free;

pub mod trampoline;
