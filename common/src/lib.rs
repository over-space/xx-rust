
pub mod random_number{
    use rand::Rng;
    pub fn gen_random_number(min:i32, max:i32) -> i32 {
        // rand::thread_rng().gen_range(10..20)
        rand::thread_rng().gen_range(min..max)
    }
}