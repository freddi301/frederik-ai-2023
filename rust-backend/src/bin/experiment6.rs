// predict next character

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{Error, Read},
    rc::Rc,
};

fn main() -> Result<(), Error> {
    let string = read_file_to_string("il-piccolo-principe.txt")?;
    let data = Rc::new(string_to_character_vector_simplified_italian(&string));
    let alphabet = derive_alphabet_from_data(&data);
    let input_variables_terms = get_input_variables(&alphabet, 3);
    println!("input variables: {}", input_variables_terms.len());
    let level_1_terms = create_base_terms(&input_variables_terms);
    println!("level 1 terms: {}", level_1_terms.len());
    let progress =
        indicatif::ProgressBar::new((level_1_terms.len() + input_variables_terms.len()) as u64);
    write_csv_file(
        "report.csv",
        &["term", "accuracy"],
        input_variables_terms
            .iter()
            .chain(level_1_terms.iter())
            .map(|term| {
                progress.inc(1);
                term
            })
            .map(|term| {
                (
                    term.human_readable(),
                    compute_accuracy(term, contexts_from_data(data.clone())),
                )
            })
            .filter(|(_, accuracy)| *accuracy > 0.0)
            .map(|(term, accuracy)| vec![term, format!("{}", accuracy)]),
    )?;
    progress.finish();
    Ok(())
}

fn read_file_to_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn string_to_character_vector_simplified_italian(string: &str) -> Vec<char> {
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

fn derive_alphabet_from_data(character_vector: &Vec<char>) -> HashSet<char> {
    let mut alphabet: HashSet<char> = HashSet::new();
    for character in character_vector {
        alphabet.insert(*character);
    }
    alphabet
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CharacterInWindow {
    character: char,
    negative_offset: usize,
}

#[derive(Debug)]
struct CharacterWindow {
    data: Rc<Vec<char>>,
    index: usize,
}

impl EvaluateIn<CharacterWindow> for CharacterInWindow {
    fn evaluate(&self, context: &CharacterWindow) -> bool {
        if self.negative_offset > context.index {
            return false;
        }
        self.character == context.data[context.index - self.negative_offset]
    }
}

fn get_input_variables<'a>(
    alphabet: &HashSet<char>,
    window_size: usize,
) -> HashSet<Rc<Term<CharacterInWindow>>> {
    let mut input_variables: HashSet<Rc<Term<CharacterInWindow>>> = HashSet::new();
    for character in alphabet {
        for negative_offset in 0..window_size {
            let character_in_window = CharacterInWindow {
                character: *character,
                negative_offset,
            };
            let term = Term::Variable(Rc::new(character_in_window));
            input_variables.insert(Rc::new(term));
        }
    }
    input_variables
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Term<Variable> {
    Variable(Rc<Variable>),
    Not(Rc<Self>),
    And(Rc<Self>, Rc<Self>),
    Or(Rc<Self>, Rc<Self>),
}

pub trait EvaluateIn<Context> {
    fn evaluate(&self, context: &Context) -> bool;
}

impl<Context, Variable: EvaluateIn<Context>> EvaluateIn<Context> for Term<Variable> {
    fn evaluate(&self, context: &Context) -> bool {
        use Term::*;
        match self {
            Variable(variable) => variable.evaluate(context),
            Not(x) => !x.evaluate(context),
            And(x, y) => x.evaluate(context) && y.evaluate(context),
            Or(x, y) => x.evaluate(context) || y.evaluate(context),
        }
    }
}

fn compute_accuracy<Context, Variable: EvaluateIn<Context>>(
    term: &Term<Variable>,
    contexts: impl Iterator<Item = Context>,
) -> f64 {
    let mut correct = 0;
    let mut total = 0;
    for context in contexts {
        if term.evaluate(&context) {
            correct += 1;
        }
        total += 1;
    }
    correct as f64 / total as f64
}

fn contexts_from_data(data: Rc<Vec<char>>) -> impl Iterator<Item = CharacterWindow> {
    (0..data.len()).map(move |index| CharacterWindow {
        data: data.clone(),
        index,
    })
}

fn create_base_terms<Variable: Eq + Hash>(
    existing_terms: &HashSet<Rc<Term<Variable>>>,
) -> HashSet<Rc<Term<Variable>>> {
    let mut base_terms: HashSet<Rc<Term<Variable>>> = HashSet::new();
    for left in existing_terms {
        base_terms.insert(Rc::new(Term::Not(left.clone())));
        for right in existing_terms {
            base_terms.insert(Rc::new(Term::And(left.clone(), right.clone())));
            base_terms.insert(Rc::new(Term::Or(left.clone(), right.clone())));
        }
    }
    base_terms
}

trait HumanReadable {
    fn human_readable(&self) -> String;
}

impl HumanReadable for CharacterInWindow {
    fn human_readable(&self) -> String {
        format!("{}{}", self.negative_offset, self.character)
    }
}

impl<Variable: HumanReadable> HumanReadable for Term<Variable> {
    fn human_readable(&self) -> String {
        use Term::*;
        match self {
            Variable(variable) => variable.human_readable(),
            Not(x) => format!("¬{}", x.human_readable()),
            And(x, y) => format!("({} ∧ {})", x.human_readable(), y.human_readable()),
            Or(x, y) => format!("({} ∨ {})", x.human_readable(), y.human_readable()),
        }
    }
}

fn write_csv_file<'a>(
    file_path: &str,
    columns: &[&str],
    rows: impl Iterator<Item = Vec<String>>,
) -> Result<(), Error> {
    let file = File::create(file_path)?;
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer.write_record(columns)?;
    for row in rows {
        csv_writer.write_record(row)?;
    }
    csv_writer.flush()?;
    Ok(())
}
