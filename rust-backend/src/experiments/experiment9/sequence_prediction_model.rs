use super::boolean_algebra::*;
use std::{collections::HashSet, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct SequencePredictionModel<Symbol: Copy> {
    rules: Vec<(Rule<SymbolAtRelativeIndex<Symbol>>, f64)>,
}

impl<Symbol: Copy + Eq + Hash + Debug> SequencePredictionModel<Symbol> {
    pub fn train(data: &Vec<Symbol>, window_size: usize) -> SequencePredictionModel<Symbol> {
        assert!(window_size > 1);
        let alphabet = Self::derive_alphabet_from_data(data);
        SequencePredictionModel {
            rules: alphabet
                .iter()
                .flat_map(|current_symbol| {
                    alphabet.iter().flat_map(|previous_symbol| {
                        (1..window_size)
                            .map(|offset| -(offset as i32))
                            .map(|negative_offset| {
                                let rule = Rule(
                                    SymbolAtRelativeIndex {
                                        symbol: *previous_symbol,
                                        relative_index: negative_offset,
                                    },
                                    SymbolAtRelativeIndex {
                                        symbol: *current_symbol,
                                        relative_index: 0,
                                    },
                                );
                                let value =
                                    rule.evaluate((0..data.len()).map(|index| (data, index)));
                                (rule, value)
                            })
                    })
                })
                .collect(),
        }
    }
    pub fn predict(&self, sequence: &mut Vec<Symbol>, length: usize) {
        for _ in 0..length {
            let next_symbol = self.predict_next_symbol(&sequence);
            sequence.push(next_symbol);
        }
    }
    fn predict_next_symbol(&self, sequence: &Vec<Symbol>) -> Symbol {
        let current_index = sequence.len();
        // println!("seuqnce: {:?}", sequence);
        self.rules
            .iter()
            .map(|(rule, value)| {
                let new_value = if rule.0.evaluate((sequence, current_index)) {
                    *value
                } else {
                    1.0 - *value
                };
                // println!("{:?} {} {}", rule, value, new_value);
                (rule, new_value)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(rule, _)| rule.1.symbol)
            .unwrap()
    }
    fn derive_alphabet_from_data(data: &Vec<Symbol>) -> HashSet<Symbol> {
        data.iter().copied().collect()
    }
}

#[derive(Debug)]
struct Rule<Term>(Term, Term);

impl<Context: Clone, Term: Evaluate<bool, Context>> Evaluate<bool, Context> for Rule<Term> {
    fn evaluate(&self, context: Context) -> bool {
        fn conditional(x: bool, y: bool) -> bool {
            match (x, y) {
                (true, true) => true,
                (true, false) => true,
                (false, true) => true,
                (false, false) => true,
            }
        }
        fn biconditional(x: bool, y: bool) -> bool {
            match (x, y) {
                (true, true) => true,
                (true, false) => false,
                (false, true) => true,
                (false, false) => true,
            }
        }
        let left = self.0.evaluate(context.clone());
        let right = self.1.evaluate(context.clone());
        biconditional(left, right)
    }
}

#[derive(Debug, Clone, Copy)]
struct SymbolAtRelativeIndex<Symbol: Copy> {
    symbol: Symbol,
    relative_index: i32,
}

impl<Symbol: Eq + Copy> Evaluate<bool, (&Vec<Symbol>, usize)> for SymbolAtRelativeIndex<Symbol> {
    fn evaluate(&self, (data, index): (&Vec<Symbol>, usize)) -> bool {
        let index = index as isize + self.relative_index as isize;
        if index < 0 || index >= data.len() as isize {
            return false;
        }
        data[index as usize] == self.symbol
    }
}
