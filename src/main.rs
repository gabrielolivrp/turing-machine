mod tm;

use tm::*;

fn main() {
  let states = vec!["q1", "q2", "q3", "q4", "q5"];
  let alphabet = "ab";
  let initial_state = "q1";
  let final_states = vec!["q5"];

  let transitions = vec![
    Transition::new("q1", Symbol::Sym('a'), "q2", Symbol::Sym('a'), Move::Left),
    Transition::new("q2", Symbol::Blank, "q3", Symbol::Blank, Move::Left),
    Transition::new("q3", Symbol::Blank, "q4", Symbol::Blank, Move::Left),
    Transition::new("q4", Symbol::Blank, "q5", Symbol::Sym('b'), Move::Stay),
  ];

  let tm = Tm::new(states, alphabet, initial_state, final_states, transitions);

  let result = execute_tm(tm, "ab");
  println!("{:?}", result);
}
