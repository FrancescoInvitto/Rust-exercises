/*
    Returns a random number (between nmin and nmax) and a new seed.
*/
fn rand_int(nmin: i32, nmax: i32, seed: u32) -> (i32, u32) {
    let mut seed : u32 = seed;
    // From "Xorshift RNGs" by George Marsaglia
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    let range = (nmax + 1 - nmin) as u32;
    let val = nmin + (seed % range) as i32;
    (val, seed)
}

/*
    Returns a time seed.
*/
fn time_seed() -> u32 {
    use std::time::SystemTime as st;
    let now = st::now().duration_since(st::UNIX_EPOCH).unwrap();
    now.as_millis() as u32
}

/*
    Launches two dices 1000 times and counts the occurrence of every possible outcome (2-12)
*/
fn main(){
    let mut occurrences = [0i32; 11];
    let mut seed = time_seed();

    for _n in 1..1001{
        let (dice1, _) = rand_int(1, 6, seed);
        seed += 1;
        let (dice2, _) = rand_int(1, 6, seed);
        seed += 1;
        let sum = dice1 + dice2;

        occurrences[(sum - 2) as usize] += 1;
    }

    for occ in occurrences{
        println!("{}", occ); 
    }
}