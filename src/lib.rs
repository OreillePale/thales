use std::time::Instant;
use serde::Serialize;
use std::fmt::Debug;
use std::hint::black_box;
use std::time::{SystemTime, UNIX_EPOCH};

use std::fs::File;
use std::io::Write;

// a few utils
fn mean(xs: &[f64]) -> f64{
    xs.iter().sum::<f64>() / (xs.len() as f64)
}

fn std(xs: &[f64]) -> f64{
    let mu = mean(&xs);
    xs.iter().map(|x| (x-mu).powi(2)).sum::<f64>().sqrt() / ((xs.len() - 1 ) as f64)
}

#[derive(Serialize)]
struct Mark<S: Serialize+Debug+Clone>{
    groupname: String,
    name: String,
    input: S,
    t_mean: f64,
    t_std: f64
}

#[derive(Serialize)]
pub struct Bencher<S: Serialize+Debug+Clone>{
    marks: Vec<Mark<S>>,
    samples: usize,
    version: String,
    unix_timestamp: u64
}

impl<S: Serialize+Debug+Clone> Bencher<S>{
    pub fn new(samples: usize) -> Self{
        Self{
            marks: Vec::new(),
            samples: samples,
            version: String::from("0.1.0"),
            unix_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }

    pub fn bench<G, F, GOutput, FOutput>(
            &mut self,
            groupname: &str,
            name: &str,
            inputs: &Vec<S>,
            generate: G, 
            func: F
        )
    where   
        G: FnOnce(&S) -> GOutput + Clone, // Generator function type
        F: FnOnce(GOutput) -> FOutput + Clone, // Function to benchmark type
    {
        // loop over all inputs
        println!("* benching {}/{} with {} repetitions", groupname, name, self.samples);
        for i in 0..inputs.len(){
            let mut times: Vec<f64> = Vec::with_capacity(self.samples);
            
            for j in 0..self.samples{
                // prepare function input
                let finput = generate.clone()(&inputs[i]);

                // evaluate 
                let start = Instant::now();
                black_box(func.clone()(finput));
                let duration = (start.elapsed().as_nanos() as f64)/1.0e9;

                times.push(duration);
            }

            // todo: Weibull ?
            // ndlr: assuming gaussian distribution positive quantity is not physical
            let mu = mean(&times);
            let std = std(&times);

            self.marks.push(Mark::<S>{
                groupname: groupname.to_string(),
                name: name.to_string(),
                input: inputs[i].clone(),
                t_mean: mu,
                t_std: std
            });
            
            println!("  * {:?}\t{:.2e}({:.2e}) s",inputs[i],mu,std);
        }
    }

    pub fn to_json(&self, path: &str){
        println!("* saving result to {}",path);
        let mut f = File::create(path).expect("Unable to create file");
        f.write(serde_json::to_string(&self).unwrap().as_bytes()).expect("Unable to write data");
    }
}


