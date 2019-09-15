/*
fn get_small_primes(n: i8) -> std::vec::Vec<i8>{

}

fn div_small_primes(numb: i32) -> bool { 
    let SMALL_PRIMES: [i8; 2048] = get_small_primes(2048);
    for p in SMALL_PRIMES.into_iter() {
        if numb % &Int::from(*p) == 0 {
            return false;
        }
    }
    return true;
}

fn little_fermat(candidate: &Int) -> bool {
    use self::ramp::RandomInt;
    let mut rng = OsRng::new().ok().expect("Failed to get OS random generator");
    let random:Int = rng.gen_uint_below(candidate); 
    let result = std::Int::modpow(&random, &(candidate - &Int::one()), candidate);
    result == Int::one()
}

fn miller_rabin(candidate: &Int, limit: usize) -> bool {
    let (s,d) = rewrite(&(candidate - &Int::one()));
    let one = Int::one();
    let two = &one + &one;

    for _ in 0..limit {
        let basis = Int::sample_range(&two, &(candidate-&two));
        let mut y = Int::modpow(&basis, &d, candidate);

        if y == one || y == (candidate - &one) {
            continue;
        } else {
            for _ in one.clone()..s-one.clone() {
                y = Int::modpow(&y, &two, candidate);
                if y == one {
                    return false
                } else if y == candidate - &one {
                    break;
                }
            }
            return false;
        }
    }
    true
}

pub fn is_prime(n: i32) -> bool{
    if !div_small_primes(n){
        return false;
    }

     // Second, Fermat's little theo test on the candidate
    if !fermat(n) {
        return false;
    }

    // Finally, Miller-Rabin test
    if !miller_rabin(n, 5) {
        return false;
    }
    return true;
}*/