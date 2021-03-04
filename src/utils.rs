pub fn rand() -> f64 {
    rand::random()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..max)
}
