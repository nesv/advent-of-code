use std::io::{Error, ErrorKind, Result};

/// An [intcode](https://adventofcode.com/2019/day/2) program.
///
/// The way to initialize a `Program` is to use one of the `from` methods,
/// which have been implemented for the following types:
///
/// * `&str`
/// * `String`
/// * `Vec<String>`
///
/// The implementations for `&str` and `String` expect the same input.
/// The input for these intcode puzzles should be a single line, of
/// comma-separated numbers, with any trailing newline characters removed.
///
/// The implementation for `Vec<String>` is expected to be the input, split on
/// each "," character.
pub struct Program {
    // Initial state of the computer.
    // This would be set by the `Computer::load` function.
    mem: Vec<String>,
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        let mem: Vec<String> = s.split(",").map(|v| v.to_string()).collect();
        Self { mem }
    }
}

impl From<String> for Program {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<Vec<String>> for Program {
    fn from(v: Vec<String>) -> Self {
        Self { mem: v }
    }
}

impl Program {
    /// Load the program from existing memory.
    pub fn load(mem: Vec<String>) -> Result<Self> {
        if mem.len() == 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "no instructions in program"));
        }
        Ok(Self { mem })
    }

    /// Execute runs the program the computer was loaded with, and returns
    /// the final memory representation of the program, along with anything
    /// that was output.
    pub fn execute(&self, input: i64) -> Result<(Vec<String>, Vec<String>)> {
        let mut mem = self.mem.clone();
        let mut output: Vec<String> = vec![];
        let mut ip: usize = 0; // instruction pointer
        loop {
            // NOTE(nesv): mem[ip] points at the current instruction.
            let nparams = Instruction::num_params(&mem[ip]);
            if ip + nparams >= mem.len() {
                break;
            }

            let params: Vec<String> = mem[ip..ip+nparams+1].to_vec();
            let inst = Instruction::parse(params);
            //eprintln!("{:?}", &mem);
            //eprintln!("{: <8}{:?}", &ip, &inst);
            match inst {
                Instruction::Add(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    mem[op] = (a + b).to_string();
                },

                Instruction::Multiply(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    mem[op] = (a * b).to_string();
                },

                Instruction::Input(op) => {
                    mem[op] = input.to_string();
                },

                Instruction::Output(p) => {
                    let val = Self::resolve_param(&mem, &p);
                    output.push(val.to_string());
                    //println!("{}", val);
                },

                Instruction::JumpIfTrue(n, v) => {
                    let a = Self::resolve_param(&mem, &n);
                    if a != 0 {
                        let b = Self::resolve_param(&mem, &v);
                        ip = b as usize;
                        continue;
                    }
                },

                Instruction::JumpIfFalse(n, v) => {
                    let a = Self::resolve_param(&mem, &n);
                    if a == 0 {
                        let b = Self::resolve_param(&mem, &v);
                        ip = b as usize;
                        continue;
                    }
                },

                Instruction::LessThan(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    if a < b {
                        mem[op] = "1".to_string();
                    } else {
                        mem[op] = "0".to_string();
                    }
                },

                Instruction::Equals(n, v, op) => {
                    let a = Self::resolve_param(&mem, &n);
                    let b = Self::resolve_param(&mem, &v);
                    if a == b {
                        mem[op] = "1".to_string();
                    } else {
                        mem[op] = "0".to_string();
                    }
                },

                Instruction::HCF => { break; },

                Instruction::BadInput(v) => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("{:?}", v)));
                },
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
    fn resolve_param(mem: &Vec<String>, param: &Parameter) -> i64 {
        match param {
            Parameter::Positional(i) => {
                let v: i64 = mem[*i].parse().unwrap();
                v
            },
            Parameter::Immediate(n) => *n,
            Parameter::UnexpectedMode(m) => { panic!("unexpected mode: {}", m); },
        }
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
    BadInput(Vec<String>),
}

impl Instruction {
    /// Parse the given instruction an parameters in `v`.
    /// The number of parameters for the instruction can be figured out by
    /// calling `Instruction::num_params` on the instruction.
    ///
    /// For example, the short program `1002,4,3,4,33`, is parsed like so:
    /// 
    /// ```
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
    fn parse(v: Vec<String>) -> Self {
        let np = Self::num_params(&v[0]);
        if np + 1 != v.len() {
            return Self::BadInput(v);
        }

        // Collect our parameters.
        //
        // Reverse the instruction so we can iterate through each character
        // naturally, and assume any missing parameter mode is a 0 (zero).
        //
        // We are also going to slice up the instruction to collect all of the
        // parameter modes, after the opcode.
        // This code looks a little wonky, but what it is doing, is making sure
        // we have 2 (two) characters for the opcode.
        // If the second character for the opcode is '-', this means the second
        // character is an implicit 0 (zero), and not an explicit 0.
        let mut params: Vec<Parameter> = vec![];
        let instruction: String = v[0].chars().rev().collect();
        let modes: &str = match instruction.chars().nth(1).unwrap_or('-') {
            '-' => "",
            _ => &instruction[2..],
        };
        for i in 0..np {
            let c = modes.chars().nth(i).unwrap_or('0');
            let param = match c {
                '0' => {
                    // Positional/reference.
                    let val: usize = v[i+1].parse().unwrap();
                    Parameter::Positional(val)
                },

                '1' => {
                    // Immediate.
                    let val: i64 = v[i+1].parse().unwrap();
                    Parameter::Immediate(val)
                },

                _ => { Parameter::UnexpectedMode(c) },
            };

            params.push(param);
        }

        // Get the output parameter (which is always the last instruction
        // argument).
        // Note that this value isn't always used; it depends on the opcode.
        let op: usize = v[v.len()-1].parse().unwrap();

        // Finally, return an Instruction.
        match instruction.chars().nth(0).unwrap() {
            '1' => Self::Add(params[0], params[1], op),
            '2' => Self::Multiply(params[0], params[1], op),
            '3' => Self::Input(op),
            '4' => Self::Output(params[0]),
            '5' => Self::JumpIfTrue(params[0], params[1]),
            '6' => Self::JumpIfFalse(params[0], params[1]),
            '7' => Self::LessThan(params[0], params[1], op),
            '8' => Self::Equals(params[0], params[1], op),
            '9' => Self::HCF,
            _ => Self::BadInput(v),
        }
    }

    // Returns the number of parameters that should be consumed by the
    fn num_params(s: &str) -> usize {
        let r: String = s.chars().rev().collect();
        match r.chars().nth(0).unwrap() {
            '1' => 3, // addition
            '2' => 3, // multiplication
            '3' => 1, // input
            '4' => 1, // output
            '5' => 2, // jump-if-true
            '6' => 2, // jump-if-false
            '7' => 3, // less-than
            '8' => 3, // equals
            '9' => {
                if r.chars().nth(1).unwrap_or('x') == '9' {
                    return 0;
                }
                panic!("Expected opcode 99 from {}", s);
            }, // end program
            _ => { panic!("Bad instruction {}", s); },
        }
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
    UnexpectedMode(char),
}

#[test]
fn test_num_params() {
    assert_eq!(Instruction::num_params("1"), 3);
    assert_eq!(Instruction::num_params("2"), 3);
    assert_eq!(Instruction::num_params("3"), 1);
    assert_eq!(Instruction::num_params("4"), 1);
    assert_eq!(Instruction::num_params("5"), 2);
    assert_eq!(Instruction::num_params("6"), 2);
    assert_eq!(Instruction::num_params("7"), 3);
    assert_eq!(Instruction::num_params("8"), 3);
    assert_eq!(Instruction::num_params("99"), 0);
}

#[test]
fn test_program() {
    use std::collections::HashMap;

    let mut tests: HashMap<&str, &str> = HashMap::new();
    tests.insert("3,0,4,0,99", "1");
    tests.insert("1002,4,3,4,33", "1002");
    tests.insert("1101,100,-1,4,0", "1101");
    
    for (inp, want) in &tests {
        let input = dbg!(String::from(*inp));
        let prog: Vec<String> = input.split(",").map(|s| s.to_string()).collect();
        let p = Program::load(prog).unwrap();
        let (mem, _output) = pc.execute(1).unwrap();
        assert_eq!(&mem[0], want);
    }
}

#[test]
fn test_eq_8_positional() {
    // Output 1 if the input is equal to 8, or 0 if it is not.
    let code = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_, out) = c.execute(8).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "1");

    let (_, out) = c.execute(88).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "0");
}

#[test]
fn test_lt_8_positional() {
    // Output 1 if the input is less than 8, or 0 otherwise.
    let code = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_, out) = c.execute(7).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "1");

    let (_, out) = c.execute(88).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "0");
}

#[test]
fn test_eq_8_immediate() {
    // Output 1 if the input is equal to 8, or 0 otherwise.
    let code = String::from("3,3,1108,-1,8,3,4,3,99");
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_, out) = c.execute(8).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "1");

    let (_, out) = c.execute(88).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "0");
}

#[test]
fn test_lt_8_immediate() {
    // Output 1 if the input is less than 8, or 0 otherwise.
    let code = String::from("3,3,1107,-1,8,3,4,3,99");
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_, out) = c.execute(7).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "1");

    let (_, out) = c.execute(88).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "0");
}

#[test]
fn test_jump_positional() {
    // Output 0 if the input was 0, otherwise return 1.
    let code = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_, out) = c.execute(0).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "0");

    let (_, out) = c.execute(88).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "1");
}

#[test]
fn test_jump_immediate() {
    // Output 0 if the input was 0, otherwise return 1.
    let code = String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_, out) = c.execute(0).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "0");

    let (_, out) = c.execute(88).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0], "1");
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
    let program: Vec<String> = code.split(",").map(|s| s.to_string()).collect();
    let c = Computer::load(program).unwrap();

    let (_mem, output) = c.execute(7).unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(&output[0], "999");

    let (_mem, output) = c.execute(8).unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(&output[0], "1000");

    let (_mem, output) = c.execute(9).unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(&output[0], "1001");
}

