# osuparse - An osu! beatmap parser crate

[![Build Status](https://travis-ci.org/eltrufas/osuparse.svg?branch=master)](https://travis-ci.org/eltrufas/osuparse)
[![](http://meritbadge.herokuapp.com/osuparse)](https://crates.io/crates/osuparse)<Paste>
[![](https://img.shields.io/crates/l/osuparse.svg)](https://github.com/eltrufas/osuparse/blob/master/LICENSE)

osuparse is a Rust crate for parsing osu! .osu beatmap files. Usage is as simple as:

```rust
let mut file = File::open("map.osu").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();

parse_beatmap(contents.as_str()).unwrap();
```

Documentation for this crate can be found [here](https://docs.rs/osuparse/0.1.0/osuparse/)

osuparse fully supports the [osu! beatmap file specification](https://osu.ppy.sh/help/wiki/osu!_File_Formats/Osu_(file_format)),
with the important exception of the Events section, which is pending
implmentation.

# License:

This crate is licensed under terms of the GPL-3.0 license, as published by the Free Software Foundation.
Code documentation comments are partially sourced from the [osu!wiki](https://github.com/ppy/osu-wiki),
which is licensed under terms of the [CC-BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/legalcode) license.
