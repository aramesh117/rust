extern crate std;

pub fn fifty_three() -> u32 {
    let mut res: u32 = 0;
    for i in 1..101 {
        for j in 1..i + 1 {
            if binom_gt_mill(i, j) {
                res = res + 1;
            }
        }
    }
    res
}

pub fn binom_gt_mill(n: u32, k_prime: u32) -> bool {
    let k = std::cmp::min(k_prime, n - k_prime);
    let mut res: u64 = 1;
    for i in 1..k + 1 {
        res = res * ((n - i + 1) as u64) / (i as u64);
        if res > 1_000_000 {
            return true;
        }
    }
    return false;
}