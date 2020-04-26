extern crate rand;
extern crate nalgebra;
use rand::Rng;
use nalgebra::DMatrix;

#[no_mangle]
pub extern fn generate_rand_f32vector(size: i16, start: f32, end: f32) -> Vec<f32>{
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
        let mut rng = rand::thread_rng();
        let k = rng.gen_range(0, x[0].len());
        let gxk = predict_linear_model_classification(w,&x[k]);
        for i in 0..x[1].len() {
            w[i + 1] += alpha * (y[k] - gxk as i8) as f32 * x[k][i];
        }
        w[0] += alpha * (y[k] - gxk as i8) as f32;
    }
}
#[no_mangle]
pub extern fn train_linear_model_regression(x: &mut [f32], y: &mut [f32]){
    assert_eq!(x.len()/2, y.len());
    let mut rng = rand::thread_rng();
    for i in 0..y.len(){
        x[i * 2] = 1.0;
        x[i * 2 + 1]  = (i + 1) as f32;
        y[i] = 1.5 * ((i + 1) as f32) + 0.75 + ((rng.gen_range(0.0,1.0) - 0.35) * 2.0);
    }
    let xm = DMatrix::from_row_slice(y.len(),2,&x);
    let ym = DMatrix::from_row_slice(y.len(),1,&y);
    let w = (((xm.transpose() * &xm).try_inverse()).unwrap() * xm.transpose()) * ym;
    println!("{:?}",w);
}

