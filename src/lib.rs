use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
        };

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equivalence_test() {
        assert_eq!(Gender::Male, Gender::Male);
        assert_eq!(Gender::Female, Gender::Female);
    }

    #[test]
    fn display_test() {
        assert_eq!(String::from("Male"), format!("{}", Gender::Male));
        assert_eq!(String::from("Female"), format!("{}", Gender::Female));
    }
}
