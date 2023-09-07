use std::collections::HashMap;

pub fn run() {
    let program = parse_program(include_str!("inputs/day24"));
    let mut blocks = vec![];
    for ins in program.into_iter() {
        if matches!(ins, Instruction::Inp(_)) {
            blocks.push(vec![ins]);
        } else {
            blocks
                .last_mut()
                .expect("a block of instructions should exist")
                .push(ins);
        }
    }

    let mut max_inputs: HashMap<_, _> = HashMap::from_iter([(0, vec![])]);
    let mut min_inputs: HashMap<_, _> = HashMap::from_iter([(0, vec![])]);
    for block in blocks.iter() {
        let mut can_reduce_z = false;
        for z0 in 0..26 {
            for w in 1..=9 {
                let [.., z] = run_alu_program(block, &[w], [0, 0, 0, z0]);
                if z == 0 {
                    can_reduce_z = true;
                }
            }
        }

        let mut new_max_inputs = HashMap::new();
        for (prev_z, prev_inputs) in max_inputs.iter() {
            for w in 1..=9 {
                let [.., z] = run_alu_program(block, &[w], [0, 0, 0, *prev_z]);
                if (can_reduce_z && z == *prev_z / 26) || !can_reduce_z {
                    let mut inputs = prev_inputs.clone();
                    inputs.push(w);
                    if let Some(v) = new_max_inputs.get_mut(&z) {
                        if &inputs > v {
                            *v = inputs;
                        }
                    } else {
                        new_max_inputs.insert(z, inputs);
                    }
                }
            }
        }
        max_inputs = new_max_inputs;

        let mut new_min_inputs = HashMap::new();
        for (prev_z, prev_inputs) in min_inputs.iter() {
            for w in 1..=9 {
                let [.., z] = run_alu_program(block, &[w], [0, 0, 0, *prev_z]);
                if (can_reduce_z && z == *prev_z / 26) || !can_reduce_z {
                    let mut inputs = prev_inputs.clone();
                    inputs.push(w);
                    if let Some(v) = new_min_inputs.get_mut(&z) {
                        if &inputs < v {
                            *v = inputs;
                        }
                    } else {
                        new_min_inputs.insert(z, inputs);
                    }
                }
            }
        }
        min_inputs = new_min_inputs;
    }
    assert_eq!(max_inputs.len(), 1);
    assert_eq!(min_inputs.len(), 1);

    let max_inputs = max_inputs.values().next().unwrap();
    let max_model_number = max_inputs
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join("");

    let min_inputs = min_inputs.values().next().unwrap();
    let min_model_number = min_inputs
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("{max_model_number}");
    println!("{min_model_number}");
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
