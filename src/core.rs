use std::{collections::HashMap, fmt::Display};

// Problem
pub struct Problem<T = String>
where
    T: Clone,
{
    varaibles: Vec<Variable<T>>,
    constraints: Vec<Constraint<T>>,
    solver: Solver,
}

#[derive(Debug)]
pub struct Solution<T>(HashMap<String, T>);

impl<T> Solution<T> {
    pub fn get_map(&self) -> &HashMap<String, T> {
        return &self.0;
    }
}

impl<T: Display> std::fmt::Display for Solution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs = self
            .0
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>();
        write!(f, "{{{}}}", pairs.join(", "))
    }
}

impl<T: Clone> Problem<T> {
    pub fn new() -> Problem<T> {
        Problem {
            varaibles: vec![],
            constraints: vec![],
            solver: Solver::default(),
        }
    }

    pub fn solver(mut self, solver: Solver) -> Self {
        self.set_solver(solver);
        self
    }

    pub fn set_solver(&mut self, solver: Solver) {
        self.solver = solver
    }

    pub fn variables<V: Into<String>>(mut self, var_names: Vec<V>, domains: Vec<T>) -> Self {
        self.add_variables(var_names, domains);
        self
    }

    pub fn add_variables<V: Into<String>>(&mut self, var_names: Vec<V>, domains: Vec<T>) {
        for var_name in var_names {
            self.add_variable(var_name.into(), domains.clone())
        }
    }

    pub fn variable<V: Into<String>>(mut self, var_name: V, domains: Vec<T>) -> Self {
        self.add_variable(var_name, domains);
        self
    }

    pub fn add_variable<V: Into<String>>(&mut self, var_name: V, domains: Vec<T>) {
        self.varaibles.push(Variable::new(var_name.into(), domains));
    }

    pub fn constraint(mut self, constraint: Constraint<T>) -> Self {
        self.add_constraint(constraint);
        self
    }

    pub fn add_constraint(&mut self, constraint: Constraint<T>) {
        self.constraints.push(constraint);
    }

    pub fn constraint_unary<N: Into<String>>(mut self, var_name: N, test: fn(&T) -> bool) -> Self {
        self.add_constraint_unary(var_name, test);
        self
    }

    pub fn add_constraint_unary<N: Into<String>>(&mut self, var_name: N, test: fn(&T) -> bool) {
        self.add_constraint(Constraint::Unary(var_name.into(), test));
    }

    pub fn constraint_binary<N: Into<String>>(
        mut self,
        var_a: N,
        var_b: N,
        test: fn(&T, &T) -> bool,
    ) -> Self {
        self.add_constraint_binary(var_a, var_b, test);
        self
    }

    pub fn add_constraint_binary<N: Into<String>>(
        &mut self,
        var_a: N,
        var_b: N,
        test: fn(&T, &T) -> bool,
    ) {
        self.add_constraint(Constraint::Binary(var_a.into(), var_b.into(), test))
    }

    pub fn get_solutions(&self) -> Vec<Solution<T>> {
        match self.solver {
            Solver::RecursiveBacktracking => basic_recursive_backtracking(self),
        }
    }
}

// Variable

#[derive(Clone)]
pub struct Variable<T: Clone = String> {
    name: String,
    domains: Vec<T>,
}

impl<T: Clone> Variable<T> {
    pub fn new(name: String, domains: Vec<T>) -> Self {
        Self { name, domains }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn domains(&self) -> &Vec<T> {
        &self.domains
    }
}

impl From<(String, Vec<String>)> for Variable {
    fn from(value: (String, Vec<String>)) -> Self {
        Variable::new(value.0, value.1)
    }
}

impl From<(&str, Vec<&str>)> for Variable {
    fn from(value: (&str, Vec<&str>)) -> Self {
        Variable::new(
            value.0.to_owned(),
            value.1.iter().map(|v| v.to_string()).collect(),
        )
    }
}

// Constraint

pub enum Constraint<T: Clone = String> {
    Unary(String, fn(&T) -> bool),
    Binary(String, String, fn(&T, &T) -> bool),
}
// Solver

#[derive(Default)]
pub enum Solver {
    #[default]
    RecursiveBacktracking,
}

#[derive(Clone)]
pub struct Assingment<T> {
    values: HashMap<String, T>,
}

impl<T: Clone> Assingment<T> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn is_consistent(&self, problem: &Problem<T>) -> bool {
        for con in problem.constraints.iter() {
            match con {
                Constraint::Unary(a, unary_test) => {
                    let Some(value_a) = self.values.get(a) else {
                        continue;
                    };

                    if !unary_test(&value_a) {
                        return false;
                    }
                }
                Constraint::Binary(a, b, binary_test) => {
                    let Some(value_a) = self.values.get(a) else {
                        continue;
                    };
                    let Some(value_b) = self.values.get(b) else {
                        continue;
                    };

                    if !binary_test(&value_a, &value_b) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn is_complete(&self, problem: &Problem<T>) -> bool {
        for var in problem.varaibles.iter() {
            if !self.values.contains_key(var.name()) {
                return false;
            }
        }
        true
    }

    pub fn select_variable(&self, problem: &Problem<T>) -> Option<Variable<T>> {
        for var in problem.varaibles.iter() {
            if !self.values.contains_key(var.name()) {
                return Some(var.clone());
            }
        }
        None
    }

    pub fn insert(&mut self, name: &str, value: &T) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn remove(&mut self, name: &str) {
        self.values.remove(name);
    }

    pub fn solution(self) -> Solution<T> {
        Solution(self.values)
    }
}

fn basic_recursive_backtracking<T: Clone>(problem: &Problem<T>) -> Vec<Solution<T>> {
    let mut assigment = Assingment::new();
    let mut solutions = vec![];
    recursive_backtracking(&mut solutions, &mut assigment, &problem);
    solutions
}

fn recursive_backtracking<T: Clone>(
    solutons: &mut Vec<Solution<T>>,
    assigment: &mut Assingment<T>,
    problem: &Problem<T>,
) -> bool {
    let Some(var) = assigment.select_variable(&problem) else {
        solutons.push(assigment.clone().solution());
        return true;
    };

    for value in var.domains() {
        assigment.insert(&var.name, &value);
        if assigment.is_consistent(&problem) {
            recursive_backtracking(solutons, assigment, &problem);
        }
        assigment.remove(&var.name)
    }

    false
}
