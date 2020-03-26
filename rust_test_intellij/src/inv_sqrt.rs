pub mod fast_inv{
    use std::mem;
    use std::time::{Instant};

    // ~30% faster, but unsafe (tests are not accurate, it is more faster)
    pub fn inv_f(x : f32) -> f32 {
        let x2 = x * 0.5;
        let threehalfs = 1.5;
    
        let mut i: u32 = unsafe { mem::transmute(x) };
        i = 0x5F3759DF - (i >> 1);
        let k: f32 = unsafe { mem::transmute(i)};
        k * (threehalfs - ( x2 * k * k ))
    }
    
    pub fn inv(x : f32) -> f32 {
        1. / x.sqrt()
    }
    
    pub fn test_inv() {
        let now = Instant::now();
        let mut sum = 0.;
        for i in 1..1_000_000_000{
            sum += inv_f(i as f32);
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
}