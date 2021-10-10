#![allow(unused)]

fn foo() {}

#[macros::nothing]
fn to_nothing() {
    // completion does not work here at all, because these tokens have no meaning
    argle.bargle()
}

#[macros::simple_identity]
fn through_simple_identity() {
    // 1.| completion works here
    foo();
}

#[macros::parsing_identity_bad]
fn through_parsing_identity_bad() {
    // 1.| completion does *not* work here
    foo();
}

#[macros::parsing_identity_workaround]
fn through_parsing_identity_workaround() {
    // 1.| completion works here
    foo();
}

#[macros::parsing_identity_body_passthrough]
fn through_parsing_identity_body_passthrough() {
    // 1.| completion works here
    foo();
}

fn main() {
    println!("Hello, world!");
}
