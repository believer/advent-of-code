#![feature(iter_array_chunks)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod grid;
pub mod math;
pub mod point;

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

aoc_lib! { year = 2023 }
