use std::{io::Error, rc::Rc};

fn main() -> Result<(), Error> {
    Ok(())
}

struct Term;

fn check<Scenario>(scenario: &Scenario, term: &Term) -> bool {
    true // TODO
}

fn is_not<Scenario>(scenario: &Scenario, result: &Term, x: &Term) -> bool {
    match (check(scenario, result), check(scenario, x)) {
        (true, true) => false,
        (true, false) => true,
        (false, true) => true,
        (false, false) => false,
    }
}

fn is_and<Scenario>(scenario: &Scenario, result: &Term, x: &Term, y: &Term) -> bool {
    match (
        check(scenario, result),
        check(scenario, x),
        check(scenario, y),
    ) {
        (true, true, true) => true,
        (true, true, false) => false,
        (true, false, true) => false,
        (true, false, false) => false,
        (false, true, true) => false,
        (false, true, false) => true,
        (false, false, true) => true,
        (false, false, false) => true,
    }
}

fn is_or<Scenario>(scenario: &Scenario, result: &Term, x: &Term, y: &Term) -> bool {
    match (
        check(scenario, result),
        check(scenario, x),
        check(scenario, y),
    ) {
        (true, true, true) => true,
        (true, true, false) => true,
        (true, false, true) => true,
        (true, false, false) => false,
        (false, true, true) => true,
        (false, true, false) => true,
        (false, false, true) => true,
        (false, false, false) => false,
    }
}

fn is_not_accuray<Scenario>(
    scenarios: impl Iterator<Item = Scenario>,
    result: &Term,
    x: &Term,
) -> f64 {
    let mut schenario_count: u32 = 0;
    let mut match_count: u32 = 0;
    for scenario in scenarios {
        if is_not(&scenario, result, x) {
            match_count += 1;
        }
        schenario_count += 1;
    }
    match_count as f64 / schenario_count as f64
}

fn is_and_accuracy<Scenario>(
    scenarios: impl Iterator<Item = Scenario>,
    result: &Term,
    x: &Term,
    y: &Term,
) -> f64 {
    let mut schenario_count: u32 = 0;
    let mut match_count: u32 = 0;
    for scenario in scenarios {
        if is_and(&scenario, result, x, y) {
            match_count += 1;
        }
        schenario_count += 1;
    }
    match_count as f64 / schenario_count as f64
}

fn is_or_accuracy<Scenario>(
    scenarios: impl Iterator<Item = Scenario>,
    result: &Term,
    x: &Term,
    y: &Term,
) -> f64 {
    let mut schenario_count: u32 = 0;
    let mut match_count: u32 = 0;
    for scenario in scenarios {
        if is_or(&scenario, result, x, y) {
            match_count += 1;
        }
        schenario_count += 1;
    }
    match_count as f64 / schenario_count as f64
}
