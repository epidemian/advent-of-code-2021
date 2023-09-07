use std::collections::HashMap;

pub fn run() {
    let input = "inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 13
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 14
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 15
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 14
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -8
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 10
    mul y x
    add z y";
    let program = parse_program(input);
    let mut zs = HashMap::new();
    for w1 in 1..=9 {
        for w2 in 1..=9 {
            for w3 in 1..=9 {
                for w4 in 1..=9 {
                    let inputs = [w1, w2, w3, w4];
                    let res = run_alu_program(&program, &inputs, [0, 0, 0, 0], false);
                    if res[3] < 5000 {
                        // println!("{w1}{w2}{w3}{w4} ->{res:?}");
                        zs.insert(res[3], inputs);
                    }
                }
            }
        }
    }
    println!("{zs:?}");

    let input2 = "inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 13
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 9
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 15
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 12
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 8
    mul y x
    add z y";
    let program2 = parse_program(input2);
    let mut zs2 = HashMap::new();
    for (z, z_inputs) in zs.iter() {
        for w5 in 1..=9 {
            for w6 in 1..=9 {
                for w7 in 1..=9 {
                    let inputs = [w5, w6, w7];
                    let res = run_alu_program(&program2, &inputs, [0, 0, 0, *z], false);
                    if res[3] < 50_000 {
                        println!("{w5}{w6}{w7}, z={z} ->{res:?}");
                        let [w1, w2, w3, w4] = *z_inputs;
                        zs2.entry(res[3])
                            .and_modify(|v: &mut [i64; 7]| {
                                *v = [w1, w2, w3, w4, w5, w6, w7].max(*v)
                            })
                            .or_insert([w1, w2, w3, w4, w5, w6, w7]);
                    }
                }
            }
        }
    }

    let input3 = "inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -4
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 13
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -15
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 12
    mul y x
    add z y";
    let program3 = parse_program(input3);
    let mut zs3 = HashMap::new();
    for (z, z_inputs) in zs2.iter() {
        for w8 in 1..=9 {
            for w9 in 1..=9 {
                let res = run_alu_program(&program3, &[w8, w9], [0, 0, 0, *z], false);
                if res[3] < 100 {
                    println!("{w8}{w9}, z={z} ->{res:?}");
                    let [w1, w2, w3, w4, w5, w6, w7] = *z_inputs;
                    zs3.entry(res[3])
                        .and_modify(|v: &mut [i64; 9]| {
                            *v = [w1, w2, w3, w4, w5, w6, w7, w8, w9].max(*v)
                        })
                        .or_insert([w1, w2, w3, w4, w5, w6, w7, w8, w9]);
                }
            }
        }
    }

    let input4 = "inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 14
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 14
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 9
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -1
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 15
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -8
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 4
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -14
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 10
    mul y x
    add z y";
    let program4 = parse_program(input4);
    let whole_program = parse_program(include_str!("inputs/day24"));
    let mut zs4 = HashMap::new();
    for (z, z_inputs) in zs3.iter() {
        for w10 in 1..=9 {
            for w11 in 1..=9 {
                for w12 in 1..=9 {
                    for w13 in 1..=9 {
                        for w14 in 1..=9 {
                            let res = run_alu_program(
                                &program4,
                                &[w10, w11, w12, w13, w14],
                                [0, 0, 0, *z],
                                false,
                            );
                            if res[3] == 0 {
                                let [w1, w2, w3, w4, w5, w6, w7, w8, w9] = *z_inputs;
                                let all_inputs =
                                    [w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14];
                                let res =
                                    run_alu_program(&whole_program, &all_inputs, [0; 4], false);
                                assert_eq!(res[3], 0);
                                println!(
                                    "{} (z={z}) is a valid model number, {res:?}!",
                                    all_inputs.map(|w| w.to_string()).join("")
                                );

                                zs4.entry(res[3])
                                    .and_modify(|v: &mut [i64; 14]| *v = (*v).max(all_inputs))
                                    .or_insert(all_inputs);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{zs4:?}");
}

fn run_alu_program(
    program: &[Instruction],
    inputs: &[i64],
    initial_vars: [i64; 4],
    debug: bool,
) -> [i64; 4] {
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
        if debug {
            println!("after {ins:?}: {vars:?}");
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
