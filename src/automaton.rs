use nannou::color::Srgb;
use nannou::prelude::{BLACK, ORANGE, PLUM};
use std::collections::HashMap;

pub struct State {
    pub color: Srgb<u8>,
    default: usize,
    transitions: HashMap<String, usize>,
}

impl State {
    pub fn next_state(&self, transition: &str) -> usize {
        match self.transitions.get(transition) {
            Some(state) => *state,
            None => self.default,
        }
    }
}

pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn get_state(&self, state: usize) -> Option<&State> {
        self.states.get(state)
    }

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
                    transitions: HashMap::new(),
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
                    transitions: HashMap::new(),
                },
                State {
                    color: PLUM,
                    default: 0,
                    transitions: HashMap::new(),
                },
            ],
        }
    }
}
