use std::{
    fs::{self, File},
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use super::{Planet, Player, Target, Vec2F};

pub fn generate_new_level_path(old_path: &str) -> Result<String, String> {
    let prev_level_num: u32 = old_path
        .split('-')
        .nth(1)
        .ok_or_else(|| String::from("Filepath doesn't match pattern level-<num>.obl"))?
        .split('.')
        .next()
        .ok_or_else(|| String::from("Filepath doesn't match pattern level-<num>.obl"))?
        .parse::<u32>()
        .map_err(|e| e.to_string())?;

    Ok(format!("levels/level-{}.obl", prev_level_num + 1))
}

pub fn save_level(
    filepath: &str,
    player: &Player,
    target: &Target,
    planets: &[Planet],
) -> Result<(), String> {
    let mut buffer = BufWriter::new(File::create(filepath).map_err(|e| e.to_string())?);

    let level_data = level_data_to_string(player, target, planets);
    let level_bytes: Vec<u8> = level_data.bytes().collect();

    buffer.write_all(&level_bytes).map_err(|e| e.to_string())?;
    buffer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

fn level_data_to_string(player: &Player, target: &Target, planets: &[Planet]) -> String {
    let mut data = String::from("0 0\n");

    data.push_str(&format!("{} {}\n", player.pos.x, player.pos.y));
    data.push_str(&format!(
        "{} {} {}\n",
        target.size.round(),
        target.pos.x,
        target.pos.y
    ));

    data.push_str(&format!("{}\n", planets.len()));

    for planet in planets {
        data.push_str(&format!(
            "{} {} {}\n",
            planet.mass.round(),
            planet.pos.x,
            planet.pos.y
        ));
    }

    data
}

pub fn get_last_file_in_dir(directory: &str) -> Result<String, String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    entries.sort();

    let Some(last) = entries.last() else {
        return Err(String::from("Could not find any files in directory"));
    };

    let Some(filepath) = last.to_str() else {
        return Err(String::from("Filepath is not valid Unicode"));
    };

    Ok(String::from(filepath))
}

pub fn load_level(filepath: &str) -> (Player, Target, Vec<Planet>) {
    let mut file = File::open(filepath).expect("Could not load file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Could not read file");

    let nums: Vec<f64> = text
        .replace('\n', " ")
        .split(' ')
        .filter(|s| !s.is_empty())
        .skip(2)
        .map(str::parse::<f64>)
        .map(|r| r.expect("Could not parse to f64"))
        .collect();

    (
        Player::new(Vec2F::new(nums[0], nums[1])),
        Target::from_nums(&nums[2..5]),
        (0..nums[5] as usize)
            .map(|i| Planet::from_nums(&nums[(i * 3 + 6)..(i * 3 + 9)]))
            .collect(),
    )
}
