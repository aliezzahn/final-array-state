use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

const MOD: i64 = 1_000_000_007;

fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    if multiplier == 1 {
        return nums; // No changes needed if multiplier is 1
    }

    let n = nums.len();
    let mut k = k as usize; // Declare k as mutable
    let multiplier = multiplier as i64;

    // Min-heap to track the smallest elements and their indices
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = nums
        .iter()
        .enumerate()
        .map(|(i, &val)| Reverse((val as i64, i)))
        .collect();

    // Set to track unseen indices
    let mut unseen: HashSet<usize> = (0..n).collect();

    // Perform operations until all indices have been processed or k is exhausted
    while k > 0 && !unseen.is_empty() {
        if let Some(Reverse((num, idx))) = heap.pop() {
            // Multiply the smallest element by the multiplier
            let new_num = num * multiplier;
            heap.push(Reverse((new_num, idx)));
            unseen.remove(&idx);
            k -= 1;
        }
    }

    // Vector to store the final results
    let mut ans = vec![0; n];

    // Distribute remaining operations and compute final values
    for i in 0..n {
        if let Some(Reverse((num, idx))) = heap.pop() {
            // Calculate the number of operations for this index
            let operations = k / n + if i < k % n { 1 } else { 0 };
            // Compute the multiplier raised to the power of operations, modulo MOD
            let mlt = power_mod(multiplier, operations as i64, MOD);
            // Compute the final value for this index
            ans[idx] = ((num % MOD) * mlt) % MOD;
        }
    }

    // Convert the results back to i32
    ans.into_iter().map(|x| x as i32).collect()
}

// Modular exponentiation function
fn power_mod(base: i64, exp: i64, mod_val: i64) -> i64 {
    let mut result = 1;
    let mut base = base % mod_val;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod_val;
        }
        base = (base * base) % mod_val;
        exp /= 2;
    }
    result
}

fn main() {
    let nums = vec![2, 1];
    let k = 3;
    let multiplier = 10;

    let result = get_final_state(nums, k, multiplier);
    println!("Final state: {:?}", result);
}