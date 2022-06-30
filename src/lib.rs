use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;
use std::io::{prelude::*, BufReader, LineWriter};

mod osu;
use osu::importer;
use osu::exporter;
use osu::{Info, Beatmap};
use osu::sections::{TimingPoints, HitObjects};

pub fn import(filename: String) -> Result<Beatmap, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;

    let info = importer::get_info(&data);
    let timing_points = importer::get_timing_points(&data);
    let hit_objects = importer::get_hit_objects(&data);

    return Ok(Beatmap {
        info,
        timing_points,
        hit_objects,
    })
}

// It looks terrible
pub fn import_info(filename: String) -> Result<Info, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;
    return Ok(importer::get_info(&data));
}

pub fn import_timing_points(filename: String) -> Result<TimingPoints, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;
    return Ok(importer::get_timing_points(&data))
}

pub fn import_hit_objects(filename: String) -> Result<HitObjects, Error> {
    let reader = open_file(&filename)?;
    let data = get_sections(reader)?;
    return Ok(importer::get_hit_objects(&data))
}

pub fn export(path: &str, beatmap: Beatmap) -> Result<(), Error> {
    let file = File::create(path)?;
    let mut writer = LineWriter::new(file);
    exporter::write_to_osu(&mut writer, beatmap);
    writer.flush()?;
    return Ok(())
}

fn get_sections(reader: BufReader<File>) -> Result<BTreeMap<String, Vec<String>>, Error> {
    let mut data: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut current_section: String = String::new();

    for line in reader.lines() {
        let line = line?;

        if line.len() == 0 {
            continue;
        }

        if is_section_line(&line) {
            current_section = line.clone();
            data.insert(line, vec![]);
            continue;
        }

        let section = data.get_mut(&current_section);
        if let Some(section) = section {
            section.push(line);
        }
    }

    return Ok(data);
}

fn is_section_line(line: &String) -> bool {
    let first_char = line.chars().next().unwrap();
    let last_char = line.chars().last().unwrap();

    if first_char == '[' && last_char == ']' {
        return true;
    }

    return false;
}

fn open_file(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

#[cfg(test)]
mod tests {
    #[test]
    fn import_beatmap() {
        let filename = String::from("test_files/beatmap.osu");
        let beatmap = crate::import(filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        assert_eq!(beatmap.info.general.preview_time, -69.0);
        assert_eq!(beatmap.info.difficulty.approach_rate, 6.9 as f32);
        assert_eq!(beatmap.info.general.letter_box_in_breaks, false);
        assert_eq!(beatmap.info.general.samples_match_playback_rate, true);
        assert_eq!(beatmap.timing_points.data[0].time, 999.0)
    }
    #[test]
    fn import_only_hit_objects() {
        let filename = String::from("test_files/refactor.osu");
        let hit_objects = crate::import_hit_objects(filename);

        let hit_objects = match hit_objects {
            Ok(hit_objects) => hit_objects,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        let filename = String::from("test_files/refactor-spinnerz.osu");
        let hit_objects = crate::import_hit_objects(filename);

        let filename = String::from("test_files/refactor-holds.osu");
        let hit_objects = crate::import_hit_objects(filename);
    }
    #[test]
    fn color_test() {
        let filename = String::from("test_files/colortest.osu");
        let beatmap = crate::import(filename);

        let beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };

        let color = beatmap.info.colors.data[0].clone();

        assert_eq!(color.0, 69);
        assert_eq!(color.1, 228);
        assert_eq!(color.2, 13);
    }
    #[test]
    fn open_blank_beatmap() {
        let filename = String::from("test_files/blank.osu");
        let beatmap = crate::import(filename);

        let _beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("|| failed to parse beatmap: {}", e),
        };
    }

    #[test]
    fn import_and_export() {
        let filename = String::from("test_files/help/Suzuyu - Light a Way (lit120) [Future Dreams].osu");
        let now = std::time::Instant::now();
        let beatmap = crate::import(filename);

        let mut beatmap = match beatmap {
            Ok(beatmap) => beatmap,
            Err(e) => panic!("🥶 failed to parse beatmap: {}", e),
        };
        let import_time = now.elapsed().as_millis();
        println!("Imported in {}", import_time);
        beatmap.info.metadata.version = "exported".to_string();
        let result = crate::export("test_files/help/new.osu", beatmap);
        match result {
            Ok(_) => println!("success"),
            Err(e) => panic!("uhhh ummm {e}")
        }
        println!("Exported in {}", now.elapsed().as_millis() - import_time)
    }
}
