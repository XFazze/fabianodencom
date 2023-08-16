pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Deserialize, Serialize, Properties, PartialEq, Clone)]
pub struct ProjectTag {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Properties, PartialEq, Clone)]
pub struct Project {
    pub name: String,
    pub tags: Vec<String>,
    pub url: String,
    pub description: String,
}
