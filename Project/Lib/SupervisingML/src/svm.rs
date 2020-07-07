use std::slice::{from_raw_parts, from_raw_parts_mut};
extern crate nalgebra;
use nalgebra::DMatrix;

// SVM
#[no_mangle]
pub extern "C" fn train_svm_model(x_ptr: *mut f64, y_ptr: *mut f64, x_size: usize, sample_size: usize) {
    let x;
    let y;
    unsafe {
        x = from_raw_parts(x_ptr , x_size * sample_size);
        y = from_raw_parts(y_ptr, sample_size);
    }
    
    //let w;
}

#[no_mangle]
pub extern "C" fn predict_svm_model(w_ptr: *mut f64, x_ptr: *mut f64, size: usize) -> f64 {
    let x;
    let w;
    unsafe {
        x = from_raw_parts(x_ptr , size);
        w = from_raw_parts(w_ptr, size);
    }

    if predict_svm_model_value(w, x, size) >= 0.0 {
        return 1.0;
    }
    else {
        return -1.0;
    }
}

//On cherche value =  0 pour avoir le max
fn predict_svm_model_value(w: &[f64], x: &[f64], size: usize) -> f64 {
    let xm = DMatrix::from_row_slice(size, 1, &x);
    let wm = DMatrix::from_row_slice(size, 1, &w);
    let result_matrix = wm.transpose() * &xm + wm.row(0);
    return result_matrix.data.as_vec().to_vec().into_boxed_slice()[0];
}