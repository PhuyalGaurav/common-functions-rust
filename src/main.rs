fn main() {
    // Factorial
    {
        const FACTNUM: u128 = 10;
        let nineteen_fact_iterative: u128 = factorial_iterative(FACTNUM);
        let nineteen_fact_recursive: u128 = factorial_recurcive(FACTNUM);
        println!("Factorial of {FACTNUM} using iterative process= {nineteen_fact_iterative}");
        println!("Factorial of {FACTNUM} using recursive process= {nineteen_fact_recursive}");
    }

    //Fibonacii Series
    {
        let fibo: Vec<u32> = fibonachii_series(10);
        for i in fibo {
            println!("{i}")
        }
    }
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

fn fibonachii_series(x: u32) -> Vec<u32> {
    let mut count: u32 = 0;
    let mut series: Vec<u32> = vec![];
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    series.push(a);
    series.push(b);
    while count != x {
        series.push(a + b);
        let temp: u32 = a;
        a = b;
        b = temp + b;
        count += 1;
    }
    return series;
}
