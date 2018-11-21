# osuparse - A osu! beatmap parser crate

osuparse is a Rust crate for parsing osu! .osu beatmap files. Usage is as simple as:

```rust
let mut file = File::open("map.osu").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();

parse_beatmap(contents.as_str()).unwrap();
```

osuparse fully supports the [osu! beatmap file specification](https://osu.ppy.sh/help/wiki/osu!_File_Formats/Osu_(file_format)),
with the important exception of the Events section, which is pending
implmentation.

