use super::sequence_prediction_model::SequencePredictionModel;

#[derive(Debug)]
pub struct CharacterSequencePredictionModel {
    sequence_prediction_model: SequencePredictionModel<char>,
}

impl CharacterSequencePredictionModel {
    pub fn train<Input: Into<Data>>(
        input: Input,
        window_size: usize,
    ) -> CharacterSequencePredictionModel {
        CharacterSequencePredictionModel {
            sequence_prediction_model: SequencePredictionModel::train(&input.into().0, window_size),
        }
    }
    pub fn predict<Input: Into<Data>>(&self, input: Input, length: usize) -> String {
        let mut sequence = input.into().0;
        let input_length = sequence.len();
        self.sequence_prediction_model
            .predict(&mut sequence, length);
        sequence.into_iter().skip(input_length).collect()
    }
}

pub struct Data(Vec<char>);

impl From<&str> for Data {
    fn from(text: &str) -> Self {
        Data(text.chars().collect())
    }
}
