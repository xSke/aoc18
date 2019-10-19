use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn execute(&self, mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
        regs[c] = match self {
            Opcode::Addr => regs[a] + regs[b],
            Opcode::Addi => regs[a] + b,
            Opcode::Mulr => regs[a] * regs[b],
            Opcode::Muli => regs[a] * b,
            Opcode::Banr => regs[a] & regs[b],
            Opcode::Bani => regs[a] & b,
            Opcode::Borr => regs[a] | regs[b],
            Opcode::Bori => regs[a] | b,
            Opcode::Setr => regs[a],
            Opcode::Seti => a,
            Opcode::Gtir => if a > regs[b] { 1 } else { 0 },
            Opcode::Gtri => if regs[a] > b { 1 } else { 0 },
            Opcode::Gtrr => if regs[a] > regs[b] { 1 } else { 0 },
            Opcode::Eqir => if a == regs[b] { 1 } else { 0 },
            Opcode::Eqri => if regs[a] == b { 1 } else { 0 },
            Opcode::Eqrr => if regs[a] == regs[b] { 1 } else { 0 }
        };
        return regs;
    }
}

#[derive(Clone, Debug)]
pub struct TestCase {
    input_registers: [usize; 4],
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
    output_registers: [usize; 4],
}

lazy_static! {
    static ref BEFORE_RE: Regex = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    static ref INPUT_RE: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    static ref AFTER_RE: Regex = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    static ref ALL_OPCODES: [Opcode; 16] = [Opcode::Addr, Opcode::Addi, Opcode::Mulr, Opcode::Muli, Opcode::Banr, Opcode::Bani, Opcode::Borr, Opcode::Bori, Opcode::Setr, Opcode::Seti, Opcode::Gtir, Opcode::Gtri, Opcode::Gtrr, Opcode::Eqir, Opcode::Eqri, Opcode::Eqrr];
}

fn parse_case(case: &str) -> TestCase {
    match case.split("\n").collect::<Vec<_>>().as_slice() {
        [before, input, after] => {
            let before_captures = BEFORE_RE.captures(before).unwrap();
            let input_captures = INPUT_RE.captures(input).unwrap();
            let after_captures = AFTER_RE.captures(after).unwrap();

            return TestCase {
                input_registers: [before_captures[1].parse().unwrap(), before_captures[2].parse().unwrap(), before_captures[3].parse().unwrap(), before_captures[4].parse().unwrap()],
                opcode: input_captures[1].parse().unwrap(),
                a: input_captures[2].parse().unwrap(),
                b: input_captures[3].parse().unwrap(),
                c: input_captures[4].parse().unwrap(),
                output_registers: [after_captures[1].parse().unwrap(), after_captures[2].parse().unwrap(), after_captures[3].parse().unwrap(), after_captures[4].parse().unwrap()],
            };
        }
        _ => unreachable!()
    }
}

fn matching_opcodes(case: &TestCase) -> Vec<Opcode> {
    let mut matching_opcodes = vec![];
    for opcode in ALL_OPCODES.iter() {
        let output = opcode.execute(case.input_registers, case.a, case.b, case.c);
        if output == case.output_registers {
            matching_opcodes.push(*opcode);
        }
    }
    return matching_opcodes;
}

pub fn part1(input: &str) -> (String, Vec<TestCase>) {
    let part_1_input: &str = input.split("\n\n\n").next().unwrap();

    let test_cases = part_1_input.split("\n\n").map(parse_case).collect::<Vec<_>>();
//    dbg!(&test_cases);

    let mut cases_over_3 = 0;
    for case in test_cases.iter() {
        let matching_opcode_count = matching_opcodes(case).len();
        if matching_opcode_count >= 3 {
            cases_over_3 += 1;
        }
    }

    return (cases_over_3.to_string(), test_cases);
}

pub fn part2(input: &str, cases: Vec<TestCase>) -> String {
    let mut opcode_possibilities: Vec<Vec<Opcode>> = vec![];
    for _ in 0..16 {
        opcode_possibilities.push(ALL_OPCODES.iter().map(|x| *x).collect());
    }

    for case in cases.iter() {
        if opcode_possibilities[case.opcode].len() == 1 { continue; }

        let matches = matching_opcodes(case);
        opcode_possibilities[case.opcode].retain(|op| matches.contains(op));
    }

    while !opcode_possibilities.iter().all(|op| op.len() == 1) {
        for op_a in 0..16 {
            if opcode_possibilities[op_a].len() == 1 {
                let the_opcode_to_remove = opcode_possibilities[op_a][0];
                for op_b in 0..16 {
                    if op_a == op_b { continue; }
                    opcode_possibilities[op_b].retain(|&o| o != the_opcode_to_remove);
                }
            }
        }
    }

    let opcodes = opcode_possibilities.iter().flatten().collect::<Vec<_>>();
    let mut registers = [0usize; 4];

    let code_str = input.split("\n\n\n").skip(1).next().unwrap();
    for code_line in code_str.split("\n") {
        if code_line.trim().len() == 0 { continue; }

        let captures = INPUT_RE.captures(code_line).unwrap();
        let (opcode, a, b, c): (usize, usize, usize, usize) = (captures[1].parse().unwrap(), captures[2].parse().unwrap(), captures[3].parse().unwrap(), captures[4].parse().unwrap());
        registers = opcodes[opcode].execute(registers, a, b, c);
    }
    return registers[0].to_string();
}