// predict next character

use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{Error, Read},
    rc::Rc,
};

use character_window::*;
use rc_repository::*;
use term::*;

fn main() -> Result<(), Error> {
    let mut variable_repository: RcRepository<CharacterInWindow> = RcRepository::new();
    let mut term_repository: RcRepository<Term<CharacterInWindow>> = RcRepository::new();
    let string = read_file_to_string("il-piccolo-principe.txt")?;
    let data = Rc::new(string_to_character_vector_simplified_italian(&string));
    let alphabet = derive_alphabet_from_data(&data);
    let input_variables_terms =
        get_input_variables(&mut variable_repository, &mut term_repository, &alphabet, 3);
    println!("input variables: {}", input_variables_terms.len());
    let level_1_terms = create_new_level_terms(&mut term_repository, &input_variables_terms);
    println!("level 1 terms: {}", level_1_terms.len());
    let input_and_level_1_terms: HashSet<Rc<Term<CharacterInWindow>>> = input_variables_terms
        .iter()
        .chain(level_1_terms.iter())
        .cloned()
        .collect();
    println!("input and level 1 terms: {}", input_and_level_1_terms.len());
    let input_and_level_1_terms_truth_tables: HashSet<TruthTable<CharacterInWindow>> =
        input_and_level_1_terms
            .iter()
            .map(|term| term.compute_truth_table())
            .collect();
    println!(
        "input and level 1 terms truth tables: {}",
        input_and_level_1_terms_truth_tables.len()
    );
    // let level_2_terms = create_new_level_terms(&mut term_repository, &input_and_level_1_terms);
    // println!("level 2 terms: {}", level_2_terms.len());
    // let input_and_level_1_and_level_2_terms: HashSet<Rc<Term<CharacterInWindow>>> =
    //     input_and_level_1_terms
    //         .iter()
    //         .chain(level_2_terms.iter())
    //         .cloned()
    //         .collect();
    // println!(
    //     "input and level 1 and level 2 terms: {}",
    //     input_and_level_1_and_level_2_terms.len()
    // );
    // let input_and_level_1_and_level_2_terms_truth_tables: HashSet<TruthTable<CharacterInWindow>> =
    //     input_and_level_1_and_level_2_terms
    //         .iter()
    //         .map(|term| term.compute_truth_table())
    //         .collect();
    // println!(
    //     "input and level 1 terms and level 2 truth tables: {}",
    //     input_and_level_1_and_level_2_terms_truth_tables.len()
    // );
    let progress = indicatif::ProgressBar::new((input_and_level_1_terms_truth_tables.len()) as u64);
    write_csv_file(
        "truth-table-report.csv",
        &["truth_table", "accuracy"],
        input_and_level_1_terms_truth_tables
            .iter()
            .map(|truth_table| {
                progress.inc(1);
                truth_table
            })
            .map(|truth_table| {
                (
                    format!("{:?}", truth_table),
                    compute_accuracy::<
                        CharacterWindow,
                        CharacterInWindow,
                        TruthTable<CharacterInWindow>,
                    >(truth_table, contexts_from_data(data.clone())),
                )
            })
            .map(|(truth_table, accuracy)| vec![truth_table, format!("{}", accuracy)]),
    )?;
    progress.finish();
    let progress = indicatif::ProgressBar::new((input_and_level_1_terms.len()) as u64);
    write_csv_file(
        "term-report.csv",
        &["term", "accuracy"],
        input_and_level_1_terms
            .iter()
            .map(|term| {
                progress.inc(1);
                term
            })
            .map(|term| {
                (
                    term.human_readable(),
                    compute_accuracy::<CharacterWindow, CharacterInWindow, Term<CharacterInWindow>>(
                        term.as_ref(),
                        contexts_from_data(data.clone()),
                    ),
                )
            })
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

fn get_input_variables(
    variable_repository: &mut RcRepository<CharacterInWindow>,
    term_repository: &mut RcRepository<Term<CharacterInWindow>>,
    alphabet: &HashSet<char>,
    window_size: usize,
) -> HashSet<Rc<Term<CharacterInWindow>>> {
    let mut input_variables: HashSet<Rc<Term<CharacterInWindow>>> = HashSet::new();
    for character in alphabet {
        for negative_offset in 0..window_size {
            let character_in_window =
                variable_repository.character_in_window(negative_offset, *character);
            let term = term_repository.var(&character_in_window);
            input_variables.insert(term);
        }
    }
    input_variables
}

pub trait EvaluateVariableIn<Context> {
    fn evaluate_variable_in(&self, context: &Context) -> bool;
}

pub trait EvaluateIn<Context> {
    fn evaluate_in(&self, context: &Context) -> bool;
}

fn compute_accuracy<Context, Variable: EvaluateVariableIn<Context>, T: EvaluateIn<Context>>(
    t: &T,
    contexts: impl Iterator<Item = Context>,
) -> f64 {
    let mut correct = 0;
    let mut total = 0;
    for context in contexts {
        if t.evaluate_in(&context) {
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

fn create_new_level_terms<Variable: Eq + Hash>(
    term_repository: &mut RcRepository<Term<Variable>>,
    existing_terms: &HashSet<Rc<Term<Variable>>>,
) -> HashSet<Rc<Term<Variable>>> {
    let mut base_terms: HashSet<Rc<Term<Variable>>> = HashSet::new();
    for left in existing_terms {
        base_terms.insert(term_repository.not(left));
        for right in existing_terms {
            base_terms.insert(term_repository.and(left, right));
            base_terms.insert(term_repository.or(left, right));
        }
    }
    base_terms
}

trait HumanReadable {
    fn human_readable(&self) -> String;
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

mod rc_repository {
    use std::{collections::HashSet, hash::Hash, rc::Rc};

    pub struct RcRepository<Item>(HashSet<Rc<Item>>);

    impl<Item: Eq + Hash> RcRepository<Item> {
        pub fn new() -> Self {
            RcRepository(HashSet::new())
        }
        pub fn get_or_create(&mut self, item: Item) -> Rc<Item> {
            if let Some(existing) = self.0.get(&item) {
                existing.clone()
            } else {
                let new = Rc::new(item);
                self.0.insert(new.clone());
                new
            }
        }
    }
}

mod character_window {
    use super::rc_repository::*;
    use super::*;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct CharacterInWindow {
        character: char,
        negative_offset: usize,
    }

    #[derive(Debug)]
    pub struct CharacterWindow {
        pub data: Rc<Vec<char>>,
        pub index: usize,
    }

    impl EvaluateVariableIn<CharacterWindow> for CharacterInWindow {
        fn evaluate_variable_in(&self, context: &CharacterWindow) -> bool {
            if self.negative_offset > context.index {
                return false;
            }
            self.character == context.data[context.index - self.negative_offset]
        }
    }

    impl HumanReadable for CharacterInWindow {
        fn human_readable(&self) -> String {
            format!("{}{}", self.negative_offset, self.character)
        }
    }

    impl RcRepository<CharacterInWindow> {
        pub fn character_in_window(
            &mut self,
            negative_offset: usize,
            character: char,
        ) -> Rc<CharacterInWindow> {
            self.get_or_create(CharacterInWindow {
                character,
                negative_offset,
            })
        }
    }
}

mod term {
    use super::rc_repository::*;
    use super::*;
    use std::{
        collections::{BTreeSet, HashMap},
        hash::Hash,
        rc::Rc,
    };

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum Expr<Variable> {
        Var(Rc<Variable>),
        Not(Rc<Term<Variable>>),
        And(Rc<Term<Variable>>, Rc<Term<Variable>>),
        Or(Rc<Term<Variable>>, Rc<Term<Variable>>),
    }

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Term<Variable>(Expr<Variable>);

    impl<Variable: Eq + Hash> RcRepository<Term<Variable>> {
        pub fn var(&mut self, x: &Rc<Variable>) -> Rc<Term<Variable>> {
            self.get_or_create(Term(Expr::Var(x.clone())))
        }
        pub fn not(&mut self, x: &Rc<Term<Variable>>) -> Rc<Term<Variable>> {
            self.get_or_create(Term(Expr::Not(x.clone())))
        }
        pub fn and(
            &mut self,
            x: &Rc<Term<Variable>>,
            y: &Rc<Term<Variable>>,
        ) -> Rc<Term<Variable>> {
            self.get_or_create(Term(Expr::And(x.clone(), y.clone())))
        }
        pub fn or(&mut self, x: &Rc<Term<Variable>>, y: &Rc<Term<Variable>>) -> Rc<Term<Variable>> {
            self.get_or_create(Term(Expr::Or(x.clone(), y.clone())))
        }
        pub fn conditional(
            &mut self,
            x: &Rc<Term<Variable>>,
            y: &Rc<Term<Variable>>,
        ) -> Rc<Term<Variable>> {
            let not_x = self.not(x);
            self.or(&not_x, y)
        }
        pub fn biconditional(
            &mut self,
            x: &Rc<Term<Variable>>,
            y: &Rc<Term<Variable>>,
        ) -> Rc<Term<Variable>> {
            let and_x_y = self.and(x, y);
            let not_x = self.not(x);
            let not_y = self.not(y);
            let and_not_x_not_y = self.and(&not_x, &not_y);
            self.or(&and_x_y, &and_not_x_not_y)
        }
    }

    impl<Context, Variable: EvaluateVariableIn<Context>> EvaluateIn<Context> for Term<Variable> {
        fn evaluate_in(&self, context: &Context) -> bool {
            use Expr::*;
            match &self.0 {
                Var(variable) => variable.evaluate_variable_in(context),
                Not(x) => !x.evaluate_in(context),
                And(x, y) => x.evaluate_in(context) && y.evaluate_in(context),
                Or(x, y) => x.evaluate_in(context) || y.evaluate_in(context),
            }
        }
    }

    impl<Variable: HumanReadable> HumanReadable for Term<Variable> {
        fn human_readable(&self) -> String {
            use Expr::*;
            match &self.0 {
                Var(variable) => variable.human_readable(),
                Not(x) => format!("¬{}", x.human_readable()),
                And(x, y) => format!("({} ∧ {})", x.human_readable(), y.human_readable()),
                Or(x, y) => format!("({} ∨ {})", x.human_readable(), y.human_readable()),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TruthTable<Variable: Ord> {
        variables: BTreeSet<Rc<Variable>>,
        results: Vec<bool>,
    }
    impl<Variable: Ord + Hash> Term<Variable> {
        fn get_variables(&self, variables: &mut BTreeSet<Rc<Variable>>) {
            use Expr::*;
            match &self.0 {
                Var(variable) => {
                    variables.insert(variable.clone());
                }
                Not(x) => x.get_variables(variables),
                And(x, y) => {
                    x.get_variables(variables);
                    y.get_variables(variables);
                }
                Or(x, y) => {
                    x.get_variables(variables);
                    y.get_variables(variables);
                }
            }
        }
        pub fn compute_truth_table(&self) -> TruthTable<Variable> {
            let mut variables: BTreeSet<Rc<Variable>> = BTreeSet::new();
            self.get_variables(&mut variables);
            let variables_index: &HashMap<Rc<Variable>, usize> = &variables
                .iter()
                .enumerate()
                .map(|(index, variable)| (variable.clone(), index))
                .collect();
            let combinations = (2 as u32).pow(variables.len() as u32) as usize;
            let mut results = vec![false; combinations];
            for combination_index in 0..combinations {
                results[combination_index] = self.evaluate_in(&TruthTableContext {
                    variables_index,
                    combination_index,
                })
            }
            TruthTable { variables, results }
        }
    }

    struct TruthTableContext<'a, Variable> {
        variables_index: &'a HashMap<Rc<Variable>, usize>,
        combination_index: usize,
    }

    impl<'a, Variable: Eq + Hash> EvaluateVariableIn<TruthTableContext<'a, Variable>> for Variable {
        fn evaluate_variable_in(&self, context: &TruthTableContext<Variable>) -> bool {
            let variable_index = context.variables_index.get(self).unwrap();
            (context.combination_index >> variable_index) & 1 == 1
        }
    }

    impl<'a, Variable: Eq + Hash> EvaluateVariableIn<HashMap<Variable, bool>> for Variable {
        fn evaluate_variable_in(&self, context: &HashMap<Variable, bool>) -> bool {
            *context.get(self).unwrap()
        }
    }

    impl<Context, Variable: Ord + EvaluateVariableIn<Context>> EvaluateIn<Context>
        for TruthTable<Variable>
    {
        fn evaluate_in(&self, context: &Context) -> bool {
            let combination_index = self.variables.iter().enumerate().fold(
                0,
                |combination_index, (variable_index, variable)| {
                    if variable.evaluate_variable_in(context) {
                        combination_index | (1 << variable_index)
                    } else {
                        combination_index
                    }
                },
            );
            self.results[combination_index]
        }
    }

    #[test]
    fn test_truth_table_correctly_derived() {
        let mut variable_repository: RcRepository<char> = RcRepository::new();
        let mut term_repository: RcRepository<Term<char>> = RcRepository::new();
        let a = term_repository.var(&variable_repository.get_or_create('a'));
        let b = term_repository.var(&variable_repository.get_or_create('b'));
        let a_and_b = term_repository.and(&a, &b);
        let a_and_a = term_repository.and(&a, &a);
        let b_and_a = term_repository.and(&b, &a);
        let a_or_a = term_repository.or(&a, &a);
        let a_or_b = term_repository.or(&a, &b);
        let b_or_a = term_repository.or(&b, &a);
        let not_a = term_repository.not(&a);
        let not_not_a = term_repository.not(&not_a);
        println!("a {:#?}", a.compute_truth_table());
        println!("a & a {:#?}", a_and_a.compute_truth_table());
        assert_eq!(a.compute_truth_table(), a_and_a.compute_truth_table());
        println!("a & b {:#?}", a_and_b.compute_truth_table());
        println!("b & a {:#?}", b_and_a.compute_truth_table());
        assert_eq!(a_and_b.compute_truth_table(), b_and_a.compute_truth_table());
        println!("a | a {:#?}", a_or_a.compute_truth_table());
        assert_eq!(a.compute_truth_table(), a_or_a.compute_truth_table());
        println!("a | b {:#?}", a_or_b.compute_truth_table());
        println!("b | a {:#?}", b_or_a.compute_truth_table());
        assert_eq!(a_or_b.compute_truth_table(), b_or_a.compute_truth_table());
        println!("!a {:#?}", not_a.compute_truth_table());
        println!("!!a {:#?}", not_not_a.compute_truth_table());
        assert_eq!(a.compute_truth_table(), not_not_a.compute_truth_table());
    }

    #[test]
    fn test_evaluate_truth_table() {
        let mut variable_repository: RcRepository<char> = RcRepository::new();
        let mut term_repository: RcRepository<Term<char>> = RcRepository::new();
        let a = term_repository.var(&variable_repository.get_or_create('a'));
        let b = term_repository.var(&variable_repository.get_or_create('b'));
        let a_and_b = term_repository.and(&a, &b);
        let a_or_b = term_repository.or(&a, &b);
        let not_a = term_repository.not(&a);
        assert_eq!(
            a.compute_truth_table()
                .evaluate_in(&HashMap::from([('a', true)])),
            true
        );
        assert_eq!(
            a.compute_truth_table()
                .evaluate_in(&HashMap::from([('a', false)])),
            false
        );
        assert_eq!(
            a_and_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', true), ('b', true)])),
            true
        );
        assert_eq!(
            a_and_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', true), ('b', false)])),
            false
        );
        assert_eq!(
            a_and_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', false), ('b', true)])),
            false
        );
        assert_eq!(
            a_and_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', false), ('b', false)])),
            false
        );
        assert_eq!(
            a_or_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', true), ('b', true)])),
            true
        );
        assert_eq!(
            a_or_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', true), ('b', false)])),
            true
        );
        assert_eq!(
            a_or_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', false), ('b', true)])),
            true
        );
        assert_eq!(
            a_or_b
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', false), ('b', false)])),
            false
        );
        assert_eq!(
            not_a
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', true)])),
            false
        );
        assert_eq!(
            not_a
                .compute_truth_table()
                .evaluate_in(&HashMap::from([('a', false)])),
            true
        );
    }
}
