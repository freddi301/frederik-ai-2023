mod boolean_algebra;
mod character_sequence_prediction_model;
mod sequence_prediction_model;

pub fn main() {
    // here only to avoid "unused code" warning
    character_sequence_prediction_model::CharacterSequencePredictionModel::train("", 0)
        .predict("", 0);
}

mod test {
    #[cfg(test)]
    use super::character_sequence_prediction_model::CharacterSequencePredictionModel;

    #[test]
    fn predict_1() {
        let model = CharacterSequencePredictionModel::train("abababababab", 2);
        assert_eq!(model.predict("a", 1), "b");
        assert_eq!(model.predict("b", 1), "a");
        assert_eq!(model.predict("a", 2), "ba");
        assert_eq!(model.predict("b", 2), "ab");
        assert_eq!(model.predict("a", 10), "bababababa");
        assert_eq!(model.predict("b", 10), "ababababab");
    }

    #[test]
    fn predict_2() {
        let model = CharacterSequencePredictionModel::train("abcabcabcabc", 2);
        assert_eq!(model.predict("a", 1), "b");
        assert_eq!(model.predict("b", 1), "c");
        assert_eq!(model.predict("c", 1), "a");
        assert_eq!(model.predict("a", 2), "bc");
        assert_eq!(model.predict("b", 2), "ca");
        assert_eq!(model.predict("c", 2), "ab");
        assert_eq!(model.predict("a", 10), "bcabcabcab");
        assert_eq!(model.predict("b", 10), "cabcabcabc");
        assert_eq!(model.predict("c", 10), "abcabcabca");
    }

    // failed to predict with this experiment strategy, " " <-> " " has value 0.7
    #[test]
    fn predict_3() {
        let model = CharacterSequencePredictionModel::train("mamma mamma mamma mamma", 2);
        dbg!(&model);
        assert_eq!(model.predict(" ", 6), "mamma ");
        assert_eq!(model.predict(" ma", 6), "mma ma");
        assert_eq!(model.predict("mm", 6), "a mamm");
    }
}
