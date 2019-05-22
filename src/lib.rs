use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// enum representation of the genders
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    /// slice of genders for iterating over etc.
    const GENDERS: [Gender; 2] = [Gender::Male, Gender::Female];

    /// returns total number of genders
    fn num() -> usize {
        Gender::GENDERS.len()
    }

    /// returns an iterator of all genders
    fn iter() -> impl Iterator<Item = &'static Gender> {
        Gender::GENDERS.iter()
    }
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

impl PartialOrd for Gender {
    fn partial_cmp(&self, other: &Gender) -> Option<Ordering> {
        match (self, other) {
            (Gender::Male, Gender::Male) => Some(Ordering::Equal),
            (Gender::Female, Gender::Female) => Some(Ordering::Equal),
            (Gender::Male, Gender::Female) => Some(Ordering::Greater),
            (Gender::Female, Gender::Male) => Some(Ordering::Less),
        }
    }
}

impl Ord for Gender {
    fn cmp(&self, other: &Gender) -> Ordering {
        self.partial_cmp(other).unwrap()
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

    #[test]
    fn num_genders_test() {
        assert_eq!(2, Gender::GENDERS.len());
    }

    #[test]
    fn ord_test() {
        assert!(Gender::Male > Gender::Female);
        assert!(Gender::Male == Gender::Male);

        let mut genders = Gender::GENDERS;
        genders.sort();

        assert_eq!([Gender::Female, Gender::Male], genders);
    }

    #[test]
    fn iter_test() {
        let mut itered_genders: Vec<Gender> = Vec::new();
        for gender in Gender::iter() {
            itered_genders.push(*gender);
        }

        for i in 0..Gender::num() {
            assert_eq!(Gender::GENDERS[i], itered_genders[i]);
        }
    }
}
