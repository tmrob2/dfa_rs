use std::collections::HashMap;

fn main() {
    // Define a functional hashmap
    let mut transitions: HashMap<u32, Box<Tfunc>> = HashMap::new();
    transitions.insert(0, Box::new(goto_a));
    // Call transitions test
    // let qprime: u32 = transitions.get(&0u32).unwrap()("a");
    println!("transitions from (0, a) -> {:?}", qprime);
    // construct a new DFA
    let test_dfa: DFA<2, 1, 1> = DFA {
        states: [0, 1],
        accepting: [1],
        rejecting: [2],
        transitions
    };
    let qprime: u32 = test_dfa.transitions.get(&0u32).unwrap()("a");
    // because we are going to use value iteration, it will
    // be important that that we do not store a dfa state just
    // whatever the input is
}

// The transition function is a boxed reference to a function
type Tfunc = dyn Fn(&str)-> u32;

#[allow(dead_code)]
struct DFA<const N: usize, const A: usize, const B: usize> {
    states: [u32; N],
    accepting: [u32; A],
    rejecting: [u32; B],
    transitions: HashMap<u32, Box<Tfunc>>
}

impl<const N: usize, const A: usize, const B: usize> DFA<N, A, B> {
    fn find_sinks(&mut self) {
        // Find the sink states for the DFA which are not accepting.
        // Alter the DFA forever.
    }
}

fn goto_a(w: &str) -> u32 {
    if w == "a"{
        1
    } else {
        0
    }
}

