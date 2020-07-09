extern crate rand;
extern crate nalgebra;
use rand::Rng;
use nalgebra::DMatrix;
use std::slice::{from_raw_parts, from_raw_parts_mut};

//Linéaire
#[no_mangle]
pub extern fn init_linear_model(size: usize) -> *mut f64 {
    let mut vector = Vec::new();
    let mut rng = rand::thread_rng();

    for _it in 0..size+1 {
        vector.push(rng.gen_range(-1.0, 1.0) as f64);
    }
    let mut slice = vector.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr
}

fn predict_linear_model_regression_(model: &[f64], x: &[f64], x_size: usize)-> f64{
    let mut sum = model[0];
    for i in 0..x_size {
        sum += model[i + 1] * x[i]
    }
    return sum;
}

fn predict_linear_model_classification_(model: &[f64], x: &[f64], x_size: usize)-> f64{
    return if predict_linear_model_regression_(model, x, x_size) >= 0.0 { 1 } else { -1 } as f64;
}

#[no_mangle]
pub extern "C" fn predict_linear_model_regression(model_ptr: *mut f64, x_ptr: *mut f64, x_size: usize)-> f64{
    let model;
    let x;
    unsafe {
        model = from_raw_parts(model_ptr, x_size + 1);
        x = from_raw_parts(x_ptr, x_size);
    }
    return predict_linear_model_regression_(model, x, x_size)
}

#[no_mangle]
pub extern "C" fn predict_linear_model_classification(model_ptr: *mut f64, x_ptr: *mut f64, x_size: usize)-> f64{
    return if predict_linear_model_regression(model_ptr, x_ptr, x_size) >= 0.0 { 1 } else { -1 } as f64;
}


#[no_mangle]
pub extern "C" fn train_linear_model_regression(x_ptr: *mut f64, y_ptr: *mut f64, x_size: usize) -> *mut f64{
    let x;
    let y;
    unsafe {
        x = from_raw_parts(x_ptr, x_size);
        y = from_raw_parts(y_ptr, x_size / 2);
    }
    
    let xm = DMatrix::from_row_slice(x_size / 2, 2, &x);
    let ym = DMatrix::from_row_slice(x_size / 2, 1, &y);
    let w_matrix = (((xm.transpose() * &xm).try_inverse()).unwrap() * xm.transpose()) * ym;
    let mut slice = w_matrix.data.as_vec().to_vec().into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr
}

#[no_mangle]
pub extern "C" fn train_linear_model_classification(w: *mut f64, x: *mut f64, y: *mut f64, sample_size: usize, result_size: usize, nb_iter: usize, alpha:f64) {
    let mut rng = rand::thread_rng();
    let model;
    let dataset_inputs;
    let dataset_outputs;

    unsafe {
        model = from_raw_parts_mut(w , result_size + 1);
        dataset_inputs = from_raw_parts(x, sample_size * result_size);
        dataset_outputs = from_raw_parts(y, sample_size);
    }

    for _it in 0..nb_iter {
        let k = rng.gen_range(0, sample_size);
        let index_k = k * result_size;
        let inputs_k = &dataset_inputs[index_k..(index_k + result_size)];
        let output_k = dataset_outputs[k];
        let gxk = predict_linear_model_classification_(model, inputs_k, result_size);
        
        for i in 0..result_size {
            model[i + 1] += alpha * (output_k - gxk )  * inputs_k[i];
        }
        model[0] += alpha * (output_k - gxk ) ;
    }
}
