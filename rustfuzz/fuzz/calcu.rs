use rand::Rng;
pub fn calculate(x: i32, y: i32) -> i32 {
    let mut rng = rand::thread_rng();  // Create a random number generator
    let num: i32 = rng.gen_range(0..4); 
    match num {
        1 => x+y,
        2 => x-y,
        3 => x*y,
        4 => if y != 0 { x / y } else { 0 },
        _ => 0,
    }
}
