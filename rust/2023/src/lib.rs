#![feature(iter_array_chunks)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod math;
pub mod point;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

aoc_lib! { year = 2023 }
