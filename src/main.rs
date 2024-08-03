use std::mem::swap;

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
            print!("{i}, ")
        }
        println!("");
    }

    // Least element
    {
        let numbers: Vec<u32> = [32, 45, 3, 43, 2, 12, 34, 23, 12, 325].to_vec();
        let minimum: u32 = minimum_element(numbers);
        println!("Minimum Element : {minimum}");
    }

    // Check Prime Numbers
    {
        let number: u32 = 32;
        if check_prime(number) {
            println!("{number} is a prime number");
        } else {
            println!("{number} is not a prime number");
        }
    }

    let random_vector: Vec<usize> = vec![1, 12, 4, 5, 324, 6, 234, 123, 6, 7];
    arr_sort(&random_vector);
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

fn minimum_element(array: Vec<u32>) -> u32 {
    if array.is_empty() {
        panic!("No number to find lowest")
    }
    let array2: Vec<u32> = array;
    let mut least: u32 = u32::MAX;
    for i in array2 {
        if i < least {
            least = i;
        }
    }
    return least;
}

fn check_prime(n: u32) -> bool {
    let mut i: u32 = 1;
    let mut count: u32 = 0;
    loop {
        if n % i == 0 {
            count = count + 1;
        }
        if i == n {
            break;
        }
        i += 1;
    }
    if count == 2 {
        return true;
    } else {
        return false;
    }
}

fn arr_sort(array: &Vec<usize>) {
    let mut new_array: Vec<usize> = array.clone();
    for (index, i) in new_array.iter().enumerate() {
        for (index2, j) in new_array.iter().enumerate() {
            if j < i {
                let temp: usize = new_array[index];
                new_array[index2] = temp;
            }
        }
    }
}
