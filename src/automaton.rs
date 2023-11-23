use nannou::color::Srgb;
use nannou::prelude::{BLACK, ORANGE, PLUM};
use std::collections::HashMap;

pub struct State {
    color: Srgb<u8>,
    default: i32,
    transitions: HashMap<String, i32>,
}

pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    pub fn game_of_life() -> Automaton {
        Automaton {
            states: vec![
                State {
                    color: BLACK,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map.insert(String::from("53"), 1);
                        map
                    },
                },
                State {
                    color: PLUM,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map.insert(String::from("53"), 1);
                        map.insert(String::from("62"), 1);
                        map
                    },
                },
            ],
        }
    }

    pub fn seeds() -> Automaton {
        Automaton {
            states: vec![
                State {
                    color: BLACK,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map.insert(String::from("62"), 1);
                        map
                    },
                },
                State {
                    color: PLUM,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map
                    },
                },
            ],
        }
    }

    pub fn bb() -> Automaton {
        Automaton {
            states: vec![
                State {
                    color: BLACK,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map.insert(String::from("026"), 1);
                        map.insert(String::from("125"), 1);
                        map.insert(String::from("224"), 1);
                        map.insert(String::from("323"), 1);
                        map.insert(String::from("422"), 1);
                        map.insert(String::from("521"), 1);
                        map.insert(String::from("620"), 1);
                        map
                    },
                },
                State {
                    color: ORANGE,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map
                    },
                },
                State {
                    color: PLUM,
                    default: 0,
                    transitions: {
                        let mut map = HashMap::new();
                        map
                    },
                },
            ],
        }
    }
}
