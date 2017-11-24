extern crate primal;

pub fn thirty_eight() -> () {
    // Four consecutive:
    let beginning = 644;
    let mut i = beginning;
    let mut finished: bool = false;
    let sieve = primal::Sieve::new(1 << 20);
    while !finished {
        let mut all_distinct = true;
        for j in 0..4 {
            if all_distinct {
                let factors = sieve.factor(i + j);
                match factors {
                    Ok(vec) => {
                        if i + j == 134047 {
                            println!("{}", vec.len());
                        }
                        all_distinct &= vec.len() == 4;
                    }
                    Err(_) => {
                        println!("Error!");
                        finished = true;
                    }
                }
            }
        }
        finished = all_distinct;
        i = i + 1;
    }
    println!("The first of the sequence is {}.", i - 1);
    for j in 0..4 {
        println!("{}", i + j - 1);
        let factors = sieve.factor(i + j - 1);
        factors.ok().unwrap().iter().for_each(|x| {
            println!("{}^{}", x.0, x.1);
        });
        println!("\n")
    }
}

