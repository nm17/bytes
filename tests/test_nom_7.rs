#![cfg(feature = "nom-7")]

use std::any::Any;
use nom_7::bytes::complete::{tag, take, take_until, take_until1, take_while};
use nom_7::character::is_digit;
use nom_7::combinator::rest;
use nom_7::multi::{separated_list0, separated_list1};
use bytes::Bytes;

#[test]
fn test_simple_parser() {
    let mut parser = separated_list1(
        tag::<Bytes, Bytes, nom::error::Error<Bytes>>(Bytes::copy_from_slice(b"|")),
        take_while(|inp| inp != b"|"[0])
    );

    let (input, abc): (Bytes, Vec<Bytes>) = parser(Bytes::copy_from_slice(b"abc|qwe")).unwrap();

    dbg!(&input, &abc);

    assert_eq!(input.len(), 0);
    assert_eq!(abc.len(), 2)
}