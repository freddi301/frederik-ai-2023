// there are 100 * 26 input node for a 26 letter alfabet with a sliding window of 100 characters
// every node represents what letter at which position

// iterate over text data and turn simultaneusly nodes to represent current window

// alfabeto = a,b,c
// finsetra = 3
// bba
//  a b c a b c a b c { what letter }
//  1 1 1 2 2 2 3 3 3 { what position }
// | |*| | |*| |*| | |{ is true }
// | |*| | | | |*| | |{ is true }
// | |*| |*| | |*| | |{ is true }
//    *  .3.3   1

// for each turned on input node, create directed link if not exists to all other turned on nodes, and increment that link count
// for each turned on input node, increment its count
// link_count / node_count is the percentage ol probability that when the node is turned on, also the linked node is

use std::collections::HashMap;

use utils::clean_data;

mod utils;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CharacterInSlidingWindow {
    character: char,
    negative_offset: usize,
}

impl CharacterInSlidingWindow {
    fn generate_permutations(
        alphabet: &Vec<char>,
        window_size: usize,
    ) -> Vec<CharacterInSlidingWindow> {
        let mut primitive_terms: Vec<CharacterInSlidingWindow> = Vec::new();
        for negative_offset in 0..window_size {
            for character in alphabet {
                primitive_terms.push(CharacterInSlidingWindow {
                    character: *character,
                    negative_offset: negative_offset,
                })
            }
        }
        primitive_terms
    }
    fn check(&self, data: &Vec<char>, index: usize) -> bool {
        if self.negative_offset <= index {
            if let Some(character) = data.get(index - self.negative_offset) {
                return self.character == *character;
            }
        }
        return false;
    }
}

fn generate_latin_lowercase_alphabet() -> Vec<char> {
    "ABCDEFGHILMNOPQRSTUVZ ,."
        .to_ascii_lowercase()
        .chars()
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Term<Primitive: Clone> {
    Input(Primitive),
    Not(Box<Term<Primitive>>),
    And(Box<Term<Primitive>>, Box<Term<Primitive>>),
    Or(Box<Term<Primitive>>, Box<Term<Primitive>>),
}

impl<Primitive: Clone> Term<Primitive> {
    fn extend_complexity_by_one(all: &mut Vec<Term<Primitive>>) {
        let length = all.len();
        for term_left_index in 0..length {
            let term_left = Box::new(all[term_left_index].clone());
            // all.push(Term::Not(term_left.clone()));
            for term_right_index in 0..length {
                let term_right = Box::new(all[term_right_index].clone());
                all.push(Term::And(term_left.clone(), term_right.clone()));
                // all.push(Term::Or(term_left.clone(), term_right.clone()));
            }
        }
    }
}

impl Term<CharacterInSlidingWindow> {
    fn check(&self, data: &Vec<char>, index: usize) -> bool {
        match self {
            Term::Input(primitive) => primitive.check(data, index),
            Term::Not(term) => !term.check(data, index),
            Term::And(left, right) => left.check(data, index) && right.check(data, index),
            Term::Or(left, right) => left.check(data, index) || right.check(data, index),
        }
    }
}

fn main() {
    let string =
        utils::read_file_to_string("il-piccolo-principe.txt").expect("could not read file");
    let data = clean_data(&string);
    let alphabet = generate_latin_lowercase_alphabet();
    let primitives = CharacterInSlidingWindow::generate_permutations(&alphabet, 2);
    println!("primitives: {}", primitives.len());
    let mut all: Vec<Term<CharacterInSlidingWindow>> = primitives
        .iter()
        .map(|primitive| Term::Input(primitive.clone()))
        .collect();
    println!("dataset size: {}", data.len());
    let mut primitives_occurrences: HashMap<CharacterInSlidingWindow, u32> = HashMap::new();
    for primitive in &primitives {
        let mut occurrences: u32 = 0;
        for index in 0..data.len() {
            if primitive.check(&data, index) {
                occurrences += 1;
            }
        }
        primitives_occurrences.insert(primitive.clone(), occurrences);
    }
    primitive_occurences_to_csv_file(
        "primitive_occurrences.csv",
        &primitives_occurrences,
        data.len(),
    );
    Term::extend_complexity_by_one(&mut all);
    Term::extend_complexity_by_one(&mut all);
    println!("all terms: {}", all.len());
    let mut all_occurrences: HashMap<Term<CharacterInSlidingWindow>, u32> = HashMap::new();
    let progress = indicatif::ProgressBar::new(all.len() as u64);
    for term in &all {
        let mut occurrences: u32 = 0;
        for index in 0..data.len() {
            if term.check(&data, index) {
                occurrences += 1;
            }
        }
        all_occurrences.insert(term.clone(), occurrences);
        progress.inc(1);
    }
    progress.finish();
    term_occurences_to_csv_file("term_occurrences.csv", &all_occurrences, data.len())
}

fn show_permutations_example() {
    let alphabet = generate_latin_lowercase_alphabet();
    let primitives = CharacterInSlidingWindow::generate_permutations(&alphabet, 2);
    let mut all: Vec<Term<CharacterInSlidingWindow>> = primitives
        .into_iter()
        .map(|primitive| Term::Input(primitive))
        .collect();
    Term::extend_complexity_by_one(&mut all);
    dbg!(&all);
    println!("{}", all.len());
}

fn primitive_occurences_to_csv_file(
    file_path: &str,
    primitive_occurrences: &HashMap<CharacterInSlidingWindow, u32>,
    data_size: usize,
) {
    let file =
        std::fs::File::create(file_path).expect("culd not create primitive occurences csv file");
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer
        .write_record(&[
            "character",
            "negative_offset",
            "occurences",
            "occurences_precentage",
        ])
        .expect("could not write csv header");
    for (primitive, occurences) in primitive_occurrences {
        csv_writer
            .write_record(&vec![
                primitive.character.to_string(),
                primitive.negative_offset.to_string(),
                occurences.to_string(),
                (*occurences as f64 / data_size as f64).to_string(),
            ])
            .expect("could not write csv line");
    }
    csv_writer.flush().expect("could not write csv file");
}

fn term_occurences_to_csv_file(
    file_path: &str,
    term_occurrences: &HashMap<Term<CharacterInSlidingWindow>, u32>,
    data_size: usize,
) {
    let file =
        std::fs::File::create(file_path).expect("culd not create primitive occurences csv file");
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer
        .write_record(&["term", "occurences", "occurences_precentage"])
        .expect("could not write csv header");
    for (term, occurences) in term_occurrences {
        csv_writer
            .write_record(&vec![
                term.pretty_text(),
                occurences.to_string(),
                (*occurences as f64 / data_size as f64).to_string(),
            ])
            .expect("could not write csv line");
    }
    csv_writer.flush().expect("could not write csv file");
}

impl CharacterInSlidingWindow {
    fn pretty_text(&self) -> String {
        format!("{}-{}", self.character, self.negative_offset)
    }
}

impl Term<CharacterInSlidingWindow> {
    fn pretty_text(&self) -> String {
        match self {
            Term::Input(primitive) => primitive.pretty_text(),
            Term::Not(term) => format!("not({})", term.pretty_text()),
            Term::And(left, right) => {
                format!("and({}, {})", left.pretty_text(), right.pretty_text())
            }
            Term::Or(left, right) => {
                format!("or({}, {})", left.pretty_text(), right.pretty_text())
            }
        }
    }
}
