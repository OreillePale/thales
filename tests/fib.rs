use serde::Serialize; // necessary for implementing the bencher
use std::fmt::Debug;

use thales::Bencher;

fn stupid_fib(n: usize) -> usize{
    if n == 0 || n == 1{
        return n+1;
    }
    
    stupid_fib(n-1)+stupid_fib(n-2)

}

fn less_stupid_fib(n:usize) -> usize{
    if n == 0 || n == 1{
        return n+1;
    }
    
    let mut ns = Vec::with_capacity(n);

    ns.push(1);
    ns.push(2);

    for i in 2..n{
        let tmp = ns[i-1]+ns[i-2];
        ns.push(tmp);
    }

    ns[ns.len()-1]
}

fn golden_fib(n: usize) -> usize{
    let PHI: f64 = (5.0_f64.sqrt()+1.)/2.;
    let inv_sqrt5: f64 = 1./5.0_f64.sqrt();
    let PHI_1: f64 = (1.-PHI);

    let x = (PHI.powi(n.try_into().unwrap())-PHI_1.powi(n.try_into().unwrap()))*inv_sqrt5; 

    x.round() as usize
}

#[derive(Serialize,Debug,Clone)]
struct FibInput{
    n: usize
}

fn gen_fib_input(input: &FibInput) -> usize{
    input.n
}

#[test]
fn test_bencher() {
    let mut bencher = Bencher::new(3);

    let inputs = vec![1,10,30,40].iter().map(|i| FibInput{n:*i}).collect();

    bencher.bench("fib","stupid",&inputs,gen_fib_input,|x| stupid_fib(x));
    bencher.bench("fib","less stupid",&inputs,gen_fib_input,|x| less_stupid_fib(x));
    bencher.bench("fib","golden",&inputs,gen_fib_input,|x| less_stupid_fib(x));

    bencher.to_json("./tests/bench_fib.json");

    assert_eq!(1,1);
}