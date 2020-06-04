extern crate cpython;
extern crate osuparse;
extern crate rayon;

use rayon::prelude::*;

use cpython::*;
use cpython::{PyDict, PyResult, Python};
use osuparse::*;
use std::fs::File;
use std::io::prelude::*;

macro_rules! section_builder {
    ($name:ident -> $type:ty
        { $($field:ident),*}
        $({$($special_field:ident: $func:ident),*})*) => {
        fn $name(py: Python, section: $type) -> PyResult<PyDict> {
            let dict = PyDict::new(py);

            $(
                dict.set_item(py, stringify!($field), section.$field)?;
            )*

            $($(
                dict.set_item(
                    py,
                    stringify!($special_field),
                    $func(py, section.$special_field)?
                )?;
            )*)*


            Ok(dict.to_py_object(py))
        }
    };
}

macro_rules! list_builder {
    ($name:ident, $T:ty, $mapper:ident) => {
        fn $name(py: Python, list: Vec<$T>) -> PyResult<PyList> {
            let result = list
                .into_iter()
                .map(|p| $mapper(py, p))
                .collect::<PyResult<Vec<PyDict>>>()
                .map(|v| v.to_py_object(py));
            result
        }
    };
}

fn build_game_mode(py: Python, mode: GameMode) -> PyResult<PyString> {
    Ok(match mode {
        GameMode::Osu => "osu".to_py_object(py),
        GameMode::Taiko => "taiko".to_py_object(py),
        GameMode::CTB => "ctb".to_py_object(py),
        GameMode::Mania => "mania".to_py_object(py),
    })
}

fn build_slider_type(py: Python, slider_type: SliderType) -> PyResult<PyString> {
    Ok(match slider_type {
        SliderType::Linear => "linear".to_py_object(py),
        SliderType::Bezier => "bezier".to_py_object(py),
        SliderType::Perfect => "perfect".to_py_object(py),
        SliderType::Catmull => "catmull".to_py_object(py),
    })
}

fn build_hit_object(py: Python, obj: HitObject) -> PyResult<PyDict> {
    match obj {
        HitObject::HitCircle(c) => Ok(("hit_circle", build_hitcircle(py, c))),
        HitObject::Slider(s) => Ok(("slider", build_slider(py, s))),
        HitObject::Spinner(s) => Ok(("spinner", build_spinner(py, s))),
        HitObject::HoldNote(n) => Ok(("hold_note", build_hold_note(py, n))),
    }
    .and_then(|(t, r)| {
        r.and_then(|d| {
            d.set_item(py, "type", t)?;
            Ok(d)
        })
    })
}

section_builder![build_editor_section -> EditorSection {
   bookmarks, distance_spacing, beat_divisor, grid_size, timeline_zoom
}];

section_builder![build_metadata_section -> MetadataSection {
    title, title_unicode, artist, artist_unicode, creator, version, source,
    tags, beatmap_id, beatmap_set_id
}];

section_builder![build_general_section -> GeneralSection {
    audio_filename, audio_lead_in, preview_time, countdown, sample_set,
    stack_leniency, letterbox_in_breaks, widescreen_storyboard,
    story_fire_in_front, special_style, epilepsy_warning, use_skin_sprites
} {
   game_mode: build_game_mode 
}];

section_builder![build_difficulty_section -> DifficultySection {
    hp_drain_rate, circle_size, overall_difficulty, approach_rate,
    slider_multiplier, slider_tick_rate
}];

section_builder![build_timing_point -> TimingPoint {
    offset, ms_per_beat, meter, sample_set, sample_index,
    volume, inherited, kiai_mode
}];

section_builder![build_extras -> HitObjectExtras {
    sample_set, addition_set, custom_index, sample_volume, filename
}];

section_builder![build_hitcircle -> HitCircle {
    x, y, new_combo, color_skip, time, hitsound
} {
    extras: build_extras
}];

section_builder![build_hold_note -> HoldNote {
    x, y, new_combo, color_skip, time, hitsound, end_time
} {
    extras: build_extras
}];

section_builder![build_spinner -> Spinner {
    x, y, new_combo, color_skip, time, hitsound, end_time
} {
    extras: build_extras
}];

section_builder![build_slider -> Slider {
    x, y, new_combo, color_skip, time, hitsound,
    curve_points, repeat, pixel_length, edge_hitsounds,
    edge_additions
} {
    extras: build_extras,
    slider_type: build_slider_type
}];

list_builder![build_timing_points, TimingPoint, build_timing_point];
list_builder![build_hit_objects, HitObject, build_hit_object];

section_builder![build_beatmap -> Beatmap {
    version
} {
    general: build_general_section,
    editor: build_editor_section,
    metadata: build_metadata_section,
    difficulty: build_difficulty_section,
    timing_points: build_timing_points,
    hit_objects: build_hit_objects
}];

// add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(osuparse, initosuparse, PyInit_osuparse, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "parse_beatmap",
        py_fn!(py, parse_beatmap_py(filename: String)),
    )?;
    m.add(
        py,
        "parse_beatmaps",
        py_fn!(py, parse_beatmaps_py(filenames: Vec<String>)),
    )?;
    Ok(())
});

enum Error {
    Parse(osuparse::Error),
    IO(std::io::Error),
}

impl From<osuparse::Error> for Error {
    fn from(err: osuparse::Error) -> Error {
        Error::Parse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

fn make_pyerr(py: Python, err: Error) -> PyErr {
    match err {
        Error::Parse(parse_err) => {
            PyErr::new::<exc::ValueError, _>(
                py,
                parse_err.to_string()
            )
        },
        Error::IO(io_err) => {
            PyErr::new::<exc::IOError, _>(
                py,
                io_err.to_string()
            )
        },
    }
}

/*
fn into(self) -> PyResult<T> {
    self.map_err(|err| {
        let py = Python::acquire_gil();
        match err {
            Error::Parse(py, parse_err) => {
                PyErr::new::<exc::ValueError, _>(
                    py,
                    parse_err.to_string()
                )
            },
            Error::IO(py, io_err) => {
                PyErr::new::<exc::IOError, _>(
                    py,
                    io_err.to_string()
                )
            },
        }
    })
}*/

fn read_beatmap_from_file(filename: &str) -> Result<Beatmap, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_beatmap(&contents).map_err(|e| e.into())
}

fn parse_beatmaps_py(py: Python, filenames: Vec<String>) -> PyResult<PyList> {
    let maps: Result<Vec<Beatmap>, Error> = py.allow_threads(move || {
        filenames
            .par_iter()
            .map(|f| read_beatmap_from_file(f))
            .collect()
    });

    maps.map_err(|e| make_pyerr(py, e))
        .and_then(|v: Vec<Beatmap>| {
            let maps: PyResult<Vec<PyDict>> =
                v.into_iter().map(|map| build_beatmap(py, map)).collect();
            maps.map(|v| v.to_py_object(py))
        })
}

fn parse_beatmap_py(py: Python, filename: String) -> PyResult<PyDict> {
    read_beatmap_from_file(&filename)
        .map_err(|e| make_pyerr(py, e))
        .and_then(|map| build_beatmap(py, map))
}
