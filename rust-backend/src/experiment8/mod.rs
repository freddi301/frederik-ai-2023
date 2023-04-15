mod boolean_algebra;
mod character_window;
mod rc_repository;
mod utils;

mod experiments {
    use std::rc::Rc;

    use super::boolean_algebra::*;
    use super::character_window::*;
    use super::rc_repository::*;
    use super::utils::*;

    #[test]
    fn next_character_biconditional() {
        let text = "abababababababababababababababab";
        let data = Rc::new(text.chars().collect::<Vec<char>>());
        let alphabet = derive_alphabet_from_data(text.chars());
        let mut v: RcRepository<CharacterInWindow> = RcRepository::new();
        let mut t: RcRepository<Term<CharacterInWindow>> = RcRepository::new();
        let mut accuracies: Vec<(char, char, f64)> = Vec::new();
        for current_character in alphabet.iter() {
            for previous_character in alphabet.iter() {
                let current_character_term = t.var(&v.character_in_window(0, *current_character));
                let previous_character_term = t.var(&v.character_in_window(1, *previous_character));
                let conditional =
                    t.biconditional(&previous_character_term, &current_character_term);
                let accuracy = compute_accuracy::<
                    CharacterWindow,
                    CharacterInWindow,
                    Term<CharacterInWindow>,
                >(
                    conditional.as_ref(), contexts_from_data(data.clone())
                );
                accuracies.push((*previous_character, *current_character, accuracy));
            }
        }
        dbg!(accuracies);
    }
}
