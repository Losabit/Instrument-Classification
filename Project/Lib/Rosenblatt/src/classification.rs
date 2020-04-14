extern crate rand;
use rand::Rng;

fn predict_linear_classification(w:Vec<f32>, xk: &Vec<f32>)-> i8{
    let mut sum = w[0];
    for i in 0..xk.len(){
        sum += w[i + 1] * xk[i];
    }
    return if sum >= 0.0 { 1 } else { -1 }
}

fn main(){
    let w = vec![0.9, 0.8, 0.36];
    let xk = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    println!("{}",predict_linear_classification(w, &xk[0]));
}