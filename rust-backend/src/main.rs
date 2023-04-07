use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_std::task;
use regex::Regex;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Debug,
    fs::File,
    io::{Error, Read, Write},
    time::Instant,
};
use tide::{http::mime, Body, Response, StatusCode};

struct QueryRoot;

#[Object]
impl QueryRoot {
    // async fn add(&self, a: i32, b: i32) -> i32 {
    //     a + b
    // }
    async fn read(
        &self,
        text_input_file_path: String,
        slice: Option<usize>,
        csv_output_file_path: Option<String>,
        json_output_file_path: Option<String>,
    ) -> Vec<PatternResult> {
        let data_load_now = Instant::now();
        let string = read_file_to_string(&text_input_file_path).expect("could not read file");
        let data: Vec<char> = string.chars().take(slice.unwrap_or(string.len())).collect();
        let data_load_duration = data_load_now.elapsed().as_secs();
        let data_length = data.len();
        let pattern_creation_now = Instant::now();
        let mut patterns: HashSet<Pattern> = HashSet::new();
        for index in 0..data_length {
            if let Some(current_character) = data.get(index) {
                patterns.insert(Pattern::CurrentCharacterIs {
                    current_character: *current_character,
                });
                if let Some(next_character) = data.get(index + 1) {
                    patterns.insert(Pattern::NextCharacterIs {
                        current_character: *current_character,
                        next_character: *next_character,
                    });
                    if let Some(next_next_character) = data.get(index + 2) {
                        patterns.insert(Pattern::NextCharacterAfterTwo {
                            current_character_1: *current_character,
                            current_character_2: *next_character,
                            next_character: *next_next_character,
                        });
                    }
                }
                if index > 0 {
                    if let Some(previous_character) = data.get(index - 1) {
                        patterns.insert(Pattern::PreviousCharacterIs {
                            current_character: *current_character,
                            previous_character: *previous_character,
                        });
                    }
                }
            }
        }
        let pattern_creation_duration = pattern_creation_now.elapsed().as_secs();
        let pattern_stats_now = Instant::now();
        let mut pattern_stats: HashMap<Pattern, PatternStats> = HashMap::new();
        for pattern in &patterns {
            let stats = pattern_stats.entry(pattern.clone()).or_default();
            for index in 0..data_length {
                if let Some(data_current_character) = data.get(index) {
                    if let Pattern::CurrentCharacterIs { current_character } = pattern {
                        stats.condition_count += 1;
                        if *current_character == *data_current_character {
                            stats.consequence_count += 1
                        }
                    }
                    if let Some(data_next_character) = data.get(index + 1) {
                        if let Pattern::NextCharacterIs {
                            current_character,
                            next_character,
                        } = pattern
                        {
                            if *current_character == *data_current_character {
                                stats.condition_count += 1;
                                if *next_character == *data_next_character {
                                    stats.consequence_count += 1
                                }
                            }
                        }
                        if let Some(data_next_next_character) = data.get(index + 2) {
                            if let Pattern::NextCharacterAfterTwo {
                                current_character_1,
                                current_character_2,
                                next_character,
                            } = pattern
                            {
                                if *current_character_1 == *data_current_character
                                    && *current_character_2 == *data_next_character
                                {
                                    stats.condition_count += 1;
                                    if *next_character == *data_next_next_character {
                                        stats.consequence_count += 1
                                    }
                                }
                            }
                        }
                    }
                    if index > 0 {
                        if let Some(data_previous_character) = data.get(index - 1) {
                            if let Pattern::PreviousCharacterIs {
                                current_character,
                                previous_character,
                            } = pattern
                            {
                                if *current_character == *data_current_character {
                                    stats.condition_count += 1;
                                    if *previous_character == *data_previous_character {
                                        stats.consequence_count += 1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let pattern_stats_duration = pattern_stats_now.elapsed().as_secs();
        let report_now = Instant::now();
        let mut result: Vec<PatternResult> = Vec::new();
        for (pattern, stats) in &pattern_stats {
            result.push(PatternResult {
                pattern: format!("{:?}", pattern),
                condition_count: stats.condition_count,
                consequence_count: stats.consequence_count,
                accuray: stats.accuracy(),
            })
        }
        result.sort_by(|a, b| b.accuray.partial_cmp(&a.accuray).unwrap());
        if let Some(path) = csv_output_file_path {
            let file = std::fs::File::create(path).expect("culd not create csv file");
            let mut csv_writer = csv::Writer::from_writer(file);
            csv_writer
                .write_record(&[
                    "pattern",
                    "current_character",
                    "next_character",
                    "previous_character",
                    "condition_count",
                    "consequence_count",
                    "accuracy",
                ])
                .expect("could not write csv header");
            for (pattern, stats) in &pattern_stats {
                let pattern_columns: Vec<String> = match pattern {
                    Pattern::CurrentCharacterIs { current_character } => vec![
                        "CurrentCharacterIs".to_string(),
                        current_character.to_string(),
                        "".to_string(),
                        "".to_string(),
                    ],
                    Pattern::NextCharacterIs {
                        current_character,
                        next_character,
                    } => vec![
                        "NextCharacterIs".to_string(),
                        current_character.to_string(),
                        next_character.to_string(),
                        "".to_string(),
                    ],
                    Pattern::PreviousCharacterIs {
                        current_character,
                        previous_character,
                    } => vec![
                        "PreviousCharacterIs".to_string(),
                        current_character.to_string(),
                        "".to_string(),
                        previous_character.to_string(),
                    ],
                    Pattern::NextCharacterAfterTwo {
                        current_character_1,
                        current_character_2,
                        next_character,
                    } => vec![
                        "NextCharacterAfterTwo".to_string(),
                        format!("{current_character_1}{current_character_2}").to_string(),
                        next_character.to_string(),
                        "".to_string(),
                    ],
                };
                let stat_columns = vec![
                    stats.condition_count.to_string(),
                    stats.consequence_count.to_string(),
                    stats.accuracy().to_string(),
                ];
                csv_writer
                    .write_record(&[pattern_columns, stat_columns].concat())
                    .expect("could not write csv line");
            }
            csv_writer.flush().expect("could not write csv file");
        }
        if let Some(path) = json_output_file_path {
            let mut file = std::fs::File::create(path).expect("culd not create json file");
            let result: Vec<(&Pattern, &PatternStats)> = pattern_stats.iter().collect();
            let string = serde_json::to_string(&result).expect("could not serialize json file");
            file.write_all(string.as_bytes())
                .expect("could not write json file");
        }
        let report_duration = report_now.elapsed().as_secs();
        println!(
            "load: {data_load_duration} create: {pattern_creation_duration} check: {pattern_stats_duration} report: {report_duration}",
        );
        result
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
enum Pattern {
    CurrentCharacterIs {
        current_character: char,
    },
    NextCharacterIs {
        current_character: char,
        next_character: char,
    },
    PreviousCharacterIs {
        current_character: char,
        previous_character: char,
    },
    NextCharacterAfterTwo {
        current_character_1: char,
        current_character_2: char,
        next_character: char,
    },
}

#[derive(Default, Serialize, Clone, Copy)]
struct PatternStats {
    condition_count: u32,
    consequence_count: u32,
}

impl PatternStats {
    fn accuracy(&self) -> f32 {
        return self.consequence_count as f32 / self.condition_count as f32;
    }
}

#[derive(SimpleObject, Serialize)]
struct PatternResult {
    pattern: String,
    condition_count: u32,
    consequence_count: u32,
    accuray: f32,
}

type TideResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> TideResult<()> {
    task::block_on(run_graphql_server())
}

async fn run_graphql_server() -> TideResult<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());
    let mut app = tide::new();
    app.at("/graphql").post(async_graphql_tide::graphql(schema));
    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(
            GraphiQLSource::build().endpoint("/graphql").finish(),
        ));
        resp.set_content_type(mime::HTML);
        return Ok(resp);
    });
    println!("GraphiQL IDE: http://{}", listen_addr);
    app.listen(listen_addr).await?;
    Ok(())
}

fn read_file_to_string(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    return Ok(text);
}

// fn save_to_file(model: &Model, file_name: &str) -> Result<(), Error> {
//     let mut file = File::create(file_name)?;
//     let text = ron::to_string(&model).expect("could not serialize model");
//     file.write_all(text.as_bytes())?;
//     return Ok(());
// }
// fn load_from_file(file_name: &str) -> Result<Model, Error> {
//     let mut file = File::open(file_name)?;
//     let mut text = String::new();
//     file.read_to_string(&mut text)?;
//     let model = ron::from_str(&text).expect("could not deserialize model");
//     return Ok(model);
// }
