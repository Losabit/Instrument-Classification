extern crate rand;
use rand::Rng;
use std::slice::{from_raw_parts};

mod rbf;
mod mlp;
mod linear;
mod svm;
//mod rbf;


fn main(){
//linear
    //classification
    /*
    let good_y;
    let mut y : Vec<f64> = vec![
        0.0,0.0,0.0,
        0.0,0.0,1.0,
        0.0,1.0,0.0,
        1.0,0.0,0.0,
        0.0,0.0,1.0,
        0.0,1.0,0.0,
        1.0,0.0,0.0
        ];

    unsafe {
        good_y = from_raw_parts(linear::output2d_to_1d(y.as_mut_ptr(),7, 3), 7);
    }    
    println!("{:?}", good_y);

    let mut w = linear::init_linear_model(3, 0.0, 1.0);
    let x = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    let y = vec![-1,1];
    
    println!("{:?}",w);
    linear::train_linear_model_classification(&mut w, &x, &y, 15, 0.1);
    println!("{:?}",w);
*/
    //regression
/*
    let mut x : Vec<f64> = vec![];
    let mut y : Vec<f64> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..20{
        x.push(1.0);
        x.push((i + 1) as f64);
        y.push(1.5 * ((i + 1) as f64) + 0.75 + (( - 0.35) * 2.0));
    }
*/
/*
    let mut x = vec![1.0,1.0,2.0,2.0, 3.0, 1.0];
    let mut y = vec![2.0,3.0,2.5];

    let model;
    let model_ptr = linear::train_linear_model_regression(x.as_mut_ptr(),y.as_mut_ptr(),x.len());

    unsafe{
        model = from_raw_parts(model_ptr, 3);
    }
    println!("{:?}",&model);
    println!("{:?}", linear::predict_linear_model_regression(model_ptr, x.as_mut_ptr(), 2));

*/
//multicouche 
         
    // let mut neurone_by_couche = [1.0,6.0,1.0];
    // let mut rng = rand::thread_rng();
    // let size = mlp::get_model_size(neurone_by_couche.as_mut_ptr(), neurone_by_couche.len());
    // let mut model_ptr = mlp::init_multicouche_model(neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
    // let mut model;
    // unsafe{
    //     model = from_raw_parts(model_ptr, size);



    //classification  
    /* 
    let mut x = vec![];
    for _ in 0..1000{
        x.push(rng.gen_range(0.0,1.0) * 2.0 - 1.0)
    }
    let mut y = vec![];
    for i in 0..500{
        if -x[i * 2] - x[i * 2 + 1] - 0.5 > 0.0 && x[i * 2 + 1] < 0.0 && x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0{
            y.push(1.0);
            y.push(0.0);
            y.push(0.0);
        }
        else if -x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0 && x[i * 2 + 1] > 0.0 && x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0{
            y.push(0.0);
            y.push(1.0);
            y.push(0.0);
        }
        else if -x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0 && x[i * 2 + 1] < 0.0 && x[i * 2] - x[i * 2 + 1] - 0.5 > 0.0{
            y.push(0.0);
            y.push(0.0);
            y.push(1.0);
        }
        else{
            y.push(0.0);
            y.push(0.0);
            y.push(0.0);
        }
    }
    
    model_ptr = mlp::train_multicouche_model_classification(model_ptr, x.as_mut_ptr(), y.as_mut_ptr(), neurone_by_couche.as_mut_ptr(), neurone_by_couche.len(), y.len() / 3, 10000.0, 0.1);
    unsafe{
        model = from_raw_parts(model_ptr, size);
    }
    println!("{:?}",model);

    let mut predict_value;
    for i in 0..20 {
        let predict_value_ptr = mlp::predict_multicouche_model_classification(model_ptr,  x[i * 2..(i + 1) * 2].as_mut_ptr(), neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
        unsafe{
            predict_value = from_raw_parts(predict_value_ptr, 3); 
        }
        println!("predict value {:?} = {:?} for {:?} / {:?}", i + 1, predict_value, &x[i * 2..(i + 1) * 2], &y[i * 3..(i + 1) * 3]);
    }
    */

    //regression
    // let mut x = vec![1.0,2.0,3.0];
    // let mut y = vec![2.0,3.0,2.5];
    //
    // model_ptr = mlp::train_multicouche_model_regression(model_ptr, x.as_mut_ptr(), y.as_mut_ptr(), neurone_by_couche.as_mut_ptr(), neurone_by_couche.len(), y.len(), 60000.0, 0.1);
    // unsafe{
    //     model = from_raw_parts(model_ptr, size);
    // }
    // println!("{:?}",model);
    //
    // let mut predict_value;
    // for i in 0..3 {
    //     let predict_value_ptr = mlp::predict_multicouche_model_regression(model_ptr,  [x[i]].as_mut_ptr(), neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
    //     unsafe{
    //         predict_value = from_raw_parts(predict_value_ptr,1);
    //     }
    //     println!("predict value {:?} = {:?} for {:?} / {:?}", i + 1, predict_value, &x[i], &y[i]);
    // }

<<<<<<< Updated upstream
    /// RBF
    let mut  x = [3.0,3.0,3.0,3.0,3.0];
    let mut y  = [2.0,2.0,2.0,2.0,2.0];
    let sample = 5 ;
    let nbPerSemple =3;
    let gamma = 0.01;
    //rbf::train_native_rbf(*x,*y,sample,nbPerSemple,gamma);
=======
    // RBF
    let mut  x = [-1.0,2.0,1.0,-1.0,-1.0,1.0];
    let mut y  = [-1.0,1.0,-1.0];
    let sample = 3 ;
    let nbPerSemple =2;
    let gamma = 1.0;
    let r = train_native_rbf(&x,&y,sample,nbPerSemple,gamma);
    println!("{:?} ",r.w);

>>>>>>> Stashed changes
   
    //SVM
    let mut x = vec![1.0,3.0,2.0,2.0,3.0,2.0];
    let mut y = vec![-1.0,1.0,1.0]; 
    let model_ptr = svm::train_svm_model(x.as_mut_ptr(), y.as_mut_ptr(), 2, 3);
    let model;
    unsafe{
        model = from_raw_parts(model_ptr, 3);
    }
    println!("model : {:?}",model);

    for i in 0..3{
        println!("predict : {:?} / result : {:?}", svm::predict_svm_model(model_ptr, x[i * 2..(i + 1) * 2].as_mut_ptr(), 3), y[i]);
    }
}