extern crate rand;
extern crate nalgebra;
use rand::Rng;
use nalgebra::DMatrix;
use std::slice::{from_raw_parts, from_raw_parts_mut};

//Linéaire
#[no_mangle]
pub extern fn init_linear_model(size: usize) -> *mut f32 {
    let mut vector: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    vector.push(1.0);

    for _it in 0..size {
        vector.push(rng.gen_range(-1.0, 1.0));
    }
    return vector.into_boxed_slice().as_mut_ptr();
}

fn predict_linear_model_regression_(model: &[f32], x: &[f32], x_size: usize)-> f32{
    let mut sum = model[0];
    for i in 0..x_size {
        sum += model[i + 1] * x[i]
    }
    return sum;
}

fn predict_linear_model_classification_(model: &[f32], x: &[f32], x_size: usize)-> f32{
    return if predict_linear_model_regression_(model, x, x_size) >= 0.0 { 1 } else { -1 } as f32;
}

#[no_mangle]
pub extern "C" fn predict_linear_model_regression(model_ptr: *mut f32, x_ptr: *mut f32, x_size: usize)-> f32{
    let model;
    let x;
    unsafe {
        model = from_raw_parts(model_ptr, x_size + 1);
        x = from_raw_parts(x_ptr, x_size);
    }
    return predict_linear_model_regression_(model, x, x_size)
}

#[no_mangle]
pub extern "C" fn predict_linear_model_classification(model_ptr: *mut f32, x_ptr: *mut f32, x_size: usize)-> f32{
    return if predict_linear_model_regression(model_ptr, x_ptr, x_size) >= 0.0 { 1 } else { -1 } as f32;
}

#[no_mangle]
pub extern "C" fn train_linear_model_regression(x_ptr: *mut f32, y_ptr: *mut f32, x_size: usize) -> *mut f32{
    let x;
    let y;
    unsafe {
        x = from_raw_parts(x_ptr, x_size);
        y = from_raw_parts(y_ptr, x_size / 2);
    }

    let xm = DMatrix::from_row_slice(x_size / 2, 2, &x);
    let ym = DMatrix::from_row_slice(x_size / 2, 1, &y);
    let w_matrix = (((xm.transpose() * &xm).try_inverse()).unwrap() * xm.transpose()) * ym;
    return w_matrix.data.as_vec().to_vec().into_boxed_slice().as_mut_ptr();
}

#[no_mangle]
pub extern "C" fn train_linear_model_classification(w: *mut f32, x: *mut f32, y: *mut f32, sample_size: usize, result_size: usize, nb_iter: usize, alpha:f32) {
    let mut rng = rand::thread_rng();
    let model;
    let dataset_inputs;
    let dataset_outputs;

    unsafe {
        model = from_raw_parts_mut(w, result_size + 1);
        dataset_inputs = from_raw_parts(x, sample_size * result_size);
        dataset_outputs = from_raw_parts(y, sample_size)
    }

    for _it in 0..nb_iter {
        let k = rng.gen_range(0, sample_size);
        let index_k = k * result_size;
        let  inputs_k = &dataset_inputs[index_k..(index_k + result_size)];
        let output_k = dataset_outputs[k];
        let gxk = predict_linear_model_classification_(model, inputs_k, result_size);
        for i in 0..result_size {
            model[i + 1] += alpha * (output_k - gxk )  * inputs_k[i];
        }
        model[0] += alpha * (output_k - gxk ) ;
    }
}

//Multi-couche
fn size_of_couche(w: &Vec<Vec<Vec<f32>>>, couche: usize, count_biais: bool) -> usize {
    if couche < w.len() {
        return if count_biais == true { w[couche].len() } else { w[couche].len() - 1}
    }
    else if couche == w.len() {
        return if count_biais == true { w[couche - 1][0].len() + 1 } else { w[couche - 1][0].len() }
    }
    else {
        return 0;
    }
}

fn calculate_signal(w:&Vec<Vec<f32>>, x:&Vec<f32>, neurone:usize) -> f32 {
    let mut value = 0.0;
    for i in 0..w.len(){
        value += w[i][neurone] * x[i];
    }
    return value;
}

pub fn init_out_neurone(model: &Vec<Vec<Vec<f32>>>, x: &mut Vec<Vec<f32>>){
    for couche in 0..model.len(){
        let mut vector : Vec<f32> = vec![];
        vector.push(1.0);
        for neurone in 0..size_of_couche(&model, couche + 1, false){
            vector.push(calculate_signal(&model[couche], &x[couche], neurone).tanh());
        }
        x.push(vector);
    }
}

pub fn refresh_out_neurone(model: &Vec<Vec<Vec<f32>>>, x: &mut Vec<Vec<f32>>){
    for couche in 1..x.len(){
        for neurone in 1..x[couche].len(){
            x[couche][neurone] = calculate_signal(&model[couche - 1], &x[couche - 1], neurone - 1).tanh();
        }
    }
}

#[no_mangle]
pub extern fn gradien_retropropagation (w : &Vec<f32>, x: &f32, sigma:f32) -> f32 {
    let mut sum = 0.0;
    for i in 1..w.len(){
        sum += w[i] * sigma;
    }
    let sig = (1.0 - x.powf(2.0) ) * sum;
    return sig;
}

#[no_mangle]
pub extern  fn gradien_retropropagation_last_classification (y: i8, xlj: f32 ) -> f32 {
    let result :f32 = (1.0 - xlj.powf(2.0) ) * (xlj - y as f32);
    return result;
}

#[no_mangle]
pub extern  fn gradien_retropropagation_last_regression (y: f32, xlj: f32 ) -> f32 {
    let result :f32 = xlj - y;
    return result;
}

#[no_mangle]
pub extern fn init_multicouche(neurones_by_couche: &[usize], start: f32, end: f32) -> Vec<Vec<Vec<f32>>> {
    let mut model: Vec<Vec<Vec<f32>>> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..neurones_by_couche.len() - 1{
        model.push(vec![]);

        let mut vector_biais: Vec<f32> = vec![];
        for _it in 0..neurones_by_couche[i + 1]{
            vector_biais.push(1.0)
        }
        model[i].push(vector_biais);

        for _j in 0..neurones_by_couche[i]{
            let mut vector: Vec<f32> = vec![];
            for _it in 0..neurones_by_couche[i + 1]{
                vector.push(rng.gen_range(start, end));
            }
            model[i].push(vector);
        }
    }
    return model;
}

#[no_mangle]
pub extern fn train_multicouche_model_classification(w: &mut Vec<Vec<Vec<f32>>>, x: &mut Vec<Vec<f32>>, y: &Vec<i8>,  nb_iter:usize, alpha:f32 ) -> Vec<f32>{
    assert_eq!(x.len() - 1, w.len());
    init_out_neurone(&w, x);
    let mut sigma : Vec<Vec<f32>> = vec![vec![]];
    for i in 0..y.len(){
        sigma[0].push(gradien_retropropagation_last_classification(y[i], x[x.len() - 1][i]));
    }
    let last_couche = sigma.len() - 1;
    
    for _it in 0..nb_iter{
        for couche in 0..w.len(){
            for neurone in 0..w[couche].len(){
                for next_neurone in 0..w[couche][neurone].len(){
                    w[couche][neurone][next_neurone] = w[couche][neurone][next_neurone] - (alpha * x[couche][neurone] * sigma[couche][neurone]);
                }  
            }    
        }
        refresh_out_neurone(&w, x);
        for i in 0..sigma[last_couche].len() {
            sigma[last_couche][i] = gradien_retropropagation_last_classification(y[i], x[x.len() - 1][i]);
        }
        for couche in (0..last_couche).rev() {
            for neurone in 0..sigma[couche].len() {
                //sigma[couche][neurone] = gradien_retropropagation()
            }
        }
    }
    
    return vec![];
}