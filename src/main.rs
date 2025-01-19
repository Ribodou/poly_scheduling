use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use clingo::{Control, Model, Part, ShowType, SolveMode};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    names: HashMap<String, String>,
    relations: Vec<(String, String, i32)>,
}

struct Dates {
    cost: i64,
    data: HashMap<u32, Vec<(String, String)>>,
}

impl Dates {
    fn from_score_and_vec_of_string(cost: i64, vec_of_strings: Vec<String>) -> Self {
        let mut data: HashMap<u32, Vec<(String, String)>> = HashMap::new();
        for s in vec_of_strings {
            assert_eq!(&s[0..5], "date(");
            let len = s.chars().count();
            // TODO assert count "," ?
            assert_eq!(
                s.chars().nth(len - 1).map(String::from),
                Some(String::from(")"))
            );
            let s = &s[5..len - 1];
            let inner_data: Vec<&str> = s.split(",").collect();
            assert_eq!(inner_data.len(), 3);
            let name_a = String::from(inner_data[0]);
            let name_b = String::from(inner_data[1]);
            let day: u32 = inner_data[2].parse().unwrap();
            data.entry(day).or_default().push((name_a, name_b));
        }
        Self { cost, data }
    }

    fn get_sorted_days_and_associated_dates(&self) -> Vec<(u32, Vec<(String, String)>)> {
        let mut days = self.data.keys().copied().collect_vec();
        days.sort_unstable();
        let mut dates: Vec<(u32, Vec<(String, String)>)> = Vec::new();
        for day in &days {
            dates.push((*day, self.data.get(day).unwrap().clone()));
        }
        dates
    }

    fn print(&self, name_table: &HashMap<String, String>) {
        let days_and_dates = self.get_sorted_days_and_associated_dates();
        for (day, dates) in days_and_dates {
            println!("day {}", day);
            println!("    dates:");
            for date in dates {
                let name_a = get_full_name(name_table, &date.0);
                let name_b = get_full_name(name_table, &date.1);
                println!("        {}, {}", name_a, name_b);
            }
        }
    }
}

fn get_full_name(name_table: &HashMap<String, String>, short_name: &str) -> String {
    match name_table.get(short_name) {
        None => String::from(short_name),
        Some(full_name) => {
            format!("{} ({})", full_name, short_name)
        }
    }
}
fn model_to_string(model: &Model) -> Dates {
    // retrieve the cost and the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
        .expect("Failed to retrieve the symbols in the model.");
    let cost = model
        .cost()
        .expect("Failed to retrieve the cost in the model.");

    assert_eq!(cost.len(), 1);
    let cost = cost[0];

    let v: Vec<String> = atoms.iter().map(|x| x.to_string()).collect();

    Dates::from_score_and_vec_of_string(cost, v)
}

fn solve(ctl: Control) -> Option<Dates> {
    // get a solve handle
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models, find the best one
    let mut best_model = None;
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(Some(model)) => {
                best_model = Some(model_to_string(model));
            }
            // stop if there are no more models
            Ok(None) => break,
            Err(e) => panic!("Error: {}", e),
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
    best_model
}

fn build_asp_program(
    scheduler: &str,
    relations_facts: &str,
    dmax: i32,
) -> clingo::GenericControl<clingo::DefaultCtx> {
    // Create a Clingo control object
    let mut control = clingo::control(vec![]).expect("unable to build clingo");

    // Load the ASP program
    control.add("base", &[], scheduler).unwrap();

    control
        .add("base", &[], &format!("#const dmax = {}.", dmax))
        .unwrap();

    // Load the relations
    control.add("base", &[], relations_facts).unwrap();

    control
}
fn main() {
    let contents =
        std::fs::read_to_string("data.yaml").expect("Should have been able to read the file");
    let scheduler = std::fs::read_to_string("src/scheduler.lp")
        .expect("Should have been able to read the scheduler");
    let data: Data = serde_yml::from_str(&contents).unwrap();

    let (names, relations) = (data.names, data.relations);

    let mut relations_facts = String::new();
    for relation in relations {
        let (a, b, w) = relation;
        relations_facts += &format!("relation({}, {}, {}).\n", a, b, w);
    }

    let mut control = build_asp_program(&scheduler, &relations_facts, 7);

    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    control.ground(&parts).unwrap();
    let best_model = solve(control).unwrap();
    best_model.print(&names);
}
