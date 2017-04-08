#![feature(test)]
extern crate test;
extern crate num;
extern crate rust_dct;

use rust_dct::dct1::{DCT1, DCT1Naive};
use rust_dct::dct2::{DCT2, DCT2Naive};
use rust_dct::dct3::{DCT3, DCT3Naive};
use rust_dct::dct4::{DCT4, DCT4Naive};

use test::Bencher;

/// Times just the DCT1 execution (not allocation and pre-calculation)
/// for a given length
fn bench_dct1_naive(b: &mut Bencher, len: usize) {

    let mut dct = DCT1Naive::new(len);

    let mut signal = vec![0_f32; len];
    let mut spectrum = signal.clone();
    b.iter(|| {dct.process(&mut signal, &mut spectrum);} );
}

#[bench] fn dct1_naive_0024(b: &mut Bencher) { bench_dct1_naive(b,   24); }
#[bench] fn dct1_naive_0025(b: &mut Bencher) { bench_dct1_naive(b,   25); }
#[bench] fn dct1_naive_0026(b: &mut Bencher) { bench_dct1_naive(b,   26); }



/// Times just the DCT2 execution (not allocation and pre-calculation)
/// for a given length
fn bench_dct2_naive(b: &mut Bencher, len: usize) {

    let mut dct = DCT2Naive::new(len);

    let mut signal = vec![0_f32; len];
    let mut spectrum = signal.clone();
    b.iter(|| {dct.process(&mut signal, &mut spectrum);} );
}

#[bench] fn dct2_naive_0003(b: &mut Bencher) { bench_dct2_naive(b,   3); }
#[bench] fn dct2_naive_0004(b: &mut Bencher) { bench_dct2_naive(b,   4); }
#[bench] fn dct2_naive_0005(b: &mut Bencher) { bench_dct2_naive(b,   5); }
#[bench] fn dct2_naive_0006(b: &mut Bencher) { bench_dct2_naive(b,   6); }





/// Times just the DCT3 execution (not allocation and pre-calculation)
/// for a given length
fn bench_dct3_naive(b: &mut Bencher, len: usize) {

    let mut dct = DCT3Naive::new(len);

    let mut signal = vec![0_f32; len];
    let mut spectrum = signal.clone();
    b.iter(|| {dct.process(&mut signal, &mut spectrum);} );
}

#[bench] fn dct3_naive_0003(b: &mut Bencher) { bench_dct3_naive(b,   3); }
#[bench] fn dct3_naive_0004(b: &mut Bencher) { bench_dct3_naive(b,   4); }
#[bench] fn dct3_naive_0005(b: &mut Bencher) { bench_dct3_naive(b,   5); }
#[bench] fn dct3_naive_0006(b: &mut Bencher) { bench_dct3_naive(b,   6); }




/// Times just the DCT4 execution (not allocation and pre-calculation)
/// for a given length
fn bench_dct4_naive(b: &mut Bencher, len: usize) {

    let mut dct = DCT4Naive::new(len);

    let mut signal = vec![0_f32; len];
    let mut spectrum = signal.clone();
    b.iter(|| {dct.process(&mut signal, &mut spectrum);} );
}

#[bench] fn dct4_naive_0016(b: &mut Bencher) { bench_dct4_naive(b,   16); }
#[bench] fn dct4_naive_0256(b: &mut Bencher) { bench_dct4_naive(b,  256); }