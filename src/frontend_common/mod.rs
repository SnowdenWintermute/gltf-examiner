use std::collections::HashSet;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PartsByName {
    pub heads: HashSet<String>,
    pub torsos: HashSet<String>,
    pub legs: HashSet<String>,
    pub weapons: HashSet<String>,
}
