extern crate rand;
extern crate nalgebra;
use rand::Rng;
//use nalgebra::DMatrix;

#[no_mangle]
pub extern fn init_linear_model(size: i16, start: f32, end: f32) -> Vec<f32>{
    let mut vector: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    for _it in 0..size{
        vector.push(rng.gen_range(start, end));
    }
    return vector;
}

#[no_mangle]
pub extern fn predict_linear_model_classification(w:&Vec<f32>, xk:&Vec<f32>)-> i8{
    let mut sum = w[0];
    for i in 0..xk.len(){
        sum += w[i + 1] * xk[i];
    }
    return if sum >= 0.0 { 1 } else { -1 }
}

#[no_mangle]
pub extern fn train_linear_model_classification(w:&mut Vec<f32>, x:&Vec<Vec<f32>>, y:&Vec<i8>, nb_iter:i32, alpha:f32) {
    for _it in 0..nb_iter {
        let k = 0;
        let gxk = predict_linear_model_classification(w,&x[k]);
        for i in 0..x[1].len() {
            w[i + 1] += alpha * (y[k] - gxk as i8) as f32 * x[k][i];
        }
        w[0] += alpha * (y[k] - gxk as i8) as f32;
    }
}


