use super::rc_repository::*;
use super::utils::*;
use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    rc::Rc,
};

pub trait EvaluateVariableIn<Context> {
    fn evaluate_variable_in(&self, context: &Context) -> bool;
}

pub trait EvaluateIn<Context> {
    fn evaluate_in(&self, context: &Context) -> bool;
}

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
    pub fn and(&mut self, x: &Rc<Term<Variable>>, y: &Rc<Term<Variable>>) -> Rc<Term<Variable>> {
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

pub fn compute_accuracy<Context, Variable: EvaluateVariableIn<Context>, T: EvaluateIn<Context>>(
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
