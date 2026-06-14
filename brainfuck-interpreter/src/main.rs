use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read},
};

enum ProgramError {
    UnmatchedBrackets,
    TapeUnderflow,
    TapeOverflow,
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnmatchedBrackets => write!(f, "unmatched brackets"),
            Self::TapeUnderflow => write!(f, "data pointer moved before start of tape"),
            Self::TapeOverflow => write!(f, "data pointer moved past end of tape"),
        }
    }
}

#[derive(Debug)]
struct Program {
    dp: usize,
    pc: usize,
    tape: Vec<u8>,
    tokens: Vec<u8>,
    loop_map: HashMap<usize, usize>,
}

impl TryFrom<Vec<u8>> for Program {
    type Error = ProgramError;

    fn try_from(program_vec: Vec<u8>) -> Result<Self, Self::Error> {
        let loop_map = get_loop_map(&program_vec)?;
        Ok(Self {
            dp: 0,
            pc: 0,
            tape: [0u8; 30000].to_vec(),
            tokens: program_vec,
            loop_map,
        })
    }
}

impl Program {
    fn execute(&mut self) -> Result<(), ProgramError> {
        while self.pc < self.tokens.len() {
            match self.tokens[self.pc] {
                b'>' => {
                    self.dp += 1;
                    if self.dp >= self.tape.len() {
                        return Err(ProgramError::TapeOverflow);
                    }
                }
                b'<' => {
                    if self.dp == 0 {
                        return Err(ProgramError::TapeUnderflow);
                    }
                    self.dp -= 1;
                }
                b'+' => self.tape[self.dp] = self.tape[self.dp].wrapping_add(1),
                b'-' => self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1),
                b'.' => print!("{}", self.tape[self.dp] as char),
                b'[' => {
                    if self.tape[self.dp] == 0 {
                        self.pc = self.loop_map[&self.pc];
                    }
                }
                b']' if self.tape[self.dp] != 0 => {
                    self.pc = self.loop_map[&self.pc];
                }
                _ => {}
            }
            self.pc += 1;
        }
        Ok(())
    }
}
fn main() {
    let program = match read_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("failed to read program: {e}");
            return;
        }
    };

    let mut program: Program = match program.try_into() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("invalid program: {e}");
            return;
        }
    };

    if let Err(e) = program.execute() {
        eprintln!("runtime error: {e}");
    }
}

fn read_program() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    match env::args().nth(1) {
        Some(path) => {
            File::open(path)?.read_to_end(&mut buf)?;
        }
        None => {
            io::stdin().read_to_end(&mut buf)?;
        }
    }
    Ok(buf)
}

fn get_loop_map(tokens: &[u8]) -> Result<HashMap<usize, usize>, ProgramError> {
    let mut stack = Vec::new();
    let mut map = HashMap::new();

    for (i, token) in tokens.iter().enumerate() {
        match token {
            b'[' => {
                stack.push(i);
            }
            b']' => {
                let open = stack.pop().ok_or(ProgramError::UnmatchedBrackets)?;
                map.insert(open, i);
                map.insert(i, open);
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        Err(ProgramError::UnmatchedBrackets)
    } else {
        Ok(map)
    }
}
