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
                vars[*var as usize] = val;
            }
            Instruction::Add(a, b) => vars[*a as usize] = vars[*a as usize] + b.value(&vars),
            Instruction::Mul(a, b) => vars[*a as usize] = vars[*a as usize] * b.value(&vars),
            Instruction::Div(a, b) => {
                let b = b.value(&vars);
                assert_ne!(b, 0);
                vars[*a as usize] = vars[*a as usize] / b
            }
            Instruction::Mod(a, b) => {
                let a_val = vars[*a as usize];
                let b_val = b.value(&vars);
                assert!(a_val >= 0);
                assert!(b_val > 0);
                vars[*a as usize] = a_val % b_val
            }
            Instruction::Eql(a, b) => {
                vars[*a as usize] = (vars[*a as usize] == b.value(&vars)) as i64
            }
        }
    }

    vars
}

#[derive(Debug, Copy, Clone)]
enum Var {
    W = 0,
    X,
    Y,
    Z,
}

#[derive(Debug)]
enum Operand {
    Var(Var),
    Lit(i64),
}
impl Operand {
    fn value(&self, vars: &[i64; 4]) -> i64 {
        match self {
            Operand::Var(var) => vars[*var as usize],
            Operand::Lit(num) => *num,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(Var),
    Add(Var, Operand),
    Mul(Var, Operand),
    Div(Var, Operand),
    Mod(Var, Operand),
    Eql(Var, Operand),
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_instruction)
        .collect()
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
            match op {
                "add" => Instruction::Add(a, b),
                "mul" => Instruction::Mul(a, b),
                "div" => Instruction::Div(a, b),
                "mod" => Instruction::Mod(a, b),
                "eql" => Instruction::Eql(a, b),
                _ => unreachable!("invalid instruction '{op}'"),
            }
        }
        _ => unreachable!("invalid instruction: '{line}'"),
    }
}

fn parse_var(var: &str) -> Var {
    match var {
        "w" => Var::W,
        "x" => Var::X,
        "y" => Var::Y,
        "z" => Var::Z,
        _ => unreachable!("invalid variable '{var}'"),
    }
}
