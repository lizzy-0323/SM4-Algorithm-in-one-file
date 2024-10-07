mod sm4;
use sm4::SM4;
use std::time::{Duration, Instant};
fn main() {
    println!("SM4 algorithm init");
    let start = Instant::now();
    let key = 0x0123456789abcdeffedcba9876543210;
    let sm = SM4::new(key);
    let plaintext = 0x0123456789abcdeffedcba9876543210;
    println!("plaintext = {:x}", plaintext);
    let ciphertext = sm.encrypt(plaintext);
    println!("ciphertext = {:x}", ciphertext);
    let decrypttext: u128 = sm.decrypt(ciphertext);
    println!("decrypttext = {:x}", decrypttext);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
