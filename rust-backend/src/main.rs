use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("missing file name");
    println!("reading file");
    let text = read_my_file(file_name)?;
    let chars = string_to_char_vector(&text);
    println!("generating patterns");
    let patterns = create_txt_patterns(&chars);
    let mut stats: HashMap<Pattern, PatternStats> = HashMap::new();
    println!("scanning text");
    scan_text(&chars, &patterns, &mut stats);
    //println!("{:#?}", stats);
    println!("writing output");
    write_my_file(&stats)?;
    println!("Predicted: {}", predict_sequence('a', 10, &stats));
    println!("Predicted: {}", predict_sequence('b', 10, &stats));
    println!("Predicted: {}", predict_sequence('c', 10, &stats));
    println!("Predicted: {}", predict_sequence('d', 10, &stats));
    println!("Predicted: {}", predict_sequence('e', 10, &stats));
    println!("Predicted: {}", predict_sequence('f', 10, &stats));
    println!("Predicted: {}", predict_sequence('g', 10, &stats));
    println!("Predicted: {}", predict_sequence('h', 10, &stats));
    println!("Predicted: {}", predict_sequence('i', 10, &stats));
    println!("Predicted: {}", predict_sequence('j', 10, &stats));
    println!("Predicted: {}", predict_sequence('k', 10, &stats));
    println!("Predicted: {}", predict_sequence('l', 10, &stats));
    println!("Predicted: {}", predict_sequence('m', 10, &stats));
    println!("Predicted: {}", predict_sequence('n', 10, &stats));
    println!("Predicted: {}", predict_sequence('o', 10, &stats));
    println!("Predicted: {}", predict_sequence('p', 10, &stats));
    println!("Predicted: {}", predict_sequence('q', 10, &stats));
    println!("Predicted: {}", predict_sequence('r', 10, &stats));
    println!("Predicted: {}", predict_sequence('s', 10, &stats));
    println!("Predicted: {}", predict_sequence('t', 10, &stats));
    println!("Predicted: {}", predict_sequence('u', 10, &stats));
    println!("Predicted: {}", predict_sequence('v', 10, &stats));
    println!("Predicted: {}", predict_sequence('x', 10, &stats));
    println!("Predicted: {}", predict_sequence('y', 10, &stats));
    println!("Predicted: {}", predict_sequence('z', 10, &stats));
    return Ok(());
}

fn read_my_file(file_name: &String) -> Result<String, Error> {
    let mut file = File::open(file_name)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    return Ok(text);
}

fn string_to_char_vector(text: &String) -> Vec<char> {
    return text.chars().collect();
}

fn create_txt_patterns(characters: &Vec<char>) -> HashSet<Pattern> {
    let mut patterns = HashSet::new();
    for i in 0..characters.len() {
        if let Some(current_character) = characters.get(i) {
            if let Option::Some(next_character) = characters.get(i + 1) {
                patterns.insert(Pattern {
                    condition: Observation::CharacterAtSlidingPosition(*current_character, 0),
                    consequence: Observation::CharacterAtSlidingPosition(*next_character, 1),
                });
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
        for index in 0..characters.len() {
            if pattern.condition.holds(index, characters) {
                let stat = stats.entry(*pattern).or_insert(PatternStats {
                    condition_count: 0,
                    consequence_count: 0,
                });
                stat.condition_count += 1;
                if pattern.consequence.holds(index, characters) {
                    stat.consequence_count += 1;
                }
            }
        }
    }
}

fn write_my_file(stats: &HashMap<Pattern, PatternStats>) -> Result<(), Error> {
    let mut file = File::create("output.json")?;
    let mut json_friendly: Vec<(Pattern, PatternStats)> = Vec::new();
    for (key, value) in stats {
        json_friendly.push((*key, *value));
    }
    let text = serde_json::to_string(&json_friendly)?;
    file.write_all(text.as_bytes())?;
    return Ok(());
}

fn predict(character: char, stats: &HashMap<Pattern, PatternStats>) -> Option<char> {
    if let Some((pattern, _)) = stats
        .iter()
        .filter(|(pattern, stats)| {
            (pattern.condition == Observation::CharacterAtSlidingPosition(character, 0))
                && stats.ratio() > 0.1
                && (if let Observation::CharacterAtSlidingPosition(next_char, _) =
                    pattern.consequence
                {
                    next_char != ' '
                } else {
                    true
                })
        })
        .max_by(|a, b| a.1.ratio().partial_cmp(&b.1.ratio()).unwrap())
    {
        match pattern.consequence {
            Observation::CharacterAtSlidingPosition(next_character, _) => {
                return Some(next_character)
            }
        }
    }
    return None;
}

fn predict_sequence(
    starting_character: char,
    max_length: usize,
    stats: &HashMap<Pattern, PatternStats>,
) -> String {
    let mut result: String = String::new();
    result.push(starting_character);
    let mut current_character = starting_character;
    while let Some(char) = predict(current_character, stats) {
        current_character = char;
        result.push(char);
        if result.len() >= max_length {
            break;
        }
    }
    return result;
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
