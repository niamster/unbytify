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

```
[dependencies]
unbytify = "0.1"
```

### Usage

In your `main.rs`:

```
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
