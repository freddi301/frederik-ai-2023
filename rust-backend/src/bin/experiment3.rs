use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Error, Read};
use std::rc::Rc;

// main function
fn main() -> Result<(), Error> {
    let string = read_file_to_string("il-piccolo-principe.txt")?;
    let data = string_to_character_vector_italian_lowercase_and_space(&string);
    // dbg!(&data.iter().collect::<String>());
    let alhapbet = generate_simplified_italian_alphabet();
    let input_terms = alphabet_to_input_terms(&alhapbet, 3);
    println!("input terms: {}", input_terms.len());
    let and_terms = synthetize_and_terms(&input_terms, &data);
    println!("and terms: {}", and_terms.len());
    let terms = [input_terms, and_terms].concat();
    term_occurrences_count_to_csv(&terms, &data, "term_occurrences_count.csv")?;
    term_occurrences_count_by_next_character_to_csv_file(
        &terms,
        &data,
        "term_occurrences_count_by_next_character.csv",
    )?;
    term_statistics_to_csv_file(&terms, &data, "term_statistics.csv")?;
    let terms_statistics_relative = get_term_statistics_relative(&terms, &data);
    // dbg!(predict_next_character_breakdown(
    //     &terms_statistics_relative,
    //     &"pri".chars().collect()
    // ));
    println!(
        "predicted: {}",
        predict_next_characters(&terms_statistics_relative, &"lavo".chars().collect(), 20)
    );
    Ok(())
}

// function that reads file to string, returning result
fn read_file_to_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// letters to lower case
// others to space
// deduplicate consecutive spaces
fn string_to_character_vector_italian_lowercase_and_space(string: &str) -> Vec<char> {
    let mut character_vector: Vec<char> = Vec::new();
    let mut last_character_was_separator = false;
    for character in string.to_lowercase().chars() {
        let transformed_character = if character.is_alphabetic() {
            character
        } else {
            ' '
        };
        if transformed_character == ' ' {
            if !last_character_was_separator {
                character_vector.push(transformed_character);
            }
            last_character_was_separator = true;
        } else {
            character_vector.push(transformed_character);
            last_character_was_separator = false;
        }
    }
    character_vector
}

fn generate_simplified_italian_alphabet() -> Vec<char> {
    "abcdefghijklmnopqrstuvwxyz àèéìòù".chars().collect()
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd)]
struct CharacterAtWindowPosition {
    character: char,
    negative_offset: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd)]
enum Term {
    Input(CharacterAtWindowPosition),
    And(Rc<Term>, Rc<Term>),
}

impl CharacterAtWindowPosition {
    fn check(&self, data: &Vec<char>, index: usize) -> bool {
        if index < self.negative_offset {
            return false;
        }
        data[index - self.negative_offset] == self.character
    }
}

impl Term {
    fn check(&self, data: &Vec<char>, index: usize) -> bool {
        match self {
            Term::Input(character_at_window_position) => {
                character_at_window_position.check(data, index)
            }
            Term::And(left, right) => left.check(data, index) && right.check(data, index),
        }
    }
}

fn alphabet_to_input_terms(alphabet: &Vec<char>, window_size: usize) -> Vec<Rc<Term>> {
    let mut input_terms: Vec<Rc<Term>> = Vec::new();
    for character in alphabet {
        for negative_offset in 0..window_size {
            input_terms.push(Rc::new(Term::Input(CharacterAtWindowPosition {
                character: *character,
                negative_offset,
            })));
        }
    }
    input_terms
}

fn term_occurrences_count(term: &Term, data: &Vec<char>) -> u32 {
    let mut occurrences: u32 = 0;
    for index in 0..data.len() {
        if term.check(data, index) {
            occurrences += 1;
        }
    }
    occurrences
}

fn term_occurrences_count_to_csv(
    terms: &Vec<Rc<Term>>,
    data: &Vec<char>,
    file_path: &str,
) -> Result<(), Error> {
    let file = File::create(file_path)?;
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer.write_record(&["term", "occurrences", "occurrences_percentage"])?;
    for term in terms {
        let occurrences = term_occurrences_count(term, data);
        let occurrences_percentage = occurrences as f64 / data.len() as f64;
        csv_writer.write_record(&[
            &term.pretty_csv(),
            &format!("{}", occurrences),
            &format!("{}", occurrences_percentage),
        ])?;
    }
    csv_writer.flush()?;
    Ok(())
}

impl CharacterAtWindowPosition {
    fn pretty_csv(&self) -> String {
        format!(
            "{}{}",
            self.negative_offset,
            if self.character == ' ' {
                '_'
            } else {
                self.character
            }
        )
    }
}

impl Term {
    fn pretty_csv(&self) -> String {
        match self {
            Term::Input(character_at_window_position) => character_at_window_position.pretty_csv(),
            Term::And(left, right) => format!("({} & {})", left.pretty_csv(), right.pretty_csv()),
        }
    }
}

fn term_occurrences_count_by_next_character(term: &Term, data: &Vec<char>) -> HashMap<char, u32> {
    let mut occurrences_by_character: HashMap<char, u32> = HashMap::new();
    for index in 0..data.len() {
        if term.check(data, index) {
            if (index + 1) < data.len() {
                let next_character = data[index + 1];
                let occurrences = occurrences_by_character.entry(next_character).or_insert(0);
                *occurrences += 1;
            }
        }
    }
    occurrences_by_character
}

fn term_occurrences_count_by_next_character_to_csv_file(
    terms: &Vec<Rc<Term>>,
    data: &Vec<char>,
    file_path: &str,
) -> Result<(), Error> {
    let file = File::create(file_path)?;
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer.write_record(&["term", "next_character", "next_character_occurrences"])?;
    for term in terms {
        let occurrences_by_character = term_occurrences_count_by_next_character(term, data);
        for (character, occurrences) in occurrences_by_character {
            csv_writer.write_record(&[
                &term.pretty_csv(),
                &format!("{}", if character == ' ' { '_' } else { character }),
                &format!("{}", occurrences),
            ])?;
        }
    }
    csv_writer.flush()?;
    Ok(())
}

fn term_statistics_to_csv_file(
    terms: &Vec<Rc<Term>>,
    data: &Vec<char>,
    file_path: &str,
) -> Result<(), Error> {
    let file = File::create(file_path)?;
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer.write_record(&[
        "term",
        "occurrences",
        "occurrences_percentage",
        "next_character",
        "next_character_occurrences",
        "next_character_occurrences_percentage",
    ])?;
    let progress = ProgressBar::new(terms.len() as u64);
    for term in terms {
        let (occurrences, occurrences_by_character) = get_term_statistics(term, data);
        let occurrences_percentage = occurrences as f64 / data.len() as f64;
        for (character, next_character_occurrences) in occurrences_by_character {
            let next_character_occurrences_percentage =
                next_character_occurrences as f64 / occurrences as f64;
            csv_writer.write_record(&[
                &term.pretty_csv(),
                &format!("{}", occurrences),
                &format!("{}", occurrences_percentage),
                &format!("{}", if character == ' ' { '_' } else { character }),
                &format!("{}", next_character_occurrences),
                &format!("{}", next_character_occurrences_percentage),
            ])?;
        }
        progress.inc(1);
    }
    progress.finish();
    csv_writer.flush()?;
    Ok(())
}

fn synthetize_and_terms(input_terms: &Vec<Rc<Term>>, data: &Vec<char>) -> Vec<Rc<Term>> {
    let mut and_terms: HashSet<Rc<Term>> = HashSet::new();
    for i in 0..data.len() {
        let matching_terms = input_terms
            .iter()
            .filter(|term| term.check(data, i))
            .cloned()
            .collect::<Vec<Rc<Term>>>();
        for left in &matching_terms {
            for right in &matching_terms {
                if let Some(std::cmp::Ordering::Less) = left.partial_cmp(right) {
                    let and_term = Rc::new(Term::And(left.clone(), right.clone()));
                    and_terms.insert(and_term);
                }
            }
        }
    }
    and_terms.iter().cloned().collect()
}

fn get_term_statistics(term: &Term, data: &Vec<char>) -> (u32, HashMap<char, u32>) {
    let mut occurrences: u32 = 0;
    let mut occurrences_by_character: HashMap<char, u32> = HashMap::new();
    for index in 0..data.len() {
        if term.check(data, index) {
            occurrences += 1;
            if (index + 1) < data.len() {
                let next_character = data[index + 1];
                let occurrences = occurrences_by_character.entry(next_character).or_insert(0);
                *occurrences += 1;
            }
        }
    }
    (occurrences, occurrences_by_character)
}

fn get_term_statistics_relative(
    terms: &Vec<Rc<Term>>,
    data: &Vec<char>,
) -> HashMap<Rc<Term>, (f64, HashMap<char, f64>)> {
    terms
        .iter()
        .map(|term| {
            let (term_occurrences, occurrences_by_character) = get_term_statistics(term, data);
            (
                term.clone(),
                (
                    term_occurrences as f64 / data.len() as f64,
                    occurrences_by_character
                        .iter()
                        .map(|(character, character_occurrences)| {
                            (
                                *character,
                                *character_occurrences as f64 / term_occurrences as f64,
                            )
                        })
                        .collect(),
                ),
            )
        })
        .collect()
}

fn predict_next_character(
    terms_statistics_relative: &HashMap<Rc<Term>, (f64, HashMap<char, f64>)>,
    sequence: &Vec<char>,
) -> Option<char> {
    if sequence.len() < 1 {
        return None;
    }
    terms_statistics_relative
        .iter()
        .filter(|(term, _)| term.check(sequence, sequence.len() - 1))
        .flat_map(|(_, (_, by_character))| by_character.iter())
        .map(|(character, probability)| {
            if *character == ' ' {
                (character, probability * 0.5)
            } else {
                (character, *probability)
            }
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(character, _)| *character)
}

fn predict_next_character_breakdown(
    terms_statistics_relative: &HashMap<Rc<Term>, (f64, HashMap<char, f64>)>,
    sequence: &Vec<char>,
) -> Vec<String> {
    if sequence.len() < 1 {
        return vec![];
    }
    let mut breakdown: Vec<(Rc<Term>, char, f64)> = terms_statistics_relative
        .iter()
        .filter(|(term, _)| term.check(sequence, sequence.len() - 1))
        .flat_map(|(term, (_, by_character))| {
            by_character
                .iter()
                .map(|(character, probability)| (term.clone(), *character, *probability))
        })
        .collect();
    breakdown.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());
    breakdown
        .iter()
        .map(|(term, character, probability)| {
            format!("{} {} {}", term.pretty_csv(), character, probability)
        })
        .collect()
}

fn predict_next_characters(
    terms_statistics_relative: &HashMap<Rc<Term>, (f64, HashMap<char, f64>)>,
    initial_sequence: &Vec<char>,
    max_predictions: usize,
) -> String {
    let mut sequence = initial_sequence.clone();
    for _ in 0..max_predictions {
        if let Some(next_character) = predict_next_character(terms_statistics_relative, &sequence) {
            sequence.push(next_character);
        } else {
            break;
        }
    }
    sequence.iter().collect()
}

// how many times a term wase used successfully to predict on trained data?
// scroll training data wth predict results

// how many time a term was used to predict on trained data?
