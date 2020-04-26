extern crate nalgebra;
extern crate rand;
mod lib;
use rand::Rng;
use nalgebra::DMatrix;


fn main(){
    //classification
    let mut w = lib::generate_rand_f32vector(3, 0.0, 1.0);
    let x = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    let y = vec![-1,1];
    
    println!("{:?}",w);
    lib::train_linear_model_classification(&mut w, &x, &y, 15, 0.1);
    println!("{:?}",w);

    //regression
    let mut x = [0f32;40];
    let mut y = [0f32;20]; 
    let mut rng = rand::thread_rng();
    for i in 0..20{
        x[i * 2] = 1.0;
        x[i * 2 + 1]  = (i + 1) as f32;  
        y[i] = 1.5 * ((i + 1) as f32) + 0.75 + ((rng.gen_range(0.0,1.0) - 0.35) * 2.0);
    }
    let xm = DMatrix::from_row_slice(20,2,&x);
    let ym = DMatrix::from_row_slice(20,1,&y);
    let w = (((xm.transpose() * &xm).try_inverse()).unwrap() * xm.transpose()) * ym;
    println!("{:?}",w);
}