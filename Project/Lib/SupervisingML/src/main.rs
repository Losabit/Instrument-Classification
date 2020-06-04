extern crate rand;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use rand::Rng;
mod lib;
//use rand::Rng;


fn main(){
//linear
    //classification
/*
    let mut w = lib::init_linear_model(3, 0.0, 1.0);
    let x = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    let y = vec![-1,1];
    
    println!("{:?}",w);
    lib::train_linear_model_classification(&mut w, &x, &y, 15, 0.1);
    println!("{:?}",w);
*/
    //regression

    // let mut x = vec![];
    // let mut y = vec![];
    // let mut rng = rand::thread_rng();
    // for i in 0..20{
    //     x.push(1.0);
    //     x.push((i + 1) as f32);
    //     y.push(1.5 * ((i + 1) as f32) + 0.75 + ((rng.gen_range(0.0,1.0) - 0.35) * 2.0));
    // }
    // let w = lib::train_linear_model_regression(x.as_mut_ptr(),y.as_mut_ptr(),40);
    // println!("{:?}",w);


//multicouche 
    //classification
    let mut neurone_by_couche = [2,3,2,1];
    let size = lib::get_model_size(neurone_by_couche.as_mut_ptr(), 4);
    let model_ptr = lib::init_multicouche_model(neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
    let model;
    unsafe{
        model = from_raw_parts(model_ptr, size);
    }
    println!("{:?}",model);
    println!("{:?}",size);

    let x = vec![1.0, 0.4, 0.7];
    let y = vec![-1,1];
   //lib:init_multicouche_model( &[1.0, 0.4, 0.7])
}