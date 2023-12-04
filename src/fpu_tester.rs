use std::io::{BufRead, BufReader};

use crate::{fpu_emulator::*, utils::u32_to_i32};

pub enum FourArithmeticOperation {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn test_fpu(operation: &str) {
    match operation {
        "fadd" => test_four_arithmetic_operation(
            "../test_result/fadd_result.txt",
            FourArithmeticOperation::Add,
        ),
        "fsub" => test_four_arithmetic_operation(
            "../test_result/fsub_result.txt",
            FourArithmeticOperation::Sub,
        ),
        "fmul" => test_four_arithmetic_operation(
            "../test_result/fmul_result.txt",
            FourArithmeticOperation::Mul,
        ),
        "fdiv" => test_four_arithmetic_operation(
            "../test_result/fdiv_result.txt",
            FourArithmeticOperation::Div,
        ),
        "fsqrt" => test_fsqrt(),
        "flt" => test_flt(),
        "fcvtsw" => test_fcvtsw(),
        "fcvtws" => test_fcvtws(),
        _ => panic!(
            "FPU operation name must be fadd, fsub, fmul, fdiv, fsqrt, flt, fcvtsw, or fcvtws."
        ),
    }
}

fn test_four_arithmetic_operation(path: &str, operation: FourArithmeticOperation) {
    let file = std::fs::File::open(path);
    let inv_map = create_inv_map_f32();
    match file {
        Err(e) => {
            eprintln!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            eprintln!("Success in opening file.");
            let reader = BufReader::new(file);
            let mut cnt = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                let line: Vec<&str> = line.split_whitespace().collect();
                if line.len() != 3 {
                    continue;
                }
                let x1 = u32::from_str_radix(line[0], 2);
                if x1.is_err() {
                    continue;
                }
                let x1 = x1.unwrap();
                let x1 = FloatingPoint::new(x1);
                let x2 = u32::from_str_radix(line[1], 2).unwrap();
                let x2 = FloatingPoint::new(x2);
                let y = u32::from_str_radix(line[2], 2).unwrap();
                let y = FloatingPoint::new(y);
                let correct_result = match operation {
                    FourArithmeticOperation::Add => x1 + x2,
                    FourArithmeticOperation::Sub => x1 - x2,
                    FourArithmeticOperation::Mul => x1 * x2,
                    FourArithmeticOperation::Div => div_fp(x1, x2, &inv_map),
                };
                let exp_of_correct_result = correct_result.get_exp();
                let ey = y.get_exp();
                if exp_of_correct_result == -127 || ey == -127 {
                    if exp_of_correct_result != ey {
                        panic!(
                            "x1 = {:?}\nx2 = {:?}\nl  = {:?}\nr  = {:?}",
                            x1, x2, correct_result, y
                        );
                    }
                } else {
                    if correct_result != y {
                        panic!(
                            "x1 = {:?}\nx2 = {:?}\nl  = {:?}\nr  = {:?}",
                            x1, x2, correct_result, y
                        );
                    }
                }
                cnt += 1;
            }
            eprintln!("{} tests passed.", cnt);
        }
    }
}

fn test_flt() {
    let file = std::fs::File::open("../test_result/flt_result.txt");
    match file {
        Err(e) => {
            eprintln!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            eprintln!("Success in opening file.");
            let reader = BufReader::new(file);
            let mut cnt = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                let line: Vec<&str> = line.split_whitespace().collect();
                if line.len() != 3 {
                    continue;
                }
                let x1 = u32::from_str_radix(line[0], 2);
                if x1.is_err() {
                    continue;
                }
                let x1 = x1.unwrap();
                let x1 = FloatingPoint::new(x1);
                let x2 = u32::from_str_radix(line[1], 2).unwrap();
                let x2 = FloatingPoint::new(x2);
                let y = u32::from_str_radix(line[2], 2).unwrap();
                let y = y == 1;
                let correct_result = x1 < x2;
                if correct_result != y {
                    panic!(
                        "x1 = {:?}\nx2 = {:?}\nl  = {:?}\nr  = {:?}",
                        x1, x2, correct_result, y
                    );
                }
                cnt += 1;
            }
            eprintln!("{} tests passed.", cnt);
        }
    }
}

fn test_fsqrt() {
    let file = std::fs::File::open("../test_result/fsqrt_result.txt");
    let sqrt_map = create_sqrt_map_f32();
    match file {
        Err(e) => {
            eprintln!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            eprintln!("Success in opening file.");
            let reader = BufReader::new(file);
            let mut cnt = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                let line: Vec<&str> = line.split_whitespace().collect();
                if line.len() != 2 {
                    continue;
                }
                let x1 = u32::from_str_radix(line[0], 2);
                if x1.is_err() {
                    continue;
                }
                let x1 = x1.unwrap();
                let x1 = FloatingPoint::new(x1);
                let y = u32::from_str_radix(line[1], 2).unwrap();
                let y = FloatingPoint::new(y);
                let correct_result = sqrt_fp(x1, &sqrt_map);
                let exp_of_correct_result = correct_result.get_exp();
                let ey = y.get_exp();
                if exp_of_correct_result == -127 || ey == -127 {
                    if exp_of_correct_result != ey {
                        panic!("x1 = {:?}\nl  = {:?}\nr  = {:?}", x1, correct_result, y);
                    }
                } else {
                    if correct_result != y {
                        panic!("x1 = {:?}\nl  = {:?}\nr  = {:?}", x1, correct_result, y);
                    }
                }
                cnt += 1;
            }
            eprintln!("{} tests passed.", cnt);
        }
    }
}

fn test_fcvtsw() {
    let file = std::fs::File::open("../test_result/fcvtsw_result.txt");
    match file {
        Err(e) => {
            eprintln!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            eprintln!("Success in opening file.");
            let reader = BufReader::new(file);
            let mut cnt = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                let line: Vec<&str> = line.split_whitespace().collect();
                if line.len() != 2 {
                    continue;
                }
                let x1 = i32::from_str_radix(line[0], 2);
                if x1.is_err() {
                    continue;
                }
                let x1 = x1.unwrap();
                let y = u32::from_str_radix(line[1], 2).unwrap();
                let y = FloatingPoint::new(y);
                let correct_result = int_to_fp(x1);
                let exp_of_correct_result = correct_result.get_exp();
                let ey = y.get_exp();
                if exp_of_correct_result == -127 || ey == -127 {
                    if exp_of_correct_result != ey {
                        panic!("x1 = {:?}\nl  = {:?}\nr  = {:?}", x1, correct_result, y);
                    }
                } else {
                    if correct_result != y {
                        panic!("x1 = {:?}\nl  = {:?}\nr  = {:?}", x1, correct_result, y);
                    }
                }
                cnt += 1;
            }
            eprintln!("{} tests passed.", cnt);
        }
    }
}

fn test_fcvtws() {
    let file = std::fs::File::open("../test_result/fcvtws_result.txt");
    match file {
        Err(e) => {
            eprintln!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            eprintln!("Success in opening file.");
            let reader = BufReader::new(file);
            let mut cnt = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                let line: Vec<&str> = line.split_whitespace().collect();
                if line.len() != 2 {
                    continue;
                }
                let x1 = u32::from_str_radix(line[0], 2);
                if x1.is_err() {
                    continue;
                }
                let x1 = x1.unwrap();
                let x1 = FloatingPoint::new(x1);
                let y = u32_to_i32(u32::from_str_radix(line[1], 2).unwrap());
                let correct_result = fp_to_int(x1);
                if correct_result != y {
                    panic!("x1 = {:?}\nl  = {:?}\nr  = {:?}", x1, correct_result, y);
                }
                cnt += 1;
            }
            eprintln!("{} tests passed.", cnt);
        }
    }
}
