use std::io::{Error, ErrorKind, Result};

/// An [intcode](https://adventofcode.com/2019/day/2) program.
///
/// The way to initialize a `Program` is to use one of the `from` methods,
/// which have been implemented for the following types:
///
/// * `&str`
/// * `String`
/// * `Vec<String>`
/// * `Vec<i64>`
///
/// The implementations for `&str` and `String` expect the same input.
/// The input for these intcode puzzles should be a single line, of
/// comma-separated numbers, with any trailing newline characters removed.
///
/// The implementation for `Vec<String>` is expected to be the input, split on
/// each "," character.
pub struct Program {
    // Initial state of the program.
    mem: Vec<i64>,

    // Any input to the program.
    input: Option<Vec<i64>>,

    // The relative base for relative parameters.
    // By default, the relative base is zero.
    rel_base: usize,

    // Instruction pointer.
    ip: usize,

    // The reason the program has stopped running.
    reason: Option<Stop>,
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        Self {
            mem: Self::mem_from_str(s),
            input: None,
            rel_base: 0,
            ip: 0,
            reason: None,
        }
    }
}

impl From<String> for Program {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<Vec<String>> for Program {
    fn from(v: Vec<String>) -> Self {
        Self {
            mem: v
                .iter()
                .map(|s| {
                    let n: i64 = s.parse().unwrap();
                    n
                })
                .collect(),
            input: None,
            rel_base: 0,
            ip: 0,
            reason: None,
        }
    }
}

impl From<Vec<i64>> for Program {
    fn from(v: Vec<i64>) -> Self {
        Self {
            mem: v,
            input: None,
            rel_base: 0,
            ip: 0,
            reason: None,
        }
    }
}

impl Program {
    fn mem_from_str(s: &str) -> Vec<i64> {
        s
            .split(",")
            .map(|v| {
                let n: i64 = v.parse().unwrap();
                n
            })
            .collect()
    }

    /// Set the input for the program to `v`.
    /// This is typically called before `execute`.
    pub fn input(&mut self, v: i64) -> &mut Self {
        if let Some(input) = &mut self.input {
            input.push(v);
            return self;
        }

        let input: Vec<i64> = vec![v];
        self.input = Some(input);
        self
    }

    /// Consumes the first input value provided via the `input` method.
    /// Successive calls to `get_input` will drain any input provided to the
    /// program before `execute` or `execute_mut` were called.
    fn take_input(&mut self) -> Option<i64> {
        if let Some(input) = &mut self.input {
            if input.len() == 0 {
                return None;
            }
            return Some(input.remove(0));
        }
        None
    }

    /// Returns the next available value for input, or `None` if there is
    /// no input available.
    fn peek_input(&self) -> Option<i64> {
        if let Some(input) = &self.input {
            if input.len() == 0 {
                return None;
            }
            return Some(input[0]);
        }
        None
    }

    /// Return the current instruction at the instruction pointer,
    /// and the position of the next instruction.
    fn peek_instruction(&self) -> (Instruction, usize) {
        // Figure out how many instructions our current instruction takes,
        // so we know how far ahead we have to scan, to get to the next
        // instruction.
        let next = self.ip + Instruction::num_params(self.mem[self.ip]) + 1;
        let params = self.mem[self.ip..next].to_vec();
        (Instruction::parse(params), next)
    }

    /// Return the next instruction, advancing the instruction pointer.
    fn take_instruction(&mut self) -> Instruction {
        let (inst, next) = self.peek_instruction();
        self.ip = next;
        inst
    }

    /// Execute runs the program the computer was loaded with, and returns
    /// the program's output.
    pub fn execute(&mut self) -> Result<Vec<i64>> {
        let mut output: Vec<i64> = vec![];
        loop {
            // If our next instruction is an Input, but we do not have any
            // input, halt here, so that the caller can provide some.
            let (inst, _) = self.peek_instruction();
            //eprintln!("{: >8}: {:?}", &self.ip, &inst);
            match inst {
                Instruction::Input(_p) => match self.peek_input() {
                    None => { 
                        self.reason = Some(Stop::WaitingForInput);
                        return Ok(output);
                    }
                    _ => {}
                }
                _ => {}
            }

            // Take the next instruction.
            let inst = self.take_instruction();
            match inst {
                Instruction::Add(n, v, op) => {
                    let a = self.resolve_param(&n);
                    let b = self.resolve_param(&v);
                    let op = self.resolve_out_ptr(&op);
                    if op >= self.mem.len() {
                        self.mem.resize(op + 1, 0);
                    }
                    self.mem[op] = a + b;
                }

                Instruction::Multiply(n, v, op) => {
                    let a = self.resolve_param(&n);
                    let b = self.resolve_param(&v);
                    let op = self.resolve_out_ptr(&op);
                    if op >= self.mem.len() {
                        self.mem.resize(op + 1, 0);
                    }

                    self.mem[op] = a * b;
                }

                Instruction::Input(p) => {
                    // We need to resolve the parameter differently than
                    // `resolve_param`, since the value we want is what's
                    // contained in `p`, and not resolved by a location in
                    // `self.mem`.
                    let op = self.resolve_out_ptr(&p);
                    if let Some(v) = self.take_input() {
                        self.set_mem(op, v)?;
                    }
                }

                Instruction::Output(p) => {
                    let val = self.resolve_param(&p);
                    output.push(val);
                }

                Instruction::JumpIfTrue(n, v) => {
                    let a = self.resolve_param(&n);
                    if a != 0 {
                        let b = self.resolve_param(&v);
                        self.ip = b as usize;
                    }
                }

                Instruction::JumpIfFalse(n, v) => {
                    let a = self.resolve_param(&n);
                    if a == 0 {
                        let b = self.resolve_param(&v);
                        self.ip = b as usize;
                    }
                }

                Instruction::LessThan(n, v, op) => {
                    let a = self.resolve_param(&n);
                    let b = self.resolve_param(&v);
                    let op = self.resolve_out_ptr(&op);
                    if a < b {
                        self.set_mem(op, 1)?;
                    } else {
                        self.set_mem(op, 0)?;
                    }
                }

                Instruction::Equals(n, v, op) => {
                    let a = self.resolve_param(&n);
                    let b = self.resolve_param(&v);
                    let op = self.resolve_out_ptr(&op);
                    if a == b {
                        self.set_mem(op, 1)?;
                    } else {
                        self.set_mem(op, 0)?;
                    }
                }

                Instruction::RelativeBaseOffset(p) => {
                    let n = self.resolve_param(&p);
                    self.rel_base = ((self.rel_base as i64) + n) as usize;
                }

                Instruction::HCF => {
                    self.reason = Some(Stop::HCF);
                    return Ok(output);
                }

                Instruction::BadInput(v) => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("{:?}", v)));
                }
            }
        }
    }

    // Returns the value for the given param.
    //
    // If `param` is a `Parameter::Positional`, the returned value will be the
    // value held in `mem` at the position indicated by `param`.
    // If `param` is a `Parameter::Immediate`, that value is returned, without
    // incurring a lookup through `mem`.
    //
    // # Panics
    //
    // This method will panic if `param` is a `Parameter::UnexpectedMode`.
    fn resolve_param(&mut self, param: &Parameter) -> i64 {
        match param {
            Parameter::Positional(i) => {
                if *i >= self.mem.len() {
                    return 0;
                }
                self.mem[*i]
            }
            Parameter::Immediate(n) => *n,
            Parameter::Relative(n) => {
                let i = (self.rel_base as i64 + n) as usize;
                if i >= self.mem.len() {
                    self.mem.resize(i + 1, 0);
                }
                self.mem[i]
            }
            Parameter::UnexpectedMode(m) => {
                panic!("unexpected mode: {}", m);
            }
        }
    }

    /// Resolves a parameter as though it is a value indicating where to store
    /// output to.
    ///
    /// # Panics
    ///
    /// If `param` is either `Relative` or `UnexpectedMode`.
    fn resolve_out_ptr(&self, param: &Parameter) -> usize {
        match param {
            Parameter::Positional(u) => *u,
            Parameter::Relative(i) => (self.rel_base as i64 + i) as usize,
            Parameter::Immediate(u) => {
                panic!("resolve immediate output pointer: {}", u);
            }
            Parameter::UnexpectedMode(m) => {
                panic!("unexpected mode: {}", m);
            }
        }
    }

    /// Set the value of memory at `position` to `value`.
    /// If `position` refers to a location outside of the program's
    /// currently-allocated memory, more memory will be allocated.
    /// Any memory between the old boundary and `position` will be filled with
    /// zeroes.
    pub fn set_mem(&mut self, position: usize, value: i64) -> Result<()> {
        if position >= self.mem.len() {
            self.mem.resize(position + 1, 0);
        }
        self.mem[position] = value;
        Ok(())
    }

    /// Resets any internal state, such as the relative base and any input,
    /// and sets the program's initial memory to `code`.
    pub fn reset(&mut self, code: &str) -> &mut Self {
        self.ip = 0;
        self.rel_base = 0;
        self.input = None;
        self.mem = Self::mem_from_str(code);
        self
    }

    // Returns the value of memory at the given position, `pos`.
    pub fn peek_mem(&self, pos: usize) -> i64 {
        if pos >= self.mem.len() {
            return 0;
        }
        self.mem[pos]
    }

    pub fn reason_for_stop(&self) -> Option<Stop> {
        self.reason
    }
}

#[derive(Copy, Clone)]
pub enum Stop {
    WaitingForInput,
    HCF,
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    RelativeBaseOffset(Parameter),

    /// Program stop.
    HCF,

    /// BadInput is returned by `Instruction::parse` when there are too many,
    /// or too few parameters provided.
    BadInput(Vec<i64>),
}

impl Instruction {
    /// Parse the given instruction an parameters in `v`.
    /// The number of parameters for the instruction can be figured out by
    /// calling `Instruction::num_params` on the instruction.
    ///
    /// For example, the short program `1002,4,3,4,33`, is parsed like so:
    ///
    /// ```ignore
    /// ABCDE
    ///  1002,4,3,4
    /// ```
    ///
    /// `DE` (the last two digits of the instruction) indicate this is a
    /// multiplication instruction.
    /// `C`  indicates the first parameter (4) is in positional mode.
    /// `B` indicates the second parameter (3) is in immediate mode.
    /// `A` is ommitted because it is a leading zero, infers positional mode
    /// for the final parameter of the instruction (4).
    ///
    /// In plain English, this multiply instruction would multiply the value
    /// at program position 4, with the immediate value 3, and
    /// store the result in position 4.
    fn parse(v: Vec<i64>) -> Self {
        let np = Self::num_params(v[0]);
        if np + 1 != v.len() {
            return Self::BadInput(v);
        }

        // Collect our parameters.
        let mut params: Vec<Parameter> = vec![];
        for i in 0..np {
            // Figure out the "mode" for the current parameter we are consuming.
            // 0 => (default) Positional mode.
            // 1 => Immediate mode.
            // 2 => Relative mode.
            let mode = (v[0] / 10_i64.pow(i as u32 + 2)) % 10;
            let param = match mode {
                0 => Parameter::Positional(v[i + 1] as usize),
                1 => Parameter::Immediate(v[i + 1]),
                2 => Parameter::Relative(v[i + 1]),
                _ => Parameter::UnexpectedMode(mode),
            };
            params.push(param);
        }

        // Parse the final argument as an output pointer.
        // Enough instructions use this to know which memory location to store
        // their output, that it makes sense to parse it all the time.
        //let optr: usize = v[v.len() - 1] as usize;

        // Finally, return an Instruction.
        match Instruction::ones(v[0]) {
            1 => Self::Add(params[0], params[1], params[2]),
            2 => Self::Multiply(params[0], params[1], params[2]),
            3 => Self::Input(params[0]),
            4 => Self::Output(params[0]),
            5 => Self::JumpIfTrue(params[0], params[1]),
            6 => Self::JumpIfFalse(params[0], params[1]),
            7 => Self::LessThan(params[0], params[1], params[2]),
            8 => Self::Equals(params[0], params[1], params[2]),
            9 => match Instruction::tens(v[0]) {
                0 => Self::RelativeBaseOffset(params[0]),
                9 => Self::HCF,
                _ => Self::BadInput(v),
            },
            _ => Self::BadInput(v),
        }
    }

    // Returns the number of parameters that should be consumed by the opcode.
    fn num_params(opcode: i64) -> usize {
        if opcode == 99 {
            return 0;
        }

        match Instruction::ones(opcode) {
            1 => 3, // addition
            2 => 3, // multiplication
            3 => 1, // input
            4 => 1, // output
            5 => 2, // jump-if-true
            6 => 2, // jump-if-false
            7 => 3, // less-than
            8 => 3, // equals
            9 => 1, // relative base offset
            _ => {
                panic!("Bad opcode {}", opcode);
            }
        }
    }

    fn ones(n: i64) -> i64 {
        n % 10
    }

    fn tens(n: i64) -> i64 {
        (n / 10) % 10
    }
}

/// Parameter is an input for an instruction.
/// Positional mode is the default, which means the value for the given
/// parameter must be looked up at the given memory address/position.
/// Immediate mode means the given value is used as-is (it is not a reference).
///
/// The `Relative` parameter holds a number indicating the "relative base" for
/// all relative parameters.
/// By default, the relative base is 0 (zero).
/// The relative base should be added to whatever the current relative base is.
#[derive(Clone, Copy, Debug)]
enum Parameter {
    Positional(usize),
    Immediate(i64),
    Relative(i64),

    // Indicates the parameter has an unexpected mode.
    UnexpectedMode(i64),
}

#[test]
fn test_num_params() {
    assert_eq!(Instruction::num_params(1), 3);
    assert_eq!(Instruction::num_params(2), 3);
    assert_eq!(Instruction::num_params(3), 1);
    assert_eq!(Instruction::num_params(4), 1);
    assert_eq!(Instruction::num_params(5), 2);
    assert_eq!(Instruction::num_params(6), 2);
    assert_eq!(Instruction::num_params(7), 3);
    assert_eq!(Instruction::num_params(8), 3);
    assert_eq!(Instruction::num_params(9), 1);
    assert_eq!(Instruction::num_params(99), 0);
}

#[test]
fn test_eq_8_positional() {
    // Output 1 if the input is equal to 8, or 0 if it is not.
    let code = String::from("3,9,8,9,10,9,4,9,99,-1,8");

    let mut program = Program::from(code.as_str());
    let out = program.input(8).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let out = program.reset(code.as_str()).input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_lt_8_positional() {
    // Output 1 if the input is less than 8, or 0 otherwise.
    let code = String::from("3,9,7,9,10,9,4,9,99,-1,8");

    let mut program = Program::from(code.as_str());
    let out = program.input(7).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let out = program.reset(code.as_str()).input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_eq_8_immediate() {
    // Output 1 if the input is equal to 8, or 0 otherwise.
    let code = String::from("3,3,1108,-1,8,3,4,3,99");

    let mut program = Program::from(code.as_str());
    let out = program.input(8).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let out = program.reset(code.as_str()).input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_lt_8_immediate() {
    // Output 1 if the input is less than 8, or 0 otherwise.
    let code = String::from("3,3,1107,-1,8,3,4,3,99");

    let mut program = Program::from(code.as_str());
    let out = program.input(7).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let out = program.reset(code.as_str()).input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_jump_positional() {
    // Output 0 if the input was 0, otherwise return 1.
    let code = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");

    let mut program = Program::from(code.as_str());
    let out = program.input(0).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);

    let out = program.reset(code.as_str()).input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);
}

#[test]
fn test_jump_immediate() {
    // Output 0 if the input was 0, otherwise return 1.
    let code = String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    let mut program = Program::from(code.as_str());

    let out = program.input(0).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);

    let out = program.reset(code.as_str()).input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);
}

#[test]
fn test_large_example() {
    // This program should return
    //
    // * 999 if the input is less-than 8;
    // * 1000 if the input is equal to 8;
    // * 1001 if the input is greater-than 8.
    //
    let code = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let mut program = Program::from(code.as_str());

    let output = program.input(7).execute().unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 999);

    let output = program.reset(code.as_str()).input(8).execute().unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 1000);

    let output = program.reset(code.as_str()).input(9).execute().unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 1001);
}

#[test]
fn test_boost_self() {
    // The output should be the initial program.
    let code = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut program = Program::from(code.as_str());

    let out = program.execute().unwrap();
    let out: Vec<String> = out.iter().map(|&n| n.to_string()).collect();
    assert_eq!(code, out.join(","));
}

#[test]
fn test_boost_bignum() {
    // Should output a 16-digit number.
    let code = String::from("1102,34915192,34915192,7,4,7,99,0");
    let mut program = Program::from(code.as_str());

    let out = program.execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].to_string().len(), 16);
}

#[test]
fn test_boost_middle_num() {
    // Should output the big number in the middle.
    let code = String::from("104,1125899906842624,99");
    let mut program = Program::from(code.as_str());
    let out = program.execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1125899906842624);
}
