extern crate rand;
use rand::Rng;

fn predict_linear_classification(w:Vec<f32>, xk: &Vec<f32>)-> i8{
    let mut sum = w[0];
    for i in 0..xk.len(){
        sum += w[i + 1] * xk[i];
    }
    return if sum >= 0.0 { 1 } else { -1 }
}
fn train_rosenblatt(mut w: Vec<Vec<f32>>, x: Vec<Vec<f32>>, y:&mut[f32], nb_iter:&mut i32, alpha : &mut i32){
   let i : i16 = 0;
    for it in 0 .. nb_iter  {
       let k:i16 = 0; // Random à mettre
        let gxk: i32 = 0;
        //gxk = predict_linear_classification(w,x[k]);
        for i  in 0.. x[1].len()  {
            w[i+1] += alpha * (y[k] - gxk ) * x[k][i];
        }
        w[0] += alpha * (y[k] - gxk);
    }

fn main(){
    let w = vec![0.9, 0.8, 0.36];
    let xk = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    println!("{}",predict_linear_classification(w, &xk[0]));
}