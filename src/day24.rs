use std::{cmp, collections::HashMap};

pub fn run() {
    let program = parse_program(include_str!("inputs/day24"));

    // Split the program into "blocks", which consist of a single `inp` instruction and its
    // following instructions until the next `inp`.
    let mut blocks = vec![];
    for ins in program {
        if matches!(ins, Instruction::Inp(_)) {
            blocks.push(vec![ins]);
        } else {
            blocks
                .last_mut()
                .expect("a block of instructions should exist")
                .push(ins);
        }
    }

    let max_model_number = find_model_number(&blocks, cmp::max);
    let min_model_number = find_model_number(&blocks, cmp::min);

    println!("{max_model_number}");
    println!("{min_model_number}");
}

fn find_model_number<P>(blocks: &[Vec<Instruction>], pick_inputs: P) -> i64
where
    P: Fn(i64, i64) -> i64,
{
    let mut inputs_by_z = HashMap::from_iter([(0, 0)]);
    for block in blocks.iter() {
        // Each block of this program behaves like this pseudo-code:
        //
        // let w = input();
        // let x = z % 26 + C1;
        // if C1 < 0 {
        //     z /= 26
        // }
        // if x != w {
        //     z *= 26
        //     z += w + C2
        // }
        //
        // The variables w, x and y are all "local" to each block. Only the variable z has a value
        // that persists between blocks, and we want that to end at 0.
        // C1 and C2 are constants and are different in each block.
        // There are 14 blocks in total, and 7 of them have C1 > 10, so the condition of x != w is
        // always true no matter the input w, and those blocks increment z by 26-ish times.
        // The other 7 blocks have a negative C1. In those ones, we need to make sure that x == w,
        // by choosing the correct w input so that z doesn't get bigger and instead gets rounded
        // down 26 times.
        // This code picks the correct w input values for z to eventually go down to 0.

        let mut can_reduce_z = false;
        for z0 in 0..26 {
            for w in 1..=9 {
                let [.., z] = run_alu_program(block, &[w], [0, 0, 0, z0]);
                if z == 0 {
                    can_reduce_z = true;
                }
            }
        }

        let mut new_inputs_by_z = HashMap::new();
        for (prev_z, prev_inputs) in inputs_by_z.iter() {
            for w in 1..=9 {
                let [.., z] = run_alu_program(block, &[w], [0, 0, 0, *prev_z]);
                let input_reduces_z = z == *prev_z / 26;
                if input_reduces_z || !can_reduce_z {
                    let inputs = *prev_inputs * 10 + w;
                    if let Some(v) = new_inputs_by_z.get_mut(&z) {
                        *v = pick_inputs(*v, inputs);
                    } else {
                        new_inputs_by_z.insert(z, inputs);
                    }
                }
            }
        }
        inputs_by_z = new_inputs_by_z;
    }
    assert_eq!(inputs_by_z.len(), 1);

    *inputs_by_z.values().next().unwrap()
}

fn run_alu_program(program: &[Instruction], inputs: &[i64], initial_vars: [i64; 4]) -> [i64; 4] {
    let mut vars = initial_vars;
    let mut input = inputs.into_iter();

    for ins in program {
        match ins {
            Instruction::Inp(var) => {
                let val = *input.next().expect("insufficient inputs for program");
                vars[*var] = val;
            }
            Instruction::Op(op, a, b) => {
                vars[*a] = {
                    let a = vars[*a];
                    let b = match b {
                        Operand::Var(var) => vars[*var],
                        Operand::Lit(num) => *num,
                    };
                    match op {
                        Op::Add => a + b,
                        Op::Mul => a * b,
                        Op::Div => a / b,
                        Op::Mod => a % b,
                        Op::Eql => (a == b) as i64,
                    }
                }
            }
        }
    }

    vars
}

enum Instruction {
    Inp(usize),
    Op(Op, usize, Operand),
}

enum Operand {
    Var(usize),
    Lit(i64),
}

enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let words: Vec<_> = line.split_whitespace().collect();
    match words[..] {
        ["inp", var] => Instruction::Inp(parse_var(var)),
        [op, a, b] => {
            let a = parse_var(a);
            let b = if let Ok(num) = b.parse() {
                Operand::Lit(num)
            } else {
                Operand::Var(parse_var(b))
            };
            let op = match op {
                "add" => Op::Add,
                "mul" => Op::Mul,
                "div" => Op::Div,
                "mod" => Op::Mod,
                "eql" => Op::Eql,
                _ => unreachable!("invalid instruction '{op}'"),
            };
            Instruction::Op(op, a, b)
        }
        _ => unreachable!("invalid instruction: '{line}'"),
    }
}

fn parse_var(var: &str) -> usize {
    match var {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => unreachable!("invalid variable '{var}'"),
    }
}
