[![Crates.io](https://img.shields.io/crates/v/unbytify.svg)](https://crates.io/crates/unbytify)
[![Docs.rs](https://docs.rs/unbytify/badge.svg)](https://docs.rs/unbytify)
[![Linux Build Status](https://travis-ci.org/niamster/unbytify.svg?branch=master)](https://travis-ci.org/niamster/unbytify)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/ljd6pkh8fsx7oh7a/branch/master?svg=true)](https://ci.appveyor.com/project/niamster/unbytify)
[![Codecov](https://codecov.io/gh/niamster/unbytify/branch/master/graph/badge.svg)](https://codecov.io/gh/niamster/unbytify)
[![Coveralls](https://coveralls.io/repos/github/niamster/unbytify/badge.svg?branch=master)](https://coveralls.io/github/niamster/unbytify?branch=master)
[![License](https://img.shields.io/crates/l/unbytify.svg)](https://opensource.org/licenses/Apache-2.0)


# Unbytify - [Rust][rust] library to parse and represent digital units

### Table of Contents

* [Introduction](#introduction)
* [`unbytify` crate in your project](#in-your-project)
* [Usage](#usage)
* [License](#license)
* [Credits](#credits)

### Introduction

Unbytify converts KiB, MiB, etc. into integer and the other way around.

Sometimes people call this humanization.

### In your project

In Cargo.toml:

```toml
[dependencies]
unbytify = "0.2"
```

### Usage

In your `main.rs`:

```rust
extern crate unbytify;
use unbytify::*;

fn main() {
    assert_eq!(unbytify("1.5K"), Ok(1024 + 512));
    assert_eq!(bytify(1024 + 512), (1.5, "KiB"));
}
```

### Documentation

Most of the useful documentation can be gotten using rustdoc.

Check it out on [docs.rs/unbytify](https://docs.rs/unbytify).

### License
Unbytify project is licensed under Apache-2.0 license.

[rust]: http://rust-lang.org
