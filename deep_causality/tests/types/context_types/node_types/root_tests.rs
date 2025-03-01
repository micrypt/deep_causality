// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.



use deep_causality::prelude::{Root, Identifiable};

#[test]
fn test_new() {

    let id = 1;
    let d = Root::new(id);
    assert_eq!(d.id(), id);
}

#[test]
fn test_id() {
    let id = 1;

    let d = Root::new(id);
    assert_eq!(d.id(), id);
}

#[test]
fn test_to_string() {
    let id = 1;

    let d = Root::new(id);
    assert_eq!(d.id(), id);

    let exp = format!("Root ID: {}", id);
    let act = d.to_string();
    assert_eq!(act, exp);
}