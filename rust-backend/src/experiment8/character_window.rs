use super::boolean_algebra::*;
use super::rc_repository::*;
use super::utils::*;
use std::collections::HashSet;
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

pub fn derive_alphabet_from_data(character_vector: impl Iterator<Item = char>) -> HashSet<char> {
    let mut alphabet: HashSet<char> = HashSet::new();
    for character in character_vector {
        alphabet.insert(character);
    }
    alphabet
}

pub fn contexts_from_data(data: Rc<Vec<char>>) -> impl Iterator<Item = CharacterWindow> {
    (0..data.len()).map(move |index| CharacterWindow {
        data: data.clone(),
        index,
    })
}
