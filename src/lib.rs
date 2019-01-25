extern crate unicase;

use error::Result;
pub use error::Error;

#[macro_use]
mod parse;
mod error;

use parse::*;

/// Represents an osu! beatmap file. Includes information specified in
/// the [specification](https://osu.ppy.sh/help/wiki/osu!_File_Formats/Osu_(file_format)).
///
/// __NOTE:__ This is missing the Event section, as parsing for this has yet to be
/// implemented in this crate.
#[derive(Default)]
pub struct Beatmap {
    /// The version of the .osu file format.
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
    /// Is number of milliseconds before the audio file should begin playing.
    /// Useful for audio files that begin immediately.
    pub audio_lead_in: i32,
    /// Is the number of milliseconds before the audio file should begin
    /// playing when selected in the song selection menu.
    pub preview_time: i32,
    /// Whether or not a countdown should occur before the first hit object
    /// appears.
    pub countdown: bool,
    /// Specifies which set of hit sounds will be used throughout the beatmap.
    /// Unlike the `sample_set` field in [`HitObjectExtras`](struct.HitObjectExtras.html),
    /// this value is a string.
    pub sample_set: String,
    pub stack_leniency: f32,
    pub countdown_offset: i32,
    pub skin_preference: String,
    pub game_mode: GameMode,
    pub letterbox_in_breaks: bool,
    pub widescreen_storyboard: bool,
    /// Whether or not display the storyboard in front of combo fire.
    pub story_fire_in_front: bool,
    /// Use special Style (N+1 style) for osu!mania.
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
    /// Is the title of the song limited to ASCII characters, e.g. `Yoru Naku Usagi wa Yume o Miru`.
    pub title: String,
    /// Is the title of the song with unicode support, e.g. `夜啼く兎は夢を見る`.
    pub title_unicode: String,
    /// Is the name of the song's artist limited to ASCII characters, e.g. `MISATO`
    pub artist: String,
    /// Is the name of the song's artist with unicode support, e.g. `美里`
    pub artist_unicode: String,
    /// Username of the maker of the beatmap, e.g. `Sotarks`
    pub creator: String,
    /// The name of the beatmap's difficulty, e.g. `Hard`
    pub version: String,
    /// The origin of the song with unicode support, e.g. [`東方Project`](https://en.wikipedia.org/wiki/Touhou_Project)
    pub source: String,
    pub tags: Vec<String>,
    /// Is the ID of the single beatmap.
    pub beatmap_id: i32,
    /// Is the ID of the beatmap set that the beatmap belongs to.
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
pub struct DifficultySection {
    pub hp_drain_rate: f32,
    /// Defines the size of the hit objects in the osu!standard mode.
    ///
    /// The radius in osu!pixels is defined by the formula
    /// `32 * (1 - 0.7 * (CircleSize - 5) / 5)`, alternatively written as
    /// `54.4 - 4.48 * CircleSize`.
    ///
    /// The value of CircleSize for ranked beatmaps must stand at from 2 to 7,
    /// inclusive.
    ///
    /// In osu!mania mode, CircleSize is the number of columns.
    pub circle_size: f32,
    /// Is the harshness of the hit window and the difficulty of spinners.
    pub overall_difficulty: f32,
    /// Defines when hit objects start to fade in relatively to when they
    /// should be hit.
    pub approach_rate: f32,
    /// Specifies the multiplier of the slider velocity. The velocity at slider
    /// multiplier = 1 is 100 osu!pixels per beat. A slider multiplier of `2`
    /// would yield a velocity of 200 osu!pixels per beat. The default slider
    /// multiplier is 1.4 when the property is omitted.
    pub slider_multiplier: f32,
    /// The number of ticks per beat. The default value is 1 tick per beat.
    pub slider_tick_rate: f32,
}

impl Default for DifficultySection {
    /// Create a [DifficultySection](structs.DifficultySection.html) where
    /// all fields are set to 5 except `slider_multiplier` and `slider_tickrate`,
    /// as [is the case in the editor](https://github.com/ppy/osu/blame/f517f98ae7bb6e3e60a6ed552a89474b9470344e/osu.Game/Beatmaps/BeatmapDifficulty.cs#L13).
    fn default() -> Self {
        DifficultySection {
            hp_drain_rate: 5.0,
            circle_size: 5.0,
            overall_difficulty: 5.0,
            approach_rate: 5.0,
            slider_multiplier: 1.4,
            slider_tick_rate: 1.0,
        }
    }
}


/// Represents a single timing point
pub struct TimingPoint {
    /// Is the number of milliseconds from the start of the song, and defines
    /// when the timing point starts. A timing point ends when the next one
    /// starts. The first timing point starts at 0, disregarding its offset.
    pub offset: f32,
    /// Defines the duration of one beat. It affect the scrolling speed in
    /// osu!taiko or osu!mania, and the slider speed in osu!standard, among
    /// other things.
    ///
    /// When positive, it is faithful to its name. When negative, it is a
    /// percentage of previous non-negative milliseconds per beat. For
    /// instance, 3 consecutive timing points with `500`, `-50`, `-100`
    /// will have a resulting beat duration of half a second, a quarter of
    /// a second, and half a second, respectively.
    pub ms_per_beat: f32,
    /// Defines the number of beats in a [measure](https://en.wikipedia.org/wiki/Bar_(music)).
    pub meter: i32,
    /// Defines the default sample that hit objects inherit if the
    /// [`sample_set`](struct.HitObjectExtras.html#structfield.sample_set) field
    /// in [`HitObjectExtras`](struct.HitObjectExtras.html) is set to `0`.
    pub sample_set: i32,
    /// The default custom index that hit objects inherit if the
    /// [`custom_index`](struct.HitObjectExtras.html#structfield.custom_index)
    /// field in [`HitObjectExtras`](struct.HitObjectExtras.html) is set to `0`.
    pub sample_index: i32,
    /// Is the default volume that hitobjects inherit. It ranges from `0`
    /// to `100`.
    pub volume: i32,
    /// Tells if the timing point can be inherited from. Inherited is
    /// redundant with the milliseconds per beat field. A positive milliseconds
    /// per beat implies inherited is `true`, and a negative one implies it is
    /// `false`.
    pub inherited: bool,
    /// Defines whether or not [Kiai Time](https://osu.ppy.sh/help/wiki/Beatmap_Editor/Kiai_Time)
    /// effects are active.
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
    /// Catmull slider type is depricated, but may still appear in an old map.
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

#[derive(Default)]
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

/// The extras field is optional and define additional parameters related to
/// the hit sound samples.
pub struct HitObjectExtras {
    /// Changes the sample set of the __normal__ hit sound.
    ///
    /// The values for these are:
    /// * 0: Auto. See below.
    /// * 1: Normal.
    /// * 2: Soft.
    /// * 3: Drum.
    ///
    /// When `sample_set` is `0`, its value is inherited from the timing point.
    pub sample_set: i32,
    /// Changes the sample set for the other hit sounds
    /// (whistle, finish, clap). See above.
    pub addition_set: i32,
    /// Is the custom sample set index, e.g. `3` in
    /// `soft-hitnormal3.wav`. The special index `1` doesn't appear in the
    /// filename, for example `normal-hitfinish.wav`.
    /// The special index `0` means it is inherited from the timing point.
    pub custom_index: i32,
    /// Is the volume of the sample, and ranges from
    /// `0` to `100`.
    pub sample_volume: i32,
    /// Names an audio file in the folder to play instead of
    /// sounds from sample sets (see above), relative to the beatmap's
    /// directory.
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
        let section = parse_section(&mut state);
        let section = state.wrap_syntax_error(section);
        match section? {
            Section::General(s) => map.general = s,
            Section::Editor(s) => map.editor = s,
            Section::Metadata(s) => map.metadata = s,
            Section::TimingPoints(s) => map.timing_points = s,
            Section::HitObjects(s) => map.hit_objects = s,
            Section::Difficulty(s) => map.difficulty = s,
            Section::Colours(s) => map.colours = s,
            Section::Events => {}
            Section::None => break,
        }
    }

    Ok(map)
}

fn match_header_line<'a>(line: &'a str) -> Option<&'a str> {
    let line = line.trim_end();
    let mut chars = line.chars();

    chars.next().filter(|c| *c == '[')
        .and(chars.last().filter(|c| *c == ']'))
        .map(|_| &line[1..line.len() - 1])
}

fn parse_section(state: &mut ParseState) -> Result<Section> {
    if let Some(header_line) = state.get_current_line() {
        let section_title = match_header_line(header_line)
            .ok_or_else(|| state.syntax_error("Malformed section header"))?;

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

            "TimingPoints" => parse_timing_points(state).map(|s| Section::TimingPoints(s)),

            "HitObjects" => parse_hit_objects(state).map(|s| Section::HitObjects(s)),

            "Colours" => parse_colours(state).map(|s| Section::Colours(s)),

            _ => Err(state.syntax_error("Unknown section header")),
        }
    } else {
        Ok(Section::None)
    }
}

fn skip_section(state: &mut ParseState) {
    loop {
        match state.read_next_line() {
            Some(l) if match_header_line(l).is_none() => {}
            _ => break,
        }
    }
}

fn parse_version_string(state: &mut ParseState) -> Result<i32> {
    state
        .get_current_line()
        .and_then(|l| l.find("osu file format v").map(|n| (n, l)))
        .and_then(|(n, l)| l[n + 17..].trim_end().parse::<i32>().ok())
        .ok_or_else(|| state.syntax_error("Unable to parse version line"))
}

fn parse_timing_points(state: &mut ParseState) -> Result<Vec<TimingPoint>> {
    let mut timing_points = Vec::with_capacity(100);
    loop {
        match state.read_next_line() {
            Some(l) if match_header_line(l).is_none() => {
                let timing_point = parse_into_struct!(",", TimingPoint, l; {
                    offset: parse_num,
                    ms_per_beat: parse_num,
                    meter: parse_num,
                    sample_set: parse_num,
                    sample_index: parse_num,
                    volume: parse_num,
                    inherited: parse_bool,
                    kiai_mode: parse_bool
                });

                timing_points.push(timing_point)
            }
            _ => break,
        };
    }

    Ok(timing_points)
}

fn parse_colours(state: &mut ParseState) -> Result<ColoursSection> {
    let mut section: ColoursSection = Default::default();

    let mut colours = Vec::with_capacity(10);

    loop {
        state.read_next_line();
        match parse_kv_pair(state) {
            Some((k, v)) if k.starts_with("Combo") => {
                let n: i32 = parse_num(&k[5..])?;
                colours.push((n, parse_colour(v)?));
            }

            Some((k, v)) if unicase::eq("SliderBody", k) => section.slider_body = parse_colour(v)?,

            Some((k, v)) if unicase::eq("SliderTrackOverride", k) => {
                section.slider_track_override = parse_colour(v)?
            }

            Some((k, v)) if unicase::eq("SliderBorder", k) => {
                section.slider_border = parse_colour(v)?
            }

            Some(_) => {},

            _ => break,
        }
    }

    colours.sort_unstable();
    section.colours = colours.into_iter().map(|(_, c)| c).collect();

    Ok(section)
}

fn parse_hit_objects(state: &mut ParseState) -> Result<Vec<HitObject>> {
    let mut hit_objects = Vec::with_capacity(100);

    loop {
        match state.read_next_line() {
            Some(l) if match_header_line(l).is_none() => {
                hit_objects.push(parse_hit_object(l)?);
            }
            _ => break,
        }
    }

    Ok(hit_objects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_parse_version_string() {
        let mut state = ParseState::new(r"osu file format v14");

        let version = parse_version_string(&mut state).unwrap();

        assert_eq!(version, 14)
    }

    #[test]
    fn test_parse_file() {
        let mut file = File::open("test.osu").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        parse_beatmap(contents.as_str()).unwrap();
    }

    #[test]
    fn test_parse_mania_map() {
        let mut file = File::open("omtest.osu").unwrap();
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
        assert_eq!(
            map.general.audio_filename,
            "Bakemonogatari_-_Kimi_no_Shiranai_Monogatari.mp3"
        );
        assert_eq!(map.general.audio_lead_in, 0);
        assert_eq!(map.general.preview_time, 239594);
        assert_eq!(map.general.stack_leniency, 0.7);
        assert_eq!(map.general.sample_set, "Soft");
        assert_eq!(map.editor.bookmarks, vec![5, 6]);
    }
}
