use std::{
    fmt::{Error, Write as fmtWrite},
    fs::{self, File},
    io::{BufWriter, Read, Write as ioWrite},
    path::{Path, PathBuf},
};

use super::{simulation::Wall, Planet, Player, Target, Vec2F};

const EOF_ERROR: &str = "Reached end of file early";

pub enum SaveMethod {
    ToCurrentFile,
    Incremental,
    As(String),
}

pub fn generate_new_level_path(old_path: &str) -> String {
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
        .expect("Captured number from filename could not be parsed")
        + 1;

    let new_level_path = format!(
        "{}{:0>3}.obl",
        reversed_filename.iter().collect::<String>(),
        new_level_num
    );

    if Path::new(&new_level_path).exists() {
        generate_new_level_path(&new_level_path)
    } else {
        new_level_path
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

pub fn load_level(filepath: &str) -> (Player, Target, Vec<Planet>, Vec<Wall>) {
    let mut file = File::open(filepath).expect("Could not load file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Could not read file");

    let binding = text.replace('\n', " ");
    let mut nums = binding
        .split(' ')
        .filter(|s| !s.is_empty())
        .skip(2)
        .map(str::parse::<f64>)
        .map(|r| r.expect("Could not parse to f64"));

    (
        Player::new(Vec2F::new(
            nums.next().expect(EOF_ERROR),
            nums.next().expect(EOF_ERROR),
        )),
        Target::from_nums(&[
            nums.next().expect(EOF_ERROR),
            nums.next().expect(EOF_ERROR),
            nums.next().expect(EOF_ERROR),
        ]),
        (0..nums.next().expect(EOF_ERROR) as usize)
            .map(|_| {
                Planet::from_nums(&[
                    nums.next().expect(EOF_ERROR),
                    nums.next().expect(EOF_ERROR),
                    nums.next().expect(EOF_ERROR),
                ])
            })
            .collect(),
        (0..nums.next().expect(EOF_ERROR) as usize)
            .map(|_| {
                Wall::from_nums(&[
                    nums.next().expect(EOF_ERROR),
                    nums.next().expect(EOF_ERROR),
                    nums.next().expect(EOF_ERROR),
                    nums.next().expect(EOF_ERROR),
                ])
            })
            .collect(),
    )
}
