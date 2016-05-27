//! The `evaluator` crate provides loop for solving ODE system
use std::collections::HashMap;
use std::boxed::Box;
use std::string::String;

/// `VariablesContext` object contains all used variables
pub type VariablesContext = HashMap<String, f32>;

/// Any function must be `Function` type, to be computed
pub type Function = Fn(&VariablesContext) -> f32;

/// 'Evaluator' struct is containing methods for initial setup and solving
pub struct Evaluator {
    variables: VariablesContext,
    functions: HashMap<String, Box<Function>>,
    /// `history` contains evaluation result
    pub history: Vec<VariablesContext>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            variables: HashMap::new(),
            functions: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Adds new variable to `VariablesContext`
    /// #Example
    ///
    /// ```
    /// use evaluator::Evaluator;
    ///	let mut e = Evaluator::new();
    ///	e.add_start_value("TEST".to_string(), 1f32);
    /// ```
    pub fn add_start_value(&mut self, key: String, value: f32) {
        self.variables.insert(key, value);
    }

    pub fn add_function(&mut self, key: String, value: Box<Function>) {
        self.functions.insert(key, value);
    }

    pub fn next_step(&mut self) {
        let copy = self.variables.clone();
        // for (key, val) in self.variables.iter_mut() {
        //    let func = self.functions.get(key).unwrap();
        //    *val = func(&copy);
        // }
        for (key, func) in self.functions.iter() {
            if let Some(val) = self.variables.get_mut(key) {
                *val = func(&copy);
            }
        }
    }

    pub fn evaluate(&mut self, time_begin: f32, time_end: f32, time_delta: f32) {
        let mut iter = time_begin;
        while iter <= time_end {
            self.next_step();
            iter += time_delta;
            self.history.push(self.variables.clone());
        }
    }

    //TODO: improve logic and make tests
    pub fn get_data_vec(&self, dataset_name: &str) -> Option<Vec<f32>> {
        Some(self.history.iter().map(|ref x| x.get(dataset_name).unwrap().clone()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_test_evaluator() -> Evaluator {
        let func = |data: &VariablesContext| data["TEST"] * 2f32;
        let mut e = Evaluator::new();
        e.add_start_value("TEST".to_string(), 1f32);
        e.add_function("TEST".to_string(), Box::new(func));
        return e;
    }

    #[test]
    fn creation_check() {
        let _e = create_test_evaluator();
    }

    #[test]
    fn step_check() {
        let mut e = create_test_evaluator();
        e.next_step();
        assert_eq!(e.variables["TEST"], 2f32);
    }
}
