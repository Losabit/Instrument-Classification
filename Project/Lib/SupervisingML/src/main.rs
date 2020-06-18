extern crate rand;
use std::slice::{from_raw_parts};
mod lib;
use rand::Rng;


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
    let mut neurone_by_couche = [2,2,1];
    let size = lib::get_model_size(neurone_by_couche.as_mut_ptr(), neurone_by_couche.len());
    let model_ptr = lib::init_multicouche_model(neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
    let model;
    unsafe{
        model = from_raw_parts(model_ptr, size);
    }
    println!("{:?}",model);

    let mut x = vec![
    0.0, 0.0, 
    1.0, 0.0, 
    0.0, 1.0, 
    1.0, 1.0];
    let mut y = vec![-1, 1, 1, -1];
    lib::train_multicouche_model_classification(model_ptr, x.as_mut_ptr(), y.as_mut_ptr(), neurone_by_couche.as_ptr(), neurone_by_couche.len(), y.len(), 1000, 0.1);
    
    let predict_value;
    let predict_value_2;
    let predict_value_3;
    let predict_value_4;
    let predict_value_ptr = lib::predict_multicouche_model_classification(model_ptr,  x[0..2].as_mut_ptr(), neurone_by_couche.as_ptr(),  neurone_by_couche.len());
    let predict_value_2_ptr = lib::predict_multicouche_model_classification(model_ptr,  x[2..4].as_mut_ptr(), neurone_by_couche.as_ptr(),  neurone_by_couche.len());
    let predict_value_3_ptr = lib::predict_multicouche_model_classification(model_ptr,  x[4..6].as_mut_ptr(), neurone_by_couche.as_ptr(),  neurone_by_couche.len());
    let predict_value_4_ptr = lib::predict_multicouche_model_classification(model_ptr,  x[6..8].as_mut_ptr(), neurone_by_couche.as_ptr(),  neurone_by_couche.len());
    unsafe{
        predict_value = from_raw_parts(predict_value_ptr, 1);
        predict_value_2 = from_raw_parts(predict_value_2_ptr, 1);
        predict_value_3 = from_raw_parts(predict_value_3_ptr, 1);
        predict_value_4 = from_raw_parts(predict_value_4_ptr, 1);
    }
    println!("predict value 1 = {:?} for {:?}",predict_value, &x[0..2]);
    println!("predict value 2 = {:?} for {:?}",predict_value_2, &x[2..4]);
    println!("predict value 3 = {:?} for {:?}",predict_value_3, &x[4..6]);
    println!("predict value 4 = {:?} for {:?}",predict_value_4, &x[6..8]);
}