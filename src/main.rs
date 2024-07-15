fn main() {
    let nineteen_fact_iterative: u128 = factorial_iterative(30);
    let nineteen_fact_recursive: u128 = factorial_recurcive(30);
    println!("Factorial of 19 using iterative process= {nineteen_fact_iterative}");
    println!("Factorial of 19 using recursive process= {nineteen_fact_recursive}");
}

fn factorial_iterative(x: u128) -> u128 {
    let mut fact: u128 = 1;
    let mut i: u128 = 1;
    loop {
        fact = fact * i;
        if x <= i {
            break;
        }
        i += 1;
    }
    return fact;
}

fn factorial_recurcive(x: u128) -> u128 {
    if x == 1 || x == 0 {
        return 1;
    } else {
        return x * factorial_recurcive(x - 1);
    }
}
