use crate::osu::importer::key_value;
use crate::osu::settings::{Color, Events};
use std::collections::BTreeMap;

pub fn get_events(_section: &Vec<String>) -> Events {
    Events {}
}

pub fn get_colors(section: &Vec<String>) -> Vec<Color> {
    if section.len() == 0 {
        return vec![];
    }

    let mut section_data: BTreeMap<String, String> = BTreeMap::new();
    key_value::get_key_value(section, &mut section_data);

    let mut colors: Vec<Color> = vec![];

    for (_, value) in section_data {
        let split: Vec<&str> = value.split(",").collect();
        if split.len() != 3 {
            println!("Error: invalid color value");
            continue;
        }

        let r = split[0].parse().unwrap_or_else(|_| 255);
        let g = split[1].parse().unwrap_or_else(|_| 255);
        let b = split[2].parse().unwrap_or_else(|_| 255);

        colors.push(Color(r, g, b));
    }

    return colors;
}