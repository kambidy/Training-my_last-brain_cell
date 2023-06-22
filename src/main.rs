#![allow(unused)]
use std::io;
use rand::{Rng, random};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

static TRAIN:[[usize;2]; 5] = [
        [0,0],
        [1,2],
        [2,4],
        [3,6],
        [4,8],
    ];

static TRAIN_COUNT: usize = TRAIN.len();

fn rand_float() -> f32{
    let mut num: f32= random();
    let den: f32 = random();
    return ((num/den)*10.0);
}
fn cost(w: f32)-> f32{
    let mut i: usize = 0;
    let mut result: f32 = 0.0;

    while i < TRAIN_COUNT.try_into().unwrap(){
        let mut x: f32 = TRAIN[i][0] as f32;
        let mut y: f32 = x * w;
        let mut d: f32 = y - TRAIN[i][1] as f32 ;
        result += d*d;
        //println!("atual:{:?}..vs..expected:{}",y,TRAIN[i][1]);

        i = i+1;
     }

     result /= TRAIN_COUNT as f32;
     return result;
}
fn main() {
    let mut w: f32 = rand_float();
    let mut eps: f32= 1e-3;
    let mut rate: f32 = 1e-3;
    println!("{}", cost(w));
    let mut lup: i32 = 0;
    while lup < 10000 {
        let mut dcost:f32 = (cost(w + eps) - cost(w))/eps;
        w -= rate*dcost;
        println!("cost:{} and value:{}",cost(w), w);
        lup = lup+1;
    }
}

