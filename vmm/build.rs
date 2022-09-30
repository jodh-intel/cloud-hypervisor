// Copyright © 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use std::fs::copy;
use std::io::Result;

fn main() -> Result<()> {
    let in_file = "../vm-config/src/lib.rs";
    let out_file = "src/config_api.rs";

    println!("cargo:rerun-if-changed={}", in_file);

    copy(in_file, out_file)?;

    Ok(())
}
