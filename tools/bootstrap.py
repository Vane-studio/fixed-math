#!/usr/bin/env python3
"""
Project Bootstrapper

Creates the initial project directory tree.

Rules:

1. Missing directories are created.
2. Missing files are created.
3. Existing files are never overwritten.
"""

from pathlib import Path


PROJECT = {
    "Cargo.toml": """\
[package]
name = "fixed-math"
version = "0.1.0"
edition = "2024"

[dependencies]
""",

    "README.md": "# fixed-math\n",

    ".gitignore": """\
target/
Cargo.lock
.idea/
.vscode/
""",

    "LICENSE": "",

    "rustfmt.toml": "",

    "docs": {
        "unit1.md": "",
    },

    "src": {
        "lib.rs": """\
pub mod number;
pub mod cf;
pub mod function;
pub mod error;
pub mod traits;

pub mod prelude;
""",

        "number": {
            "mod.rs": "",
            "fixed.rs": "",
            "constant.rs": "",
            "rounding.rs": "",
            "format.rs": "",
        },

        "cf": {
            "mod.rs": "",
            "model.rs": "",
            "evaluator.rs": "",
            "coefficient.rs": "",
            "lambert.rs": "",
        },

        "function": {
            "mod.rs": "",
            "exp.rs": "",
            "ln.rs": "",
            "common.rs": "",
        },

        "error": {
            "mod.rs": "",
            "model.rs": "",
        },

        "traits": {
            "mod.rs": "",
            "evaluate.rs": "",
            "function.rs": "",
        },

        "prelude.rs": "",
    },

    "tests": {},

    "examples": {},

    "benches": {},
}


def create(tree: dict, root: Path):
    """
    Recursively create project tree.
    """

    for name, value in tree.items():

        path = root / name

        if isinstance(value, dict):

            if not path.exists():
                path.mkdir(parents=True)
                print(f"[DIR ] {path}")

            create(value, path)

        else:

            if path.exists():
                print(f"[SKIP] {path}")
                continue

            path.write_text(value, encoding="utf-8")

            print(f"[FILE] {path}")


def main():

    create(PROJECT, Path("."))

    print()
    print("Bootstrap complete.")


if __name__ == "__main__":
    main()