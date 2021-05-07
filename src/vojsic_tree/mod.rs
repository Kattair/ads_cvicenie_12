pub fn solution(n: usize) -> u128 {
    let mut res = 0;

    if n == 0 {
        return 1;
    }

    let x = std::cmp::min(3, n - 1);
    for k in 0..(x + 1) {
        res += solution(k) * solution(n - 1 - k);
    }
    res
}

pub fn solution_optimized(n: usize) -> u128 {
    let mut result = vec![1 as u128; n + 1];

    for i in 2..(n + 1) {
        result[i] = 0;

        let x = std::cmp::min(3, i - 1);
        for k in 0..(x + 1) {
            result[i] += result[k] * result[i - 1 - k];
        }
    }

    result[n]
}