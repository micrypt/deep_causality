// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use deep_causality::prelude::{Dataoid, Identifiable};

#[test]
fn test_new() {
    let id = 1;
    let data = 42;

    let d = Dataoid::new(id, data);
    assert_eq!(d.id(), id);
}

#[test]
fn test_id() {
    let id = 1;
    let data = 42;

    let d = Dataoid::new(id, data);
    assert_eq!(d.id(), id);
}

#[test]
fn test_data() {
    let id = 1;
    let data = 42;

    let d = Dataoid::new(id, data);
    assert_eq!(d.id(), id);
    assert_eq!(d.data(), data);
}

#[test]
fn test_to_string() {
    let id = 1;
    let data = 42;

    let d = Dataoid::new(id, data);
    assert_eq!(d.id(), id);
    assert_eq!(d.data(), data);

    let exp = format!("Dataoid: id: {} data: {}", id, data);
    let act = d.to_string();
    assert_eq!(act, exp);
}