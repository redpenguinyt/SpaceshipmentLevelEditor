use std::{
    fs::File,
    io::{BufWriter, Read, Write as ioWrite},
    path::Path,
};

use super::{LevelData, Planet, Player, Target, Vec2F, Wall};

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

pub fn save_level(filepath: &str, level_data: &LevelData) -> Result<(), String> {
    let mut buffer = BufWriter::new(File::create(filepath).map_err(|e| e.to_string())?);

    let level_bytes: Vec<u8> = level_data.to_string().bytes().collect();

    buffer.write_all(&level_bytes).map_err(|e| e.to_string())?;
    buffer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_level(filepath: &str) -> Result<LevelData, String> {
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
        .map(str::parse::<f64>);

    let mut nums = Vec::new();
    for num in r_nums {
        nums.push(num.map_err(|_| format!("Found non-number in {filepath}"))?);
    }

    nums.reverse();

    let level_position = (pop_or_eof(&mut nums)? as i32, pop_or_eof(&mut nums)? as i32);

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

    Ok(LevelData {
        level_position,
        player,
        target,
        planets,
        walls,
    })
}

fn pop_or_eof(nums: &mut Vec<f64>) -> Result<f64, String> {
    nums.pop()
        .map_or_else(|| Err(String::from("Reached end of file early")), Ok)
}
