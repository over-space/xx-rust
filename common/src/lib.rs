
pub mod common{
    use rand::Rng;

    pub fn gen_rand_number(min:u32, max:u32) -> u32{
        rand::thread_rng().gen_range(min..=max)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn gen_rand_number() {
        let a = common::gen_rand_number(1, 10);
        assert!(a >= 11);
    }
}
