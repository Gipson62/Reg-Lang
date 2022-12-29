#![allow(unused)]

pub mod types;
pub mod instructions;
pub mod tools;


#[macro_use]
use core::panic;
use std::process::Output;
use crate::instructions::set::Instructions;
use tools::{
    stack::RuntimeStack,
    operators::Operator,
};
use types::{
    number::{
        base_number::{
            Arithmetics, Numbers
        },
        float::float64::Float64,
        int::int64::Int64
    },
    types::Types,
};

pub fn run(instructions: Vec<Instructions>) {
    let mut runtime_stack = RuntimeStack::new();
    for instruction in instructions {
        match instruction {
            Instructions::LoadInt64(i) => {
                runtime_stack.stack.push(Types::Int64(Int64(i)));
            },
            Instructions::LoadFloat64(f) => {
                runtime_stack.stack.push(Types::Float64(Float64(f)));
            },
            Instructions::Add => {
                let add = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    Operator::Add
                ).to_types();
                runtime_stack.stack.push(add);
            },
            Instructions::Sub => {
                let sub = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    Operator::Sub
                ).to_types();
                runtime_stack.stack.push(sub);

            },
            Instructions::Mod => {
                let modulo = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    Operator::Mod
                ).to_types();
                runtime_stack.stack.push(modulo);
            },
            Instructions::Div => {
                let div = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    Operator::Div
                ).to_types();
                runtime_stack.stack.push(div);
            },
            Instructions::Mul => {
                let right = &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number"));
                let mul = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    Operator::Mul
                ).to_types();
                runtime_stack.stack.push(mul);
            },
            Instructions::Print => {
                println!("{:?}", runtime_stack.stack[runtime_stack.stack.len() - 1]);
            },
            _ => {
                panic!("WHAT THE FUCK YOU DOING ?");
            }
        }
    }
}

fn operation(right: &Numbers, left: &Numbers, operator: Operator) -> Numbers {
    match operator {
        Operator::Add => {
            return left.add(right);
        },
        Operator::Mul => {
            return left.mul(right);
        },
        Operator::Mod => {
            return left.rem(right);
        },
        Operator::Sub => {
            return left.sub(right);
        },
        Operator::Div => {
            return left.div(right);
        },
        _ => {
            panic!("Not a valid operator !")
        }
    }
}