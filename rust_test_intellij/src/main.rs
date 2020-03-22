use std::mem;
use std::time::{Duration, Instant};
use std::thread::sleep;


fn main() {
    test_inv();

}


fn fstInv(x : f32) -> f32 {
    let x2 = x * 0.5;
    let threehalfs = 1.5;

    let mut i: u32 = unsafe { mem::transmute(x) };
    i = 0x5F3759DF - (i >> 1);
    let k: f32 = unsafe { mem::transmute(i)};
    k * (threehalfs - ( x2 * k * k ))
}

fn inv(x : f32) -> f32 {
    1. / x.sqrt()
}

fn test_inv() {
    let now = Instant::now();
    let mut sum = 0.;
    for i in 1..1_000_000_000{
        sum += fstInv(i as f32);
    }
    println!("{}", sum);

    println!("{}", now.elapsed().as_millis());

    let now = Instant::now();
    let mut sum = 0.;
    for i in 1..1_000_000_000{
        sum += inv(i as f32);
    }
    println!("{}", sum);

    println!("{}", now.elapsed().as_millis());
}
