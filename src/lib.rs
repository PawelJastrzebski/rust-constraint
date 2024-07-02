#![doc = include_str!("../README.md")]

pub mod core;

pub mod prelude {
    pub use crate::core::*;
}

#[cfg(test)]
pub mod examples {
    use crate::prelude::*;

    #[test]
    fn test() {
        let p: Problem<i32> = Problem::new()
            .variable("a", vec![1, 2, 3])
            .variable("b", vec![3, 4, 5, 6])
            .constraint_unary("a", |a| *a != 1)
            .constraint_unary("a", |a| *a != 2)
            .constraint_binary("a", "b", |a, b| a != b);

        println!("{:?}", p.get_solutions());
        println!("{}", p.get_solutions()[0]);

        let p: Problem<i32> = Problem::new()
            .variable("a", (0..100).into_iter().collect())
            .variable("b", (0..100).into_iter().collect())
            .constraint_binary("a", "b", |a, b| 5 * a + 2 * b <= 20)
            .constraint_binary("a", "b", |a, b| 10 * a + 12 * b >= 90);

        println!("{:?}", p.get_solutions());
        println!("{}", p.get_solutions()[0]);
    }

    #[test]
    fn cs50_example() {
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
    }
}
