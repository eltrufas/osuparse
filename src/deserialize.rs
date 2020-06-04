use super::*;

/// If a struct has this trait, it means that it represents something that can
/// be parsed from a .osu file.
pub trait Parsable {
    /// Returns a string equal to the struct's representation in a .osu file,
    /// or in other words a string that can be parsed to get this struct.
    fn as_parsed(&self) -> String;
}

impl Parsable for Beatmap {
    fn as_parsed(&self) -> String {
        let hitobjects_string = self
            .hit_objects
            .iter()
            .map(|hitobject| hitobject.as_parsed())
            .collect::<Vec<String>>()
            .join("\n");

        let timing_points_string = self
            .timing_points
            .iter()
            .map(|timing_point| timing_point.as_parsed())
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r#"osu file format v{}

{}

{}

{}

{}

[TimingPoints]
{}

{}

[HitObjects]
{}
"#,
            self.version,
            self.general.as_parsed(),
            self.editor.as_parsed(),
            self.metadata.as_parsed(),
            self.difficulty.as_parsed(),
            timing_points_string,
            self.colours.as_parsed(),
            hitobjects_string
        )
    }
}

impl Parsable for GeneralSection {
    fn as_parsed(&self) -> String {
        format!(
            r#"[General]
AudioFilename: {}
AudioLeadIn: {}
Previewtime: {}
Countdown: {}
SampleSet: {}
StackLeniency: {}
Mode: {}
LetterboxInBreaks: {}
WidescreenStoryboard: {}
StoryFireInFront: {}
SpecialStyle: {}
EpilepsyWarning: {}
UseSkinSprites: {}"#,
            self.audio_filename,
            self.audio_lead_in,
            self.preview_time,
            self.countdown as u8,
            self.sample_set,
            self.stack_leniency,
            self.game_mode as u8,
            self.letterbox_in_breaks as u8,
            self.widescreen_storyboard as u8,
            self.story_fire_in_front as u8,
            self.special_style as u8,
            self.epilepsy_warning as u8,
            self.use_skin_sprites as u8
        )
    }
}

impl Parsable for EditorSection {
    fn as_parsed(&self) -> String {
        let bookmark_string = if !self.bookmarks.is_empty() {
            format!(
                "Bookmarks: {}",
                self.bookmarks
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else {
            "".to_string()
        };

        format!(
            r#"[Editor]
{}
DistanceSpacing: {}
BeatDivisor: {}
GridSize: {}
TimelineZoom: {}"#,
            bookmark_string,
            self.distance_spacing,
            self.beat_divisor,
            self.grid_size,
            self.timeline_zoom
        )
    }
}

impl Parsable for MetadataSection {
    fn as_parsed(&self) -> String {
        let tags_string = self.tags.join(" ");

        format!(
            r#"[Metadata]
Title: {}
TitleUnicode: {}
Artist: {}
ArtistUnicode: {}
Creator: {}
Version: {}
Source: {}
Tags: {}
BeatmapID: {}
BeatmapSetID: {}"#,
            self.title,
            self.title_unicode,
            self.artist,
            self.artist_unicode,
            self.creator,
            self.version,
            self.source,
            tags_string,
            self.beatmap_id,
            self.beatmap_set_id,
        )
    }
}

impl Parsable for DifficultySection {
    fn as_parsed(&self) -> String {
        format!(
            r#"[Difficulty]
HPDrainRate: {}
CircleSize: {}
OverallDifficulty: {}
ApproachRate: {}
SliderMultiplier: {}
SliderTickRate: {}"#,
            self.hp_drain_rate,
            self.circle_size,
            self.overall_difficulty,
            self.approach_rate,
            self.slider_multiplier,
            self.slider_tick_rate,
        )
    }
}

impl Parsable for TimingPoint {
    fn as_parsed(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.offset,
            self.ms_per_beat,
            self.meter,
            self.sample_set,
            self.sample_index,
            self.volume,
            self.inherited as u8,
            self.kiai_mode as u8,
        )
    }
}

impl Parsable for HitObject {
    fn as_parsed(&self) -> String {
        match self {
            HitObject::HitCircle(circle) => circle.as_parsed(),
            HitObject::Slider(slider) => slider.as_parsed(),
            HitObject::Spinner(spinner) => spinner.as_parsed(),
            HitObject::HoldNote(hold_note) => hold_note.as_parsed(),
        }
    }
}

impl Parsable for HitCircle {
    fn as_parsed(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.x,
            self.y,
            self.time,
            get_type(1, self.new_combo, self.color_skip),
            self.hitsound,
            self.extras.as_parsed(),
        )
    }
}

impl Parsable for SliderType {
    fn as_parsed(&self) -> String {
        match self {
            SliderType::Linear => "L".to_string(),
            SliderType::Bezier => "B".to_string(),
            SliderType::Perfect => "P".to_string(),
            SliderType::Catmull => "C".to_string(),
        }
    }
}

impl Parsable for Slider {
    fn as_parsed(&self) -> String {
        let curve_points = self
            .curve_points
            .iter()
            .map(|(x, y)| format!("{}:{}", x, y))
            .collect::<Vec<String>>()
            .join("|");

        let edge_hitsounds = self
            .edge_hitsounds
            .iter()
            .map(|hitsound| hitsound.to_string())
            .collect::<Vec<String>>()
            .join("|");

        let edge_additions = self
            .edge_additions
            .iter()
            .map(|(sample, addition)| format!("{}:{}", sample, addition))
            .collect::<Vec<String>>()
            .join("|");

        format!(
            "{},{},{},{},{},{}|{},{},{},{},{},{}",
            self.x,
            self.y,
            self.time,
            get_type(2, self.new_combo, self.color_skip),
            self.hitsound,
            self.slider_type.as_parsed(),
            curve_points,
            self.repeat,
            self.pixel_length,
            edge_hitsounds,
            edge_additions,
            self.extras.as_parsed(),
        )
    }
}

impl Parsable for Spinner {
    fn as_parsed(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.x,
            self.y,
            self.time,
            get_type(8, self.new_combo, self.color_skip),
            self.hitsound,
            self.end_time,
            self.extras.as_parsed(),
        )
    }
}

impl Parsable for HoldNote {
    fn as_parsed(&self) -> String {
        format!(
            "{},{},{},{},{},{}:{}",
            self.x,
            self.y,
            self.time,
            get_type(128, self.new_combo, self.color_skip),
            self.hitsound,
            self.end_time,
            self.extras.as_parsed(),
        )
    }
}

impl Parsable for HitObjectExtras {
    fn as_parsed(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.sample_set,
            self.addition_set,
            self.custom_index,
            self.sample_volume,
            self.filename,
        )
    }
}

impl Parsable for Colour {
    fn as_parsed(&self) -> String {
        format!("{},{},{}", self.0, self.1, self.2)
    }
}

impl Parsable for ColoursSection {
    fn as_parsed(&self) -> String {
        let colours = self
            .colours
            .iter()
            .enumerate()
            .map(|(i, colour)| format!("Combo{} : {}", i + 1, colour.as_parsed()))
            .collect::<Vec<String>>()
            .join("\n");

        format!("[Colours]\n{}", colours)
    }
}

/// Helper function htat, given a base number of either 1, 2, 4, or 128,
/// returns the `type` bitmap for hitobjects.
fn get_type(base: u8, new_combo: bool, color_skip: i32) -> u8 {
    debug_assert_eq!(base & 0b0111_0100, 0);
    let mut ret = base;
    if new_combo {
        ret |= 0b0000_0100;
    }
    ret += (color_skip as u8) << 4;
    return ret;
}
