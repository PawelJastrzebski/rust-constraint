# Constraint Satisfaction Problem (CSPs)

This crate is ideal for solving scheduling problems, graph coloring, and other CSPs where variables need to be assigned values under specific constraints.

Useful for educational purposes, AI projects, and any application requiring formal logical reasoning.

## Example

In this context, the variables A, B, C, etc., represent different classes that need to be scheduled on specific days of the week (Monday, Tuesday, Wednesday) while ensuring that certain classes do not occur on the same day.

```rust
use rust_constraint::prelude::*;

let mut problem: Problem<&str> = Problem::new().variables(
    vec!["A", "B", "C", "D", "E", "F", "G"],
    vec!["Monday", "Tuesday", "Wednesday"],
);

let constraints = vec![
    ("A", "B"),
    ("A", "C"),
    ("B", "C"),
    ("B", "D"),
    ("B", "E"),
    ("C", "E"),
    ("C", "F"),
    ("D", "E"),
    ("E", "F"),
    ("E", "G"),
    ("F", "G"),
];

for (x, y) in constraints {
    problem.add_constraint_binary(x, y, |x, y| x != y)
}

for solution in problem.get_solutions() {
    println!("{solution}")
}
```
Output
```text
{B: Tuesday, F: Tuesday, G: Wednesday, E: Monday, D: Wednesday, A: Monday, C: Wednesday}
{B: Wednesday, F: Wednesday, G: Tuesday, E: Monday, D: Tuesday, A: Monday, C: Tuesday}
{B: Monday, F: Monday, G: Wednesday, E: Tuesday, D: Wednesday, A: Tuesday, C: Wednesday}
{B: Wednesday, F: Wednesday, G: Monday, E: Tuesday, D: Monday, A: Tuesday, C: Monday}
{B: Monday, F: Monday, G: Tuesday, E: Wednesday, D: Tuesday, A: Wednesday, C: Tuesday}
{B: Tuesday, F: Tuesday, G: Monday, E: Wednesday, D: Monday, A: Wednesday, C: Monday}
```
Source: [CS50](https://youtu.be/qK46ET1xk2A?si=i0IjHglUAPOF87FD&t=3058)


## Acknowledgments

Inspired by: [python-constraint](https://pypi.org/project/python-constraint/) used in CS50’s Introduction to Artificial Intelligence with Python.