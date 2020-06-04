use std;

use super::*;
use error::{Error, Result};

pub struct ParseState<'a> {
    lines: Box<dyn Iterator<Item=(usize, &'a str)> + 'a>,
    // lines: std::iter::Filter<std::str::Lines<'a>, fn(&&str) -> bool>,
    current_line: Option<(usize, &'a str)>,
}

impl<'a> ParseState<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut ps = ParseState {
            lines: Box::new(input.lines().enumerate()
                            .filter(|(_, l)| !l.trim().is_empty())),
            current_line: None,
        };

        ps.read_next_line();

        ps
    }
    pub fn get_current_line(&self) -> Option<&'a str> {
        self.current_line.map(|(_, l)| l)
    }

    pub fn read_next_line(&mut self) -> Option<&'a str> {
        let next_line = self.lines.next();
        self.current_line = next_line;

        next_line.map(|(_, l)| l)
    }

    pub fn syntax_error(&self, reason: &str) -> Error {
        let line = self.current_line.map(|(i, l)| (i, String::from(l)));
        Error::Syntax(line, String::from(reason))
    }

    pub fn wrap_syntax_error<T>(&self, res: Result<T>) -> Result<T> {
        res.map_err(|err| {
            match err {
                Error::Message(m) => self.syntax_error(&m),
                _ => err,
            }
        })
    }
}

/// Get the next item of given iterator, and convert it to the correct
/// type using the given function.
macro_rules! read_val {
    ($iter:ident, $func:expr) => {
        $iter.next().ok_or(Error::Parse).and_then($func)
    };
}

/// Get the next item of given iterator (presumably of type string),
/// split using given seperator, and convert each split string to the
/// correct type using given function.
macro_rules! read_list {
    ($sep:expr, $iter:ident, $func:expr) => {
        $iter
            .next()
            .ok_or(Error::Parse)
            .and_then(|s| s.split($sep).map($func).collect())
    };
}

macro_rules! parse_into_struct {
	($sep:expr, $dest:ident, $line:expr; {$($field:ident: $f:expr),*}) => {
		{
			let mut iter = $line.split($sep).map(|s| s.trim());
			$dest {
				$($field: {
                    iter.next()
						.ok_or_else(|| Error::Message("Unable to parse line"))
                        .and_then($f)?
				}),*
			}
		}
	};
}

macro_rules! value_parser {
    ($v:expr, $fn:expr) => {
        $fn($v)
    };
    ($v:expr, $fn:expr, $sep:expr) => {
        $v.split($sep)
            .map(|s| $fn(s.trim()))
            .collect::<std::result::Result<Vec<_>, _>>()
    };
}

/// Parse key-value pair.
pub fn parse_kv_pair<'a>(state: &'a ParseState) -> Option<(&'a str, &'a str)> {
    state
        .get_current_line()
        .and_then(|l| {
            let mut iter = l.splitn(2, ":");
            iter.next().and_then(|left| iter.next().map(|right| (left.trim(), right.trim())))
        })
}

macro_rules! parse_kv_section {
    (|$s_t:ty, $state:ident| {$($str:expr => $field:ident: $($f:expr),*;)*}) => {
        {
            let mut section: $s_t = Default::default();

            loop {
                $state.read_next_line();
                match parse_kv_pair($state) {
                    $(
                    Some((k, v)) if unicase::eq(k, $str) => {
                        section.$field = $state
                            .wrap_syntax_error(value_parser!(v, $($f),*))?
                    },
                    )*
                    Some(_) => {},
                    _ => break,
                }
            }

            section
        }
    }
}

pub fn parse_num<T: std::str::FromStr>(n: &str) -> Result<T> {
    n.parse()
        .map_err(|_| Error::Message("Unable to parse number"))
}

pub fn parse_string(s: &str) -> Result<String> {
    Ok(String::from(s))
}

pub fn parse_bool(s: &str) -> Result<bool> {
    s.parse::<i32>()
        .map(|n| n != 0)
        .map_err(|_| Error::Message("Could not parse bool"))
}

pub fn parse_mode(s: &str) -> Result<GameMode> {
    match s {
        "0" => Ok(GameMode::Osu),
        "1" => Ok(GameMode::Taiko),
        "2" => Ok(GameMode::CTB),
        "3" => Ok(GameMode::Mania),
        _ => Err(Error::Message("Unable to parse gamemode")),
    }
}

pub fn parse_colour(s: &str) -> Result<Colour> {
    let mut iter = s.split(",");
    Ok(Colour(
        read_val!(iter, parse_num)?,
        read_val!(iter, parse_num)?,
        read_val!(iter, parse_num)?,
    ))
}

pub fn parse_extras(s: &str) -> Result<HitObjectExtras> {
    Ok(parse_into_struct!(":", HitObjectExtras, s; {
        sample_set: parse_num,
        addition_set: parse_num,
        custom_index: parse_num,
        sample_volume: parse_num,
        filename: parse_string
    }))
}

pub fn parse_slider_type(s: &str) -> Result<SliderType> {
    match s {
        "L" => Ok(SliderType::Linear),
        "B" => Ok(SliderType::Bezier),
        "P" => Ok(SliderType::Perfect),
        "C" => Ok(SliderType::Catmull),
        _ => Err(Error::Message("Invalid slider type")),
    }
}

pub fn parse_coord(s: &str) -> Result<(i32, i32)> {
    let mut iter = s.split(":");
    Ok((read_val!(iter, parse_num)?, read_val!(iter, parse_num)?))
}

fn parse_curve_points(s: &str) -> Result<(SliderType, Vec<(i32, i32)>)> {
    let mut iter = s.split("|");

    let slider_type = read_val!(iter, parse_slider_type)?;

    let points = iter.map(parse_coord).collect::<Result<Vec<(i32, i32)>>>()?;

    Ok((slider_type, points))
}

pub fn parse_hit_object(s: &str) -> Result<HitObject> {
    let mut iter = s.split(",");

    let x: i32 = read_val!(iter, parse_num)?;
    let y: i32 = read_val!(iter, parse_num)?;
    let time: i32 = read_val!(iter, parse_num)?;
    let obj_type: i32 = read_val!(iter, parse_num)?;

    let new_combo = obj_type & 4 != 0;
    let color_skip = (obj_type >> 4) & 7;

    let hitsound = read_val!(iter, parse_num)?;

    match obj_type & 139 {
        1 => Ok(HitObject::HitCircle(HitCircle {
            x,
            y,
            new_combo,
            color_skip,
            time,
            hitsound,

            extras: read_val!(iter, parse_extras).unwrap_or(Default::default()),
        })),

        2 => {
            let (slider_type, curve_points) = read_val!(iter, parse_curve_points)?;
            Ok(HitObject::Slider(Slider {
                x,
                y,
                new_combo,
                color_skip,
                time,
                hitsound,
                slider_type,
                curve_points,

                repeat: read_val!(iter, parse_num)?,
                pixel_length: read_val!(iter, parse_num)?,

                edge_hitsounds: read_list!("|", iter, parse_num).unwrap_or(Vec::new()),

                edge_additions: read_list!("|", iter, parse_coord).unwrap_or(Vec::new()),

                extras: read_val!(iter, parse_extras).unwrap_or(Default::default()),
            }))
        }

        8 => Ok(HitObject::Spinner(Spinner {
            x,
            y,
            time,
            new_combo,
            color_skip,
            hitsound,

            end_time: read_val!(iter, parse_num)?,

            extras: read_val!(iter, parse_extras).unwrap_or(Default::default()),
        })),

        128 => {
            let mut obj = HoldNote {
                x,
                y,
                time,
                new_combo,
                color_skip,
                hitsound,

                ..Default::default()
            };

            let (end_time, extras) = iter.next()
                .and_then(|s| {
                    let mut iter = s.splitn(2, ':');
                    iter.next().and_then(|et| {
                        iter.next().map(|ex| (et, ex))
                    })
                })
                .ok_or_else(|| Error::Message("Could not read object extras"))
                .and_then(|(et, ex)| {
                    let et: i32 = parse_num(et)?;
                    let ex = parse_extras(ex)?;

                    return Ok((et, ex))
                })?;

            obj.end_time = end_time;
            obj.extras = extras;

            Ok(HitObject::HoldNote(obj))
        },

        _ => Err(Error::Message("Invalid hit object type")),
    }
}
