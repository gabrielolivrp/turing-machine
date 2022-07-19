pub type State = &'static str;

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
pub struct Transition {
  pub read_state: State,
  pub read_symbol: Symbol,
  pub write_state: State,
  pub write_symbol: Symbol,
  pub mov: Move,
}

#[derive(Debug)]
pub struct Tm {
  pub states: Vec<State>,
  pub alphabet: Vec<Symbol>,
  pub initial_state: State,
  pub final_states: Vec<State>,
  pub transitions: Vec<Transition>,
}

impl Tm {
  pub fn new(
    states: Vec<State>,
    alphabet: &str,
    initial_state: State,
    final_states: Vec<State>,
    transitions: Vec<Transition>,
  ) -> Tm {
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

impl Transition {
  pub fn new(
    read_state: State,
    read_symbol: Symbol,
    write_state: State,
    write_symbol: Symbol,
    mov: Move,
  ) -> Transition {
    Transition {
      read_state,
      read_symbol,
      write_state,
      write_symbol,
      mov,
    }
  }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TmResult {
  tape: Tape,
  accepted: bool,
  state: State,
}

impl TmResult {
  pub fn new(tape: Tape, state: State) -> TmResult {
    TmResult {
      tape,
      state,
      accepted: false,
    }
  }
}

pub fn execute_tm<'a>(tm: Tm, inp: &'a str) -> TmResult {
  let tape = Tape::new(inp);
  let mut result = TmResult::new(tape, tm.initial_state);

  while let Some(transition) = tm.get_transition(result.state, result.tape.focus()) {
    result.state = transition.write_state;
    result.tape.write(transition.write_symbol);
    match transition.mov {
      Move::Left => result.tape.move_left(),
      Move::Right => result.tape.move_right(),
      _ => {}
    }
  }

  if tm.final_states.contains(&result.state) {
    result.accepted = true;
  }

  result
}
