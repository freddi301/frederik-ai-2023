use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::Error;
use std::io::Read;
use std::io::Write;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).expect("missing command");
    let max_offset: usize = args
        .get(2)
        .expect("missing max offset")
        .parse()
        .expect("max offset not an unsigned int");
    match command.as_str() {
        "train" => {
            let file_name = args.get(3).expect("missing file name");
            let text = read_data_file(file_name).expect("could not read file");
            let chars = string_to_char_vector(&text);
            let stats = train(&chars, max_offset);
            save_stats_file(&stats)?;
            //println!("{:#?}", stats);
            return Ok(());
        }
        "predict" => {
            let stats = load_stats_file().expect("could not load stats");
            for text in vec!["ma", "dol", "con", "amo", "amor"] {
                println!(
                    "{} => {}",
                    text,
                    predict_sequence(&text.chars().collect(), 20, &stats, max_offset)
                );
            }
            for starting_character in "abcdefghijklmnopqrstuvwxyz".chars() {
                println!(
                    "{} => {}",
                    starting_character,
                    predict_sequence(&vec![starting_character], 100, &stats, max_offset)
                );
            }
            return Ok(());
        }
        _ => Ok(()),
    }
}

fn train(chars: &Vec<char>, max_offset: usize) -> HashMap<Pattern, PatternStats> {
    let patterns = create_txt_patterns(&chars, max_offset);
    let mut stats: HashMap<Pattern, PatternStats> = HashMap::new();
    scan_text(chars, &patterns, &mut stats);
    return stats;
}

fn read_data_file(file_name: &String) -> Result<String, Error> {
    let mut file = File::open(file_name)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    return Ok(text);
}

fn string_to_char_vector(text: &String) -> Vec<char> {
    return text.chars().collect();
}

fn create_txt_patterns(characters: &Vec<char>, max_offset: usize) -> HashSet<Pattern> {
    let mut patterns = HashSet::new();
    for i in 0..characters.len() {
        if let Some(current_character) = characters.get(i) {
            for offset in 1..max_offset {
                if let Option::Some(next_character) = characters.get(i + offset) {
                    patterns.insert(Pattern {
                        condition: Observation::CharacterAtSlidingPosition(*current_character, 0),
                        consequence: Observation::CharacterAtSlidingPosition(
                            *next_character,
                            offset,
                        ),
                    });
                }
            }
        }
    }
    return patterns;
}

fn scan_text(
    characters: &Vec<char>,
    patterns: &HashSet<Pattern>,
    stats: &mut HashMap<Pattern, PatternStats>,
) {
    let pattern_bar = ProgressBar::new(patterns.len() as u64);
    for pattern in patterns {
        pattern_bar.inc(1);
        let stat = stats.entry(*pattern).or_insert(PatternStats {
            condition_count: 0,
            consequence_count: 0,
        });
        for index in 0..characters.len() {
            if pattern.condition.holds(index, characters) {
                stat.condition_count += 1;
                if pattern.consequence.holds(index, characters) {
                    stat.consequence_count += 1;
                }
            }
        }
    }
}

fn save_stats_file(stats: &HashMap<Pattern, PatternStats>) -> Result<(), Error> {
    let mut file = File::create("stats.json")?;
    let mut json_friendly: Vec<(Pattern, PatternStats)> = Vec::new();
    for (key, value) in stats {
        json_friendly.push((*key, *value));
    }
    json_friendly.sort_by(|(_, a), (_, b)| a.ratio().partial_cmp(&b.ratio()).unwrap());
    let text = serde_json::to_string(&json_friendly)?;
    file.write_all(text.as_bytes())?;
    return Ok(());
}

fn load_stats_file() -> Result<HashMap<Pattern, PatternStats>, Error> {
    let mut file = File::open("stats.json")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let json_friendly: Vec<(Pattern, PatternStats)> =
        serde_json::from_str(&text).expect("stats file wrong format");
    let mut stats: HashMap<Pattern, PatternStats> = HashMap::new();
    for (pattern, stat) in json_friendly {
        stats.insert(pattern, stat);
    }
    return Ok(stats);
}

fn predict(
    sequence: &Vec<char>,
    stats: &HashMap<Pattern, PatternStats>,
    max_offset: usize,
) -> Option<char> {
    let mut score_by_char: HashMap<char, f64> = HashMap::new();
    for (pattern, stat) in stats {
        for offset in 1..max_offset {
            if sequence.len() > (offset - 1) {
                if let Some(previous_character) = sequence.get(sequence.len() - offset) {
                    if pattern.condition
                        == Observation::CharacterAtSlidingPosition(*previous_character, 0)
                    {
                        let Observation::CharacterAtSlidingPosition(next_character, position) =
                            pattern.consequence;
                        if position == offset {
                            *score_by_char.entry(next_character).or_insert(0.0) += stat.ratio();
                        }
                    }
                }
            }
        }
    }
    *score_by_char.entry(' ').or_insert(0.0) *= 0.5;
    if let Some((next_character, _)) = score_by_char
        .iter()
        // .filter(|(character, _)| **character != ' ')
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    {
        return Some(*next_character);
    }
    return None;
}

fn predict_sequence(
    sequence: &Vec<char>,
    max_length: usize,
    stats: &HashMap<Pattern, PatternStats>,
    max_offset: usize,
) -> String {
    let mut result = sequence.clone();
    while let Some(next_character) = predict(&result, stats, max_offset) {
        result.push(next_character);
        if result.len() >= max_length {
            break;
        }
    }
    return result.iter().collect();
}

impl Observation {
    fn holds(&self, index: usize, characters: &Vec<char>) -> bool {
        match self {
            Observation::CharacterAtSlidingPosition(character, position) => {
                if let Some(character_at_position) = characters.get(index + position) {
                    return character == character_at_position;
                }
                return false;
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
struct Pattern {
    condition: Observation,
    consequence: Observation,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
enum Observation {
    CharacterAtSlidingPosition(char, usize),
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
struct PatternStats {
    condition_count: u32,
    consequence_count: u32,
}

impl PatternStats {
    fn ratio(&self) -> f64 {
        return (self.consequence_count as f64) / (self.condition_count as f64);
    }
}
