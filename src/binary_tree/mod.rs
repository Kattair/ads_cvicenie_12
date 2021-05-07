pub fn solution(n: usize) -> usize {
    let mut res = 0;

    if n == 0 {
        return 1;
    }

    for k in 0..n {
        res += solution(k) * solution(n  - 1 - k);
    }
    res
}

pub fn solution_optimized(n: usize) -> usize {
    let mut result = vec![1; n + 1];

    for i in 2..(n + 1) {
        result[i] = 0;

        for k in 0..i {
            result[i] += result[k] * result[i - 1 - k];
        }
    }

    result[n]
}