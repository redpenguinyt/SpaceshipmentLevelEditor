use std::path::Path;

pub fn generate(old_path: &str) -> Result<String, String> {
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
        generate(&new_level_path)
    } else {
        Ok(new_level_path)
    }
}