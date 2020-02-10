use std::collections::{HashMap, BTreeSet};

pub struct Nfa {
    // transition[start node][char][outgoing#] = end node
    // Starting state is always node 0
    // Lambda is always char 0
    transitions: Vec<Vec<Vec<usize>>>,
    accepting_states: Vec<usize>,
    character_map:HashMap<usize, char>,
}

impl Nfa {
    pub fn new -> Self {
        todo!;
    }

    pub fn to_dfa(Self) -> dfa::table {
        let mut table = dfa::new();
        let mut statemap: HashMap<BTreeSet<usize>, usize> = HashMap::new();
        let counter = 0;
        let mut L = Vec::new();
        let mut B = BTreeSet::new();
        B.insert(0); // insert starting node
        B = follow_lambda(B);
        L.push(B);
        statemap[B] = counter;
        counter = counter + 1;
        //TODO insert a blank row into table

        while L.len() > 0 {
            let S = L.pop();
            // psuedo code ahead 
        }

        
        todo!
        return table
    }

    /*
    * returns the set of NFA states encountered by
    * recursively following only λ transitions.
    */
    fn follow_lambda(Self, states: BTreeSet) -> BTreeSet<usize> {
        let L = Vec::from(states.iter());
        while L.len() > 0 {
            let t = L.pop();
            for l_tran in Self.transitions[t][0] {
                if (!states.contains(&l_tran)) {
                    states.insert(l_tran);
                    L.push(l_tran);
                }
            }
        }
        states;
    }

    /*
    * returns the set of NFA states obtained from following character c
    * from a set of states.
    */
    fn follow_char(Self, states: BTreeSet, c: usize) -> BTreeSet<usize> {
        let mut follow = BTreeSet::new();
        for state in states {
            for transition in Self.transitions[state][c] {
                follow.insert(transition);
            }
        }
        return follow;
    }
}

impl From<BuffReader> for Nfa {
    todo!
}