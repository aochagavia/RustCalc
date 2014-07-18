/*

Implements the environment struct, which keeps track of the user-defined variables and functions
when interpreting multiple commands.

*/

use std::collections::HashMap;
use std::str::Slice;

use super::CalcResult;
use super::function::Function;

pub struct Environment {
    functions: HashMap<String, Function>,
    variables: HashMap<String, f64>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { functions: HashMap::new(), variables: HashMap::new() }
    }

    pub fn set_fn(&mut self, name: &str, function: Function) -> CalcResult<()> {
        let key = name.to_string();
        if self.functions.contains_key(&key) {
            Err(Slice("The function already exists"))
        } else {
            self.functions.insert(key, function);
            Ok(())
        }
    }

    pub fn set_var(&mut self, name: &str, value: f64) -> CalcResult<()> {
        let key = name.to_string();
        if self.variables.contains_key(&key) {
           Err(Slice("The variable already exists"))
        } else {
            self.variables.insert(key, value);
            Ok(())
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<Function> {
        // FIXME: there should be a better way to do this without allocating a String
        // each time the function is called...
        self.functions.find(&name.to_string()).map(|&f| f)
    }

    pub fn get_var(&self, name: &str) -> Option<f64> {
        // FIXME: there should be a better way to do this without allocating a String
        // each time the function is called...
        self.variables.find(&name.to_string()).map(|&x| x)
    }
}
