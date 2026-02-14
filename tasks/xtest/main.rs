// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │  ██████╗ ██████╗  ██████╗   Copyright (C) 2022, The Prospective Company   │
// │  ██╔══██╗██╔══██╗██╔═══██╗                                                │
// │  ██████╔╝██████╔╝██║   ██║  This file is part of the Procss library,      │
// │  ██╔═══╝ ██╔══██╗██║   ██║  distributed under the terms of the            │
// │  ██║     ██║  ██║╚██████╔╝  Apache License 2.0.  The full license can     │
// │  ╚═╝     ╚═╝  ╚═╝ ╚═════╝   be found in the LICENSE file.                 │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

use std::env::args;
use std::process::{exit, Command};

use once_cell::sync::Lazy;
use procss_utils::*;

static GRCOV_BIN: &str = env!("CARGO_BIN_FILE_GRCOV_grcov");
static IS_RELEASE: Lazy<bool> = Lazy::new(|| args().any(|x| x == "--release"));
static IS_COVERAGE: Lazy<bool> = Lazy::new(|| args().any(|x| x == "--coverage"));

fn main() {
    let target = get_default_target();
    let mut cargo_cmd = Command::new("cargo");
    let mut cargo_cmd_args = vec!["test", "--features", "iotest", "--target", target];
    if *IS_RELEASE {
        cargo_cmd_args.push("--release");
    }

    cargo_cmd_args.push("--");
    cargo_cmd_args.push("--test-threads=1");
    cargo_cmd.args(cargo_cmd_args);
    if *IS_COVERAGE {
        std::fs::remove_dir_all("target/prof").unwrap_or_default();
        const COV_RUST_FLAGS: [&str; 7] = [
            "-Cinstrument-coverage",
            "-Ccodegen-units=1",
            "-Copt-level=0",
            "-Clink-dead-code",
            "-Coverflow-checks=off",
            "-Zpanic_abort_tests",
            "-Cpanic=abort",
        ];

        cargo_cmd
            .env("RUSTFLAGS", COV_RUST_FLAGS.join(" "))
            .env("RUSTDOCFLAGS", "-Cpanic=abort")
            .env("CARGO_INCREMENTAL", "0")
            .env("LLVM_PROFILE_FILE", "target/prof/cargo-test-%p-%m.profraw");
    }

    cargo_cmd.execute();
    if *IS_COVERAGE {
        const GRCOV_ARGS: [&str; 11] = [
            "./target/prof",
            "--binary-path",
            "./target/debug/deps/",
            "-s",
            ".",
            "--branch",
            "--ignore-not-existing",
            "--ignore",
            "../*",
            "--ignore",
            "/*",
        ];

        std::fs::remove_dir_all("target/coverage/html").unwrap_or_default();
        std::fs::create_dir_all("target/coverage/html").unwrap_or_default();
        let html_args = ["-t", "html", "-o", "target/coverage/html"];
        Command::new(GRCOV_BIN)
            .args(GRCOV_ARGS.into_iter().chain(html_args))
            .execute();

        std::fs::remove_dir_all("target/coverage/lcov").unwrap_or_default();
        std::fs::create_dir_all("target/coverage/lcov").unwrap_or_default();
        let lcov_args = ["-t", "lcov", "-o", "target/coverage/lcov/lcov.info"];
        let code = Command::new(GRCOV_BIN)
            .args(GRCOV_ARGS.into_iter().chain(lcov_args))
            .status()
            .expect("failed to execute process")
            .code();

        exit(code.unwrap());
    }
}
