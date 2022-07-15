pub type State<'a> = &'a str;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Symbol {
  Sym(char),
  Blank,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
  Left,
  Right,
  Stay,
}

#[derive(Debug, Clone)]
pub struct Transition<'a> {
  pub read_state: State<'a>,
  pub read_symbol: Symbol,
  pub write_state: State<'a>,
  pub write_symbol: Symbol,
  pub mov: Move,
}

#[derive(Debug)]
pub struct Tm<'a> {
  pub states: Vec<State<'a>>,
  pub alphabet: Vec<Symbol>,
  pub initial_state: State<'a>,
  pub final_states: Vec<State<'a>>,
  pub transitions: Vec<Transition<'a>>,
}

impl<'a> Tm<'a> {
  pub fn new(
    states: Vec<State<'a>>,
    alphabet: &str,
    initial_state: State<'a>,
    final_states: Vec<State<'a>>,
    transitions: Vec<Transition<'a>>,
  ) -> Tm<'a> {
    let alphabet: Vec<Symbol> = alphabet.chars().map(|x| Symbol::Sym(x)).collect();
    Tm {
      states,
      transitions,
      final_states,
      initial_state,
      alphabet,
    }
  }

  pub fn get_transition(&self, state: State, sym: Symbol) -> Option<&Transition> {
    self
      .transitions
      .iter()
      .find(|transition| transition.read_state == state && transition.read_symbol == sym)
  }
}

impl<'a> Transition<'a> {
  pub fn new(
    read_state: State<'a>,
    read_symbol: Symbol,
    write_state: State<'a>,
    write_symbol: Symbol,
    mov: Move,
  ) -> Transition<'a> {
    Transition {
      read_state,
      read_symbol,
      write_state,
      write_symbol,
      mov,
    }
  }
}

#[derive(Debug)]
pub struct Tape {
  tape: Vec<Symbol>,
  index: i16,
  len: usize,
}

impl Tape {
  pub fn new(inp: &str) -> Tape {
    Tape {
      tape: inp.chars().map(|x| Symbol::Sym(x)).collect(),
      len: inp.len(),
      index: 0,
    }
  }

  pub fn move_right(&mut self) {
    if self.index > self.len as i16 {
      self.tape.push(Symbol::Blank);
      self.len += 1;
    } else {
      self.index += 1;
    }
  }

  pub fn move_left(&mut self) {
    if self.index == 0 {
      self.tape.insert(0, Symbol::Blank);
      self.len += 1;
    } else {
      self.index -= 1;
    }
  }

  pub fn write(&mut self, sym: Symbol) {
    self.tape[self.index as usize] = sym;
  }

  pub fn focus(&self) -> Symbol {
    self.tape[self.index as usize]
  }
}

pub fn execute_tm(tm: Tm, inp: &str) {
  let mut state: State = tm.initial_state;
  let mut tape = Tape::new(inp);

  while let Some(transition) = tm.get_transition(state, tape.focus()) {
    state = transition.write_state;
    tape.write(transition.write_symbol);
    match transition.mov {
      Move::Left => tape.move_left(),
      Move::Right => tape.move_right(),
      _ => {}
    }
  }

  println!("{:?}", tape);
}
