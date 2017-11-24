extern crate primal;

pub fn thirty_nine() -> () {
    let sieve = primal::Sieve::new(10_000);
    for i in sieve.primes_from(1000).take_while(|p| *p < 10_000) {
        let i1 = i as i32;
        for j in sieve.primes_from(i + 1).take_while(|p| *p < 10_000) {
            let j1 = j as i32;
            for k in sieve.primes_from(j + 1).take_while(|p| *p < 10_000) {
                let k1 = k as i32;
                if i1 < j1 && j1 < k1 && k1 - j1 == j1 - i1 && are_permutations(i, j) && are_permutations(j, k) {
                    println!("{}{}{}", i, j, k);
                }
            }
        }
    }
}

pub fn are_permutations(i: usize, j: usize) -> bool {
    let i_string: String = i.to_string();
    let j_string: String = j.to_string();
    if i_string.len() != j_string.len() {
        return false;
    } else {
        let mut i_chars: Vec<char> = i_string.chars().collect();
        i_chars.sort_by(|a, b| { b.cmp(a) });
        let mut j_chars: Vec<char> = j_string.chars().collect();
        j_chars.sort_by(|a, b| { b.cmp(a) });
        i_chars == j_chars
    }
}