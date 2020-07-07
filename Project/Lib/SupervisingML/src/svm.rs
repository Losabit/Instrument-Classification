use std::slice::{from_raw_parts, from_raw_parts_mut};
use osqp::{CscMatrix, Problem, Settings};
extern crate nalgebra;
use nalgebra::DMatrix;


// SVM
// https://docs.rs/osqp/0.6.0/osqp/
#[no_mangle]
pub extern "C" fn train_svm_model(x_ptr: *mut f64, y_ptr: *mut f64, dimension: usize, sample_size: usize) -> *mut f64{
    let x;
    let y;
    unsafe {
        x = from_raw_parts(x_ptr , dimension * sample_size);
        y = from_raw_parts(y_ptr, sample_size);
    }

    // Min 1/2 * X(transpos) * PX + q(transpos)X
    // Fonction objective S^n_+ = x(transpos)i * xj * yi * yj
    let mut P : Vec<&[f64]> = vec![];
    // Fonction objective vector R^n = -1
    let mut q : Vec<f64> = vec![];

    // l <= Ax <= u
    // contraintes R^m * n = yT
    let mut A : Vec<&[f64]> = vec![];
    let mut vector_ay = vec![];
    let mut vector_a1 = vec![];
    // contrainte basse 0
    let mut l : Vec<f64> = vec![];
    l.push(0.0);
    l.push(0.0);
    // contrainte haute f64::INFINITY
    let mut u : Vec<f64> = vec![];
    u.push(f64::INFINITY);
    u.push(0.0);

    for i in 0..sample_size{
        let mut vector = vec![];
        for j in 0..sample_size{
            let mut sum = 0.0;
            for n in 0..dimension{
                sum += x[i * dimension + n] * x[j * dimension + n]
            }
            vector.push(y[i] * y[j] * sum);
        }
        
        let mut slice_p = vector.into_boxed_slice();
        let ptr_p = slice_p.as_mut_ptr();
        let buff_p;
        unsafe{
            buff_p = from_raw_parts(ptr_p, sample_size);
        }
        Box::leak(slice_p);
        

        P.push(buff_p);
        q.push(-1.0);
        vector_ay.push(y[i]);
        vector_a1.push(1.0);
    }

    let mut slice_a = vector_ay.into_boxed_slice();
    let mut slice_1 = vector_a1.into_boxed_slice();
    let ptr_a = slice_a.as_mut_ptr();
    let ptr_1 = slice_1.as_mut_ptr();
    let buff_a;
    let buff_1;
    unsafe{
        buff_a = from_raw_parts(ptr_a, sample_size);
        buff_1 = from_raw_parts(ptr_1, sample_size);
    }
    Box::leak(slice_a);
    Box::leak(slice_1);
    A.push(buff_1);
    A.push(buff_a);

    println!("P : {:?}",P);
    println!("q : {:?}",q);
    println!("A : {:?}",A);
    println!("l : {:?}",l);
    println!("u : {:?}",u);
    

    let P = CscMatrix::from(P).into_upper_tri();
    let settings = Settings::default()
        .alpha(1.0)
        .verbose(false);

    let mut prob = Problem::new(P, &q, A, &l, &u, &settings).expect("failed to setup problem");
    let result = prob.solve();
    let alpha = result.x().expect("failed to solve problem");
    println!("alpha : {:?}", alpha);

    let mut w : Vec<f64> = vec![];
    let mut indice_to_take : usize = 0;
    let mut found_indice = false;
    for i in 0..sample_size{
        if alpha[i] > 0.0 && !found_indice{
            indice_to_take = i;
            found_indice = true;
        }
        for j in 0..dimension{
            if i == 0{
                w.push(0.0);
            }
            w[j] += x[i * dimension + j] * y[i] * alpha[i];
        }
    }

    if found_indice {
        let mut sum = 0.0;
        for i in 0..dimension{
            sum += w[i] * x[indice_to_take * dimension + i];
        }
        w.insert(0, 1.0 / y[indice_to_take] - sum);
    }
    println!("W : {:?}", w);

    let mut slice = w.into_boxed_slice();
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

#[no_mangle]
pub extern "C" fn predict_svm_model(w_ptr: *mut f64, x_ptr: *mut f64, size: usize) -> f64 {
    let x;
    let w;
    unsafe {
        x = from_raw_parts(x_ptr , size);
        w = from_raw_parts(w_ptr, size + 1);
    }

    //if predict_svm_model_value(w, x, size) >= 0.0 {
    if predict_linear_model_regression_(w, x, size) >= 0.0{
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