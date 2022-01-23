fn sum_of_multiples(n: i32) -> i32 {
    return (n + 999 - 999 % n) * (999 / n) / 2;
}

fn main() {
    let ans = sum_of_multiples(3) + sum_of_multiples(5) - sum_of_multiples(15);
    println!("{}", ans);
}
