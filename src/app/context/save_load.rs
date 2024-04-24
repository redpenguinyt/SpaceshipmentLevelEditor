use std::{
    ffi::OsStr,
    fmt::{Error, Write as fmtWrite},
    fs::{self, File},
    io::{BufWriter, Read, Write as ioWrite},
    path::{Path, PathBuf},
};

use super::{simulation::Wall, Planet, Player, Target, Vec2F};

pub enum SaveMethod {
    ToCurrentFile,
    Incremental,
    As(String),
}

pub fn generate_new_level_path(old_path: &str) -> Result<String, String> {
    let mut reversed_filename: Vec<char> = old_path.trim_end_matches(".obl").chars().collect();

    let mut number = String::new();

    for c in reversed_filename.clone().into_iter().rev() {
        reversed_filename.pop();

        if c.is_numeric() {
            number.push(c);
            continue;
        }

        reversed_filename.push(c);

        break;
    }

    if number.is_empty() {
        number.push_str("001");
    } else {
        number = number.chars().rev().collect();
    }

    let new_level_num = number
        .parse::<u32>()
        .map_err(|e| e.to_string())
        .map_err(|_| "Captured number from filename could not be parsed")?
        + 1;

    let new_level_path = format!(
        "{}{:0>3}.obl",
        reversed_filename.iter().collect::<String>(),
        new_level_num
    );

    if Path::new(&new_level_path).exists() {
        generate_new_level_path(&new_level_path)
    } else {
        Ok(new_level_path)
    }
}

pub fn save_level(
    filepath: &str,
    player: &Player,
    target: &Target,
    planets: &[Planet],
    walls: &[Wall],
) -> Result<(), String> {
    let mut buffer = BufWriter::new(File::create(filepath).map_err(|e| e.to_string())?);

    let level_data = level_data_to_string(player, target, planets, walls)
        .map_err(|e| format!("Failed to convert level data to obl: {e}"))?;
    let level_bytes: Vec<u8> = level_data.bytes().collect();

    buffer.write_all(&level_bytes).map_err(|e| e.to_string())?;
    buffer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

fn level_data_to_string(
    player: &Player,
    target: &Target,
    planets: &[Planet],
    walls: &[Wall],
) -> Result<String, Error> {
    let mut data = String::from("0 0\n");

    writeln!(&mut data, "{} {}", player.pos.x, player.pos.y)?;
    writeln!(&mut data, "{} {}", target.size.round(), target.pos)?;

    writeln!(&mut data, "{}", planets.len())?;
    for planet in planets {
        writeln!(&mut data, "{} {}", planet.mass.round(), planet.pos)?;
    }

    writeln!(&mut data, "{}", walls.len())?;
    for wall in walls {
        writeln!(&mut data, "{} {}", wall.pos1, wall.pos2)?;
    }

    Ok(data)
}

pub fn get_last_file_in_dir(directory: &str) -> Result<String, String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension() == Some(OsStr::new("obl")))
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

pub fn load_level(filepath: &str) -> Result<(Player, Target, Vec<Planet>, Vec<Wall>), String> {
    let mut file = File::open(filepath).map_err(|_| String::from("Failed to open file"))?;
    let mut text = String::new();
    file.read_to_string(&mut text)
        .map_err(|_| String::from("File is not valid UTF-8"))?;

    let mut clean_text = String::new();
    // Ignore comments
    let mut chars = text.chars();
    loop {
        let Some(c) = chars.next() else {
            break;
        };

        if c == '#' {
            chars.find(|ch| *ch == '\n');
        } else {
            clean_text.push(c);
        }
    }
    clean_text = clean_text.replace('\n', " ");

    let r_nums = clean_text
        .split(' ')
        .filter(|s| !s.is_empty())
        .skip(2)
        .map(str::parse::<f64>);

    let mut nums = Vec::new();
    for num in r_nums {
        nums.push(num.map_err(|_| format!("Found non-number in {filepath}"))?);
    }

    nums.reverse();

    let player = Player::new(Vec2F::new(pop_or_eof(&mut nums)?, pop_or_eof(&mut nums)?));

    let target = Target::from_nums(&[
        pop_or_eof(&mut nums)?,
        pop_or_eof(&mut nums)?,
        pop_or_eof(&mut nums)?,
    ]);

    let mut planets = Vec::new();
    for _ in 0..nums.pop().unwrap_or_default() as usize {
        planets.push(Planet::from_nums(&[
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
        ]));
    }

    let mut walls = Vec::new();
    for _ in 0..nums.pop().unwrap_or_default() as usize {
        walls.push(Wall::from_nums(&[
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
            pop_or_eof(&mut nums)?,
        ]));
    }

    Ok((player, target, planets, walls))
}

fn pop_or_eof(nums: &mut Vec<f64>) -> Result<f64, String> {
    nums.pop()
        .map_or_else(|| Err(String::from("Reached end of file early")), Ok)
}
