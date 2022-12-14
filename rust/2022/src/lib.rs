#![feature(iter_array_chunks)]

extern crate aoc_runner;
extern crate pathfinding;

#[macro_use]
extern crate aoc_runner_derive;

pub mod common;
pub mod math;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;

// Testing the nom parser against the example inputs
// pub mod parser_tests;

aoc_lib! { year = 2022 }
