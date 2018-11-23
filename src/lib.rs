extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate unicase;

use regex::Regex;

use error::{Error, Result};

#[macro_use]
mod parse;
mod error;

use parse::*;

/// Represents an osu! beatmap file. Includes information specified in
/// the [specification](https://osu.ppy.sh/help/wiki/osu!_File_Formats/Osu_(file_format)).
///
/// NOTE: This is missing the Event section, as parsing for this has yet to be
/// implemented in this crate.
#[derive(Default)]
pub struct Beatmap {
    pub version: i32,
    pub general: GeneralSection,
    pub editor: EditorSection,
    pub metadata: MetadataSection,
	pub timing_points: Vec<TimingPoint>,
    pub hit_objects: Vec<HitObject>,
	pub difficulty: DifficultySection,
    pub colours: ColoursSection,
}

/// One of the four currently available osu! gamemodes.
#[derive(Debug)]
pub enum GameMode {
    Osu,
    Taiko,
    CTB,
    Mania,
}

/// General properties of a beatmap.
#[derive(Debug)]
pub struct GeneralSection {
    pub audio_filename: String,
    pub audio_lead_in: i32,
    pub preview_time: i32,
    pub countdown: bool,
    pub sample_set: String,
    pub stack_leniency: f32,
    pub countdown_offset: i32,
    pub skin_preference: String,
    pub game_mode: GameMode,
    pub letterbox_in_breaks: bool,
    pub widescreen_storyboard: bool,
    pub story_fire_in_front: bool,
    pub special_style: bool,
    pub epilepsy_warning: bool,
    pub use_skin_sprites: bool,
}

impl Default for GeneralSection {
    fn default() -> Self {
        GeneralSection {
            audio_filename: String::new(),
            audio_lead_in: 0,
            preview_time: 0,
            countdown: false,
            sample_set: String::new(),
            skin_preference: String::new(),
            stack_leniency: 0.0,
            countdown_offset: 0,
            game_mode: GameMode::Osu,
            letterbox_in_breaks: false,
            widescreen_storyboard: false,
            story_fire_in_front: false,
            special_style: false,
            epilepsy_warning: false,
            use_skin_sprites: false,
        } 
    }
}

/// Properties relating to the beatmap editor state
pub struct EditorSection {
    pub bookmarks: Vec<i32>,
    pub distance_spacing: f32,
    pub beat_divisor: i32,
    pub grid_size: i32,
    pub timeline_zoom: f32,
}

impl Default for EditorSection {
    fn default() -> Self {
        EditorSection {
            bookmarks: Vec::new(),
            distance_spacing: 1.22,
            beat_divisor: 4,
            grid_size: 4,
            timeline_zoom: 1.0,
        } 
    }
}

/// Metadata relating to the beatmap
pub struct MetadataSection {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
}

impl Default for MetadataSection {
    fn default() -> Self {
        MetadataSection {
            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            version: String::new(),
            source: String::new(),
            tags: Vec::new(),
            beatmap_id: 0,
            beatmap_set_id: 0,
        }
    }
}

/// Difficulty modifiers for the beatmap
#[derive(Default)]
pub struct DifficultySection {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,
}

/// Represents a single timing point
pub struct TimingPoint {
	pub offset: f32,
	pub ms_per_beat: f32,
	pub meter: i32,
	pub sample_set: String,
	pub sample_index: i32,
	pub volume: i32,
	pub inherited: bool,
	pub kiai_mode: bool,
}

/// One of the four possible hit objects appearing on an osu! map.
pub enum HitObject {
    HitCircle(HitCircle),
    Slider(Slider),
    Spinner(Spinner),
    HoldNote(HoldNote),
}

pub struct HitCircle {
    pub x: i32,
    pub y: i32,
	pub new_combo: bool,
    pub color_skip: i32,
    pub time: i32,
    pub hitsound: i32,
    pub extras: HitObjectExtras,
}

/// Type of slider curve
pub enum SliderType {
    Linear,
    Bezier,
    Perfect,
    Catmull,
}

pub struct Slider {
    pub x: i32,
    pub y: i32,
	pub new_combo: bool,
    pub color_skip: i32,
    pub time: i32,
    pub slider_type: SliderType,
    pub curve_points: Vec<(i32, i32)>,
    pub repeat: i32,
    pub pixel_length: f32,
    pub edge_hitsounds: Vec<i32>,
    pub edge_additions: Vec<(i32, i32)>,
    pub hitsound: i32,
    pub extras: HitObjectExtras,
}

pub struct Spinner {
    pub x: i32,
    pub y: i32,
	pub new_combo: bool,
    pub color_skip: i32,
    pub time: i32,
    pub hitsound: i32,
    pub end_time: i32,
    pub extras: HitObjectExtras,
}

pub struct HoldNote {
    pub x: i32,
    pub y: i32,
	pub new_combo: bool,
    pub color_skip: i32,
    pub time: i32,
    pub hitsound: i32,
    pub end_time: i32,
    pub extras: HitObjectExtras,
}

pub struct HitObjectExtras {
    pub sample_set: i32,
    pub addition_set: i32,
    pub custom_index: i32,
    pub sample_volume: i32,
    pub filename: String,
}

impl Default for HitObjectExtras {
    fn default() -> Self {
        HitObjectExtras {
            sample_set: 0,
            addition_set: 0,
            custom_index: 0,
            sample_volume: 0,
            filename: String::new(),
        }
    }
}

/// An RGB triplet representing a colour.
#[derive(Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Colour(i32, i32, i32);

/// Includes a beatmap's combo colours as well as slider colour overrides.
#[derive(Default)]
pub struct ColoursSection {
	pub colours: Vec<Colour>,
	pub slider_body: Colour,
	pub slider_track_override: Colour,
	pub slider_border: Colour,
}

enum Section {
    General(GeneralSection),
    Editor(EditorSection),
    Metadata(MetadataSection),
	TimingPoints(Vec<TimingPoint>),
	HitObjects(Vec<HitObject>),
	Difficulty(DifficultySection),
    Colours(ColoursSection),
    Events,
    None,
}

/// Reads input from a string and attempts to output an osu beatmap.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::prelude::*;
///
/// use osuparse::parse_beatmap;
///
/// let mut file = File::open("map.osu").unwrap();
/// let mut contents = String::new();
/// file.read_to_string(&mut contents).unwrap();
///
/// parse_beatmap(contents.as_str()).unwrap();
/// ```
pub fn parse_beatmap(input: &str) -> Result<Beatmap> {
    let mut state = ParseState::new(input); 

    let version = parse_version_string(&mut state)?;
    state.read_next_line();

    let mut map = Beatmap {
        version,
        ..Default::default()
    };

    loop {
        match parse_section(&mut state)? {
            Section::General(s) => map.general = s,
            Section::Editor(s) => map.editor = s,
            Section::Metadata(s) => map.metadata = s,
			Section::TimingPoints(s) => map.timing_points = s,
			Section::HitObjects(s) => map.hit_objects = s,
			Section::Difficulty(s) => map.difficulty = s,
            Section::Colours(s) => map.colours = s,
            Section::Events => {},
            Section::None => break,
        }
     }

    Ok(map)
}

fn parse_section(state: &mut ParseState) -> Result<Section> {
    if let Some(header_line) = state.get_current_line() {
        lazy_static! {
            static ref HEADER_RE: Regex = Regex::new(r"^\[([^\[\]]*)\]\s*$").unwrap();
        }


        let section_title = HEADER_RE.captures(header_line)
            .and_then(|c| c.get(1))
            .map(|c| c.as_str())
            .ok_or_else(|| Error::Syntax(format!("Malformed section header: {}", header_line)))?;

        match section_title {
            "General" => Ok(Section::General(parse_kv_section! {
                |GeneralSection, state| {
                    "AudioFilename" => audio_filename: parse_string;
                    "AudioLeadIn" => audio_lead_in: parse_num;
                    "PreviewTime" => preview_time: parse_num;
                    "Countdown" => countdown: parse_bool;
                    "CountdownOffset" => countdown_offset: parse_num;
                    "SampleSet" => sample_set: parse_string;
                    "SkinPreference" => skin_preference: parse_string;
                    "StackLeniency" => stack_leniency: parse_num;
                    "Mode" => game_mode: parse_mode;
                    "LetterboxInBreaks" => letterbox_in_breaks: parse_bool;
                    "WidescreenStoryboard" => widescreen_storyboard: parse_bool;
                    "EpilepsyWarning" => epilepsy_warning: parse_bool;
                    "StoryFireInFront" => story_fire_in_front: parse_bool;
                    "SpecialStyle" => special_style: parse_bool;
                }
            })),

            "Editor" => Ok(Section::Editor(parse_kv_section! {
                |EditorSection, state| {
                    "Bookmarks" => bookmarks: parse_num, ",";
                    "DistanceSpacing" => distance_spacing: parse_num;
                    "BeatDivisor" => beat_divisor: parse_num;
                    "GridSize" => grid_size: parse_num;
                    "TimelineZoom" => timeline_zoom: parse_num;
                }
            })),
            
            "Metadata" => Ok(Section::Metadata(parse_kv_section! {
                |MetadataSection, state| {
                    "Title" => title: parse_string;
                    "TitleUnicode" => title_unicode: parse_string;
                    "Artist" => artist: parse_string;
                    "ArtistUnicode" => artist_unicode: parse_string;
                    "Creator" => creator: parse_string;
                    "Version" => version: parse_string;
                    "Source" => source: parse_string;
                    "Tags" => tags: parse_string, " ";
                    "BeatmapID" => beatmap_id: parse_num;
                    "BeatmapSetID" => beatmap_set_id: parse_num;
                }
            })),

			"Difficulty" => Ok(Section::Difficulty(parse_kv_section! {
                |DifficultySection, state| {
                    "HPDrainRate" => hp_drain_rate: parse_num;
                    "CircleSize" => circle_size: parse_num;
                    "OverallDifficulty" => overall_difficulty: parse_num;
                    "ApproachRate" => approach_rate: parse_num;
                    "SliderMultiplier" => slider_multiplier: parse_num;
                    "SliderTickRate" => slider_tick_rate: parse_num;
                }
            })),

            "Events" => {
                // Just skipping this for now
                skip_section(state);
                Ok(Section::Events)
            }

			"TimingPoints" =>
                parse_timing_points(state).map(|s| Section::TimingPoints(s)),

			"HitObjects" => parse_hit_objects(state).map(|s| Section::HitObjects(s)),

            "Colours" => parse_colours(state).map(|s| Section::Colours(s)),

            _ => {
				Err(Error::Syntax(format!(
					"Unknown section header {}",
					section_title
				)))
			},
        }
    } else {
        Ok(Section::None)
    }
}

fn skip_section(state: &mut ParseState) {
    lazy_static! {
		static ref HEADER_RE: Regex = Regex::new(r"^\[([^\[\]]*)\]\s*$").unwrap();
	}

	loop {
        match state.read_next_line() {
            Some(l) if !HEADER_RE.is_match(l) => {},
            _ => break,
        }
	}
}

fn parse_version_string(state: &mut ParseState) -> Result<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*osu file format v(\d+)$").unwrap();
    }

    state.get_current_line()
        .and_then(|line| RE.captures(line))
        .and_then(|line| line.get(1))
        .and_then(|ver| ver.as_str().parse::<i32>().ok())
		.ok_or_else(make_syntax_err!("unable to parse version string"))
}

fn parse_timing_points(state: &mut ParseState) -> Result<Vec<TimingPoint>> {
	lazy_static! {
		static ref HEADER_RE: Regex = Regex::new(r"^\[([^\[\]]*)\]\s*$").unwrap();
	}

	let mut timing_points = Vec::with_capacity(100);
	loop {
        match state.read_next_line() {
            Some(l) if !HEADER_RE.is_match(l) => {
                let timing_point = parse_into_struct!(",", TimingPoint, l; {
                    offset: parse_num,
                    ms_per_beat: parse_num,
                    meter: parse_num,
                    sample_set: parse_string,
                    sample_index: parse_num,
                    volume: parse_num,
                    inherited: parse_bool,
                    kiai_mode: parse_bool
                });

                timing_points.push(timing_point)
            },
            _ => break,
        };
	}

	Ok(timing_points)
}

fn parse_colours(state: &mut ParseState) -> Result<ColoursSection> {
	lazy_static! {
		static ref COLOR_RE: Regex = Regex::new(r"^Combo\d+$").unwrap();
	}

	let mut section: ColoursSection = Default::default();

    let mut colours = Vec::with_capacity(10);

    loop {
        match parse_kv_pair(state) {
            Some((k, v)) if COLOR_RE.is_match(k)  => {
                let n: i32 = parse_num(&k[5..])?;
                colours.push((n, parse_colour(v)?));
            }

            Some((k, v)) if unicase::eq("SliderBody", k) => section.slider_body = parse_colour(v)?,

            Some((k, v)) if unicase::eq("SliderTrackOverride", k) => {
                section.slider_track_override = parse_colour(v)?
            },

            Some((k, v))  if unicase::eq("SliderBorder", k) => section.slider_border = parse_colour(v)?,

            Some((k, _)) => {
                return Err(Error::Syntax(format!("Unknown key value: {}", k)))
            },
            
            _ => break,
        }
    }

    colours.sort_unstable();
    section.colours = colours.into_iter().map(|(_, c)| c).collect();

	Ok(section)
}

fn parse_hit_objects(state: &mut ParseState) -> Result<Vec<HitObject>> {
	lazy_static! {
		static ref HEADER_RE: Regex = Regex::new(r"^\[([^\[\]]*)\]\s*$").unwrap();
	}

	let mut hit_objects = Vec::with_capacity(100);

	loop {
        match state.read_next_line() {
            Some(l) if !HEADER_RE.is_match(l) => {
			    hit_objects.push(parse_hit_object(l)?);
            },
            _ => break,
        }
	}

	Ok(hit_objects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn test_parse_version_string() {
        let mut state = ParseState::new(r"osu file format v14");

        let version = parse_version_string(&mut state).unwrap();

        assert_eq!(version, 14)
    }

    #[test]
    fn test_parse_file() {
        let mut file = File::open("test8.osu").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        parse_beatmap(contents.as_str()).unwrap();
    }

    #[test]
    fn test_parse_map() {
        let map = parse_beatmap(r"osu file format v14

[General]
AudioFilename: Bakemonogatari_-_Kimi_no_Shiranai_Monogatari.mp3
AudioLeadIn: 0
PreviewTime: 239594
Countdown: 0
SampleSet: Soft
StackLeniency: 0.7
Mode: 0
LetterboxInBreaks: 1
WidescreenStoryboard: 0

[Editor]
Bookmarks: 5,6
DistanceSpacing: 1
BeatDivisor: 4
GridSize: 4
TimelineZoom: 5.100003

[Metadata]
Title:Kimi no Shiranai Monogatari
TitleUnicode:君の知らない物語
Artist:supercell
ArtistUnicode:supercell
Creator:monstrata
Version:Celestial
Source:化物語
Tags:ed ending Bakemonogatari shaft nagi yanagi ryo araragi senjougahara hanekawa sengoku kanbaru hachikuji shinobu tsukihi karen senjou gahara hitagi koyomi oshino nadeko tsubasa surug
BeatmapID:651744
BeatmapSetID:289074

[TimingPoints]
764,363.636363636364,4,2,1,50,1,0
764,-133.333333333333,4,2,1,50,0,0
3480,363.636363636364,4,2,1,50,1,8
3661,363.636363636364,4,2,1,50,1,0
3661,-133.333333333333,4,2,1,50,0,0
9479,-133.333333333333,4,1,0,50,0,0
12388,-100,4,1,0,50,0,0
17466,363.636363636364,4,1,0,50,1,8
18180,363.636363636364,4,1,0,50,1,0
19651,363.636363636364,4,1,0,50,1,0
24023,363.636363636364,4,1,0,50,1,0
25474,363.636363636364,4,1,1,50,1,0
29837,-142.857142857143,4,2,1,50,0,0
32775,363.636363636364,4,2,1,50,1,0
32775,-142.857142857143,4,2,1,50,0,0

[HitObjects]
47,196,764,6,0,L|38:127,2,63.7500024318696,2|0|0,0:0|0:0|0:0,0:0:0:0:
60,277,1309,2,0,L|196:333,1,127.500004863739,2|0,0:2|0:0,0:0:0:0:
254,357,1854,1,0,0:0:0:0:
319,306,2036,6,0,L|387:334,2,63.7500024318696,0|2|0,0:0|0:0|0:0,0:0:0:0:
242,275,2582,1,0,0:0:0:0:
230,192,2764,2,0,L|207:41,1,127.500004863739,2|0,0:0|0:0,0:0:0:0:
307,223,3480,1,0,0:0:0:0:
242,275,3661,6,0,L|179:325,2,63.7500024318696,2|0|0,0:0|0:0|0:0,0:0:0:0:
307,223,4206,1,0,0:0:0:0:
295,140,4388,2,0,L|357:89,2,63.7500024318696,2|0|0,0:0|0:0|0:0,0:0:0:0:
230,192,4933,1,0,0:0:0:0:
165,244,5115,54,0,L|94:302,1,63.7500024318696,2|0,0:0|0:0,0:0:0:0:
152,161,5479,2,0,L|66:128,1,63.7500024318696
217,108,5842,2,0,L|204:19,1,63.7500024318696,2|0,0:0|0:0,0:0:0:0:

").unwrap();

        assert_eq!(map.version, 14);
        assert_eq!(map.general.audio_filename, "Bakemonogatari_-_Kimi_no_Shiranai_Monogatari.mp3");
        assert_eq!(map.general.audio_lead_in, 0);
        assert_eq!(map.general.preview_time, 239594);
        assert_eq!(map.general.stack_leniency, 0.7);
        assert_eq!(map.general.sample_set, "Soft");
        assert_eq!(map.editor.bookmarks, vec![5, 6]);
    }
}
