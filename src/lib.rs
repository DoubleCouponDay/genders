use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Genders {
    Male,
    Female,
}

impl fmt::Display for Genders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Genders::Male => "Male",
            Genders::Female => "Female",
        };

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equivalence_test() {
        assert_eq!(Genders::Male, Genders::Male);
        assert_eq!(Genders::Female, Genders::Female);
    }

    #[test]
    fn display_test() {
        assert_eq!(String::from("Male"), format!("{}", Genders::Male));
        assert_eq!(String::from("Female"), format!("{}", Genders::Female));
    }
}
