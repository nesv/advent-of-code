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
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        let mem: Vec<i64> = s
            .split(",")
            .map(|v| {
                let n: i64 = v.parse().unwrap();
                n
            })
            .collect();
        Self { mem, input: None }
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
        }
    }
}

impl From<Vec<i64>> for Program {
    fn from(v: Vec<i64>) -> Self {
        Self {
            mem: v,
            input: None,
        }
    }
}

impl Program {
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
    fn get_input(&mut self) -> Option<i64> {
        if let Some(input) = &mut self.input {
            return Some(input.remove(0));
        }
        None
    }

    /// Execute runs the program the computer was loaded with, and returns
    /// the final memory representation of the program, along with anything
    /// that was output.
    pub fn execute(&mut self) -> Result<(Vec<i64>, Vec<i64>)> {
        let mut mem = self.mem.clone();
        let mut output: Vec<i64> = vec![];
        let mut ip: usize = 0; // instruction pointer
        loop {
            // NOTE(nesv): mem[ip] points at the current instruction.
            let nparams = Instruction::num_params(mem[ip]);
            if ip + nparams >= mem.len() {
                break;
            }

            let params: Vec<i64> = mem[ip..ip + nparams + 1].to_vec();
            let inst = Instruction::parse(params);
            //eprintln!("{:?}", &mem);
            //eprintln!("{: <8}{:?}", &ip, &inst);
            match inst {
                Instruction::Add(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    mem[op] = a + b;
                }

                Instruction::Multiply(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    mem[op] = a * b;
                }

                Instruction::Input(op) => {
                    if let Some(v) = self.get_input() {
                        mem[op] = v;
                    } else {
                        return Err(Error::new(ErrorKind::InvalidInput, "no input available"));
                    }
                }

                Instruction::Output(p) => {
                    let val = Self::resolve_param(&mem, &p);
                    output.push(val);
                }

                Instruction::JumpIfTrue(n, v) => {
                    let a = Self::resolve_param(&mem, &n);
                    if a != 0 {
                        let b = Self::resolve_param(&mem, &v);
                        ip = b as usize;
                        continue;
                    }
                }

                Instruction::JumpIfFalse(n, v) => {
                    let a = Self::resolve_param(&mem, &n);
                    if a == 0 {
                        let b = Self::resolve_param(&mem, &v);
                        ip = b as usize;
                        continue;
                    }
                }

                Instruction::LessThan(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    if a < b {
                        mem[op] = 1;
                    } else {
                        mem[op] = 0;
                    }
                }

                Instruction::Equals(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    if a == b {
                        mem[op] = 1;
                    } else {
                        mem[op] = 0;
                    }
                }

                Instruction::HCF => {
                    break;
                }

                Instruction::BadInput(v) => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("{:?}", v)));
                }
            };

            ip += nparams + 1;
        }

        Ok((mem, output))
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
    fn resolve_param(mem: &Vec<i64>, param: &Parameter) -> i64 {
        match param {
            Parameter::Positional(i) => mem[*i],
            Parameter::Immediate(n) => *n,
            Parameter::UnexpectedMode(m) => {
                panic!("unexpected mode: {}", m);
            }
        }
    }

    /// Set the value of memory at `position` to `value`.
    /// If `position` is not within the range of the currently-allocated memory,
    /// an error is returned.
    pub fn set_mem(&mut self, position: usize, value: i64) -> Result<()> {
        if position >= self.mem.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "position is out of range",
            ));
        }
        self.mem[position] = value;
        Ok(())
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Input(usize),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),

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
            let mode = (v[0] / 10_i64.pow(i as u32 + 2)) % 10;
            let param = match mode {
                0 => Parameter::Positional(v[i + 1] as usize),
                1 => Parameter::Immediate(v[i + 1]),
                _ => Parameter::UnexpectedMode(mode),
            };
            params.push(param);
        }

        // Parse the final argument as an output pointer.
        // Enough instructions use this to know which memory location to store
        // their output, that it makes sense to parse it all the time.
        let optr: usize = v[v.len() - 1] as usize;

        // Finally, return an Instruction.
        match Instruction::ones(v[0]) {
            1 => Self::Add(params[0], params[1], optr),
            2 => Self::Multiply(params[0], params[1], optr),
            3 => Self::Input(optr),
            4 => Self::Output(params[0]),
            5 => Self::JumpIfTrue(params[0], params[1]),
            6 => Self::JumpIfFalse(params[0], params[1]),
            7 => Self::LessThan(params[0], params[1], optr),
            8 => Self::Equals(params[0], params[1], optr),
            9 => Self::HCF,
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
#[derive(Clone, Copy, Debug)]
enum Parameter {
    Positional(usize),
    Immediate(i64),

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
    assert_eq!(Instruction::num_params(99), 0);
}

#[test]
fn test_eq_8_positional() {
    // Output 1 if the input is equal to 8, or 0 if it is not.
    let code = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let mut program = Program::from(code);

    let (_, out) = program.input(8).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let (_, out) = program.input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_lt_8_positional() {
    // Output 1 if the input is less than 8, or 0 otherwise.
    let code = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let mut program = Program::from(code);

    let (_, out) = program.input(7).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let (_, out) = program.input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_eq_8_immediate() {
    // Output 1 if the input is equal to 8, or 0 otherwise.
    let code = String::from("3,3,1108,-1,8,3,4,3,99");
    let mut program = Program::from(code);

    let (_, out) = program.input(8).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let (_, out) = program.input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_lt_8_immediate() {
    // Output 1 if the input is less than 8, or 0 otherwise.
    let code = String::from("3,3,1107,-1,8,3,4,3,99");
    let mut program = Program::from(code);

    let (_, out) = program.input(7).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);

    let (_, out) = program.input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);
}

#[test]
fn test_jump_positional() {
    // Output 0 if the input was 0, otherwise return 1.
    let code = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let mut program = Program::from(code);

    let (_, out) = program.input(0).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);

    let (_, out) = program.input(88).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 1);
}

#[test]
fn test_jump_immediate() {
    // Output 0 if the input was 0, otherwise return 1.
    let code = String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    let mut program = Program::from(code);

    let (_, out) = program.input(0).execute().unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], 0);

    let (_, out) = program.input(88).execute().unwrap();
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
    let mut program = Program::from(code);

    let (_mem, output) = program.input(7).execute().unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 999);

    let (_mem, output) = program.input(8).execute().unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 1000);

    let (_mem, output) = program.input(9).execute().unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 1001);
}
