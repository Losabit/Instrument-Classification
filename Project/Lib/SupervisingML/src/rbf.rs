use std::slice::{from_raw_parts};
use nalgebra::{DMatrix};


#[no_mangle]
pub extern "C" fn predict_rbf(w_ptr: *const f64, start_x_ptr: *const f64, x_ptr: *const f64, gamma: f64, model_size: f64, input_per_sample: f64) -> f64{
    let x;
    let w;
    let start_x;
    let model_size = model_size as usize;
    let input_per_sample = input_per_sample as usize;
    unsafe {
        w = from_raw_parts(w_ptr, model_size);
        x = from_raw_parts(x_ptr, input_per_sample);
        start_x = from_raw_parts(start_x_ptr, model_size * input_per_sample);
    }
   /*
    println!("model : {:?}", w);
    println!("x_start : {:?}", start_x);
    println!("x : {:?}", x);
    println!("gamma : {:?}", gamma);
    println!("model_size : {:?}", model_size);
    println!("input_per_sample : {:?}", input_per_sample);
*/
    let mut sum = 0.0;
    for i in 0..model_size{
        let mut vector_x = Vec::new();
        for k in 0..x.len(){
            vector_x.push(x[k] - start_x[input_per_sample * i + k]);
        }
        let alpha = DMatrix::from_row_slice(x.len(), 1, &vector_x);
        let toexp = -gamma * (alpha.norm() * alpha.norm());
        let result = w[i] * toexp.exp();
        //println!("toexp : {:?}, toexp.exp() : {:?}", toexp, toexp.exp());
        sum += result;
        
    }
    return sum;
}

#[no_mangle]
pub extern "C" fn train_rbf(x_ptr: *const f64, y_ptr: *const f64, input_per_sample: f64, nb_sample: f64, gamma: f64) -> *mut f64{
    let x;
    let y;
    let input_per_sample = input_per_sample as usize;
    let nb_sample = nb_sample as usize;
    unsafe {
        x = from_raw_parts(x_ptr, input_per_sample * nb_sample);
        y = from_raw_parts(y_ptr, input_per_sample);
    }
    
    /*
    println!("x : {:?}", x);
    println!("size : {:?}", input_per_sample);
    println!("input_shape : {:?}", nb_sample);
    println!("gamma : {:?}", gamma);
    println!("y : {:?}", y);
    */
    
    let xm =  DMatrix::from_row_slice(input_per_sample , nb_sample ,x);
    let ym = DMatrix::from_row_slice(input_per_sample , 1, y);
    let mut phi = Vec::new();
    
    for i in 0.. input_per_sample  {
        for j in  0.. input_per_sample  {
            let mut sum = Vec::new();
            for k in 0..nb_sample{
                sum.push(xm.row(i)[k] - xm.row(j)[k])
            }

            let alpha = DMatrix::from_row_slice(nb_sample,1,&sum);
            let toexp = -gamma * (alpha.norm() * alpha.norm());
            phi.push(toexp.exp());
        }
    }
    let phi_mat = DMatrix::from_row_slice(input_per_sample,input_per_sample,&phi);

    /*
    println!("{:?}", phi_mat);
    */

    let wm = phi_mat.try_inverse().unwrap() * ym;
    let mut slice = wm.data.as_vec().to_vec().into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr
}

/*
fn get_centroids(x: &[f64], input_per_sample: usize, nb_sample: usize){

}
*/