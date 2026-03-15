pub mod search {
    pub enum Pattern {
        Application,
        Equation,
    }

    pub fn pattern_match(match_string: &String) -> Pattern {
        return Pattern::Application;
    }
}
