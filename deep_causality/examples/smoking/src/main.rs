// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

mod run;

use deep_causality::prelude::time_execution;

fn main() {
    time_execution(run::run, "main_run");
}
