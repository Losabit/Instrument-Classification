use std::slice::{from_raw_parts};
use osqp::{CscMatrix, Problem, Settings};
use rand::Rng;
use nalgebra::{DMatrix};

// SVM
// https://docs.rs/osqp/0.6.0/osqp/
// type_solver : 0 -> régréssion/ 1 -> classification 
#[no_mangle]
pub extern "C" fn train_svm_model_rbf_kernel(x_ptr: *mut f64, y_ptr: *mut f64, dimension: usize, sample_size: usize, gamma: f64, type_solver: usize) -> *mut f64{
    let x;
    let y;
    unsafe {
        x = from_raw_parts(x_ptr , dimension * sample_size);
        y = from_raw_parts(y_ptr, sample_size);
    }
    /*
    println!("x : {:?}", x);
    println!("y : {:?}", y);
    println!("dimensions : {:?}", dimension);
    println!("samples : {:?}", sample_size);
    println!("gamma : {:?}", gamma);
    println!("type_solver : {:?}", type_solver);
    */
    
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
    // contrainte basse 
    let mut l : Vec<f64> = vec![];
    l.push(0.0);
    l.push(0.0);
    // contrainte haute
    let mut u : Vec<f64> = vec![];
    u.push(f64::INFINITY);
    u.push(0.0);

    let model = rbf_kernel_build(&x, &y, sample_size, dimension, gamma);
    for i in 0..sample_size{
        let mut vector = vec![];
        for j in 0..sample_size{
           
            let x1 = &x[i * dimension .. (i + 1) * dimension];
            let x2 = &x[j * dimension .. (j + 1) * dimension];
            if type_solver == 0 {
                vector.push(y[i] * y[j] * rbf_kernel_compute(&model, x1) * rbf_kernel_compute(&model, x2));
               // vector.push(y[i] * y[j] * rbf_kernel_compute2(&model, x1, x2));
            }
            else if type_solver == 1 {
                vector.push(y[i] * y[j] * rbf_kernel_compute(&model, x1).tanh() * rbf_kernel_compute(&model, x2).tanh());
               // vector.push(y[i] * y[j] * rbf_kernel_compute2(&model, x1, x2).tanh());
            }
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
    /*
    println!("P : {:?}",P);
    println!("q : {:?}",q);
    println!("A : {:?}",A);
    println!("l : {:?}",l);
    println!("u : {:?}",u);
    */

    let P = CscMatrix::from(P).into_upper_tri();
    let settings = Settings::default()
        .alpha(1.0)
        .verbose(false);

    let mut prob = Problem::new(P, &q, A, &l, &u, &settings).expect("failed to setup problem");
    let result = prob.solve();
    let alpha = result.x().expect("failed to solve problem");
    //println!("alpha : {:?}", alpha);

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
    //println!("W : {:?}", w);

    let mut slice = w.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr  
}

#[no_mangle]
pub extern "C" fn train_svm_model_basic_kernel(x_ptr: *mut f64, y_ptr: *mut f64, dimension: usize, sample_size: usize) -> *mut f64{
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
    // contrainte basse 
    let mut l : Vec<f64> = vec![];
    l.push(0.0);
    l.push(0.0);
    // contrainte haute
    let mut u : Vec<f64> = vec![];
    u.push(f64::INFINITY);
    u.push(0.0);

    for i in 0..sample_size{
        let mut vector = vec![];
        for j in 0..sample_size{
            let x1 = &x[i * dimension .. (i + 1) * dimension];
            let x2 = &x[j * dimension .. (j + 1) * dimension];
            vector.push(y[i] * y[j] * basic_kernel_compute(x1, x2));
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
    /*
    println!("P : {:?}",P);
    println!("q : {:?}",q);
    println!("A : {:?}",A);
    println!("l : {:?}",l);
    println!("u : {:?}",u);
    */

    let P = CscMatrix::from(P).into_upper_tri();
    let settings = Settings::default()
        .alpha(1.0)
        .verbose(false);

    let mut prob = Problem::new(P, &q, A, &l, &u, &settings).expect("failed to setup problem");
    let result = prob.solve();
    let alpha = result.x().expect("failed to solve problem");
    //println!("alpha : {:?}", alpha);

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
    //println!("W : {:?}", w);

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

    if predict_linear_model_regression_(w, x, size) >= 0.0{
        return 1.0;
    }
    else {
        return -1.0;
    }
}

// KERNELS
// Polynomial
fn polynomial_kernel_compute(x1: &[f64], x2: &[f64], degree: i32) -> f64 {
    assert_eq!(x1.len(), x2.len());
    let sum = basic_kernel_compute(x1, x2);
    return (1.0 + sum).powi(degree);
}

// Basique
fn basic_kernel_compute(x1: &[f64], x2: &[f64]) -> f64 {
    assert_eq!(x1.len(), x2.len());
    let mut sum = 0.0;
    for i in 0..x1.len(){
        sum += x1[i] * x2[i];
    }
    return sum;
}

// RBF
pub struct RBF { pub(crate) w: DMatrix<f64>, pub(crate) x: DMatrix<f64>, pub(crate) gamma: f64 }

pub fn randvalue(size: usize, it : usize) -> f64 {
    let mut rng = rand::thread_rng();
    let myrand = (rng.gen_range(0,it) % size as f64 as usize) as f64;
    return myrand;
}

fn init_rbf(w : DMatrix<f64> , x :DMatrix<f64>, gamma : f64) -> RBF{
    let r = RBF{
        w,
        x,
        gamma
    };
    return r;
}

pub fn rbf_kernel_compute(model: &RBF, x: &[f64]) -> f64{
    let mut for_sum : Vec<f64> = vec![];
    for i in 0..model.w.len(){
        let mut vector_x = Vec::new();
        for k in 0..x.len(){
            vector_x.push(x[k] - model.x.row(i)[k])
        }
        let alpha = DMatrix::from_row_slice(x.len(), 1, &vector_x);
        let toexp = -model.gamma * (alpha.norm() * alpha.norm());
        let result = model.w.row(i) * toexp.exp();
        for_sum.push(result.row(0)[0]);
    }
    let mut sum = 0.0;
    for i in 0..for_sum.len(){
        sum += for_sum[i];
    }
    return sum;
}

pub fn rbf_kernel_compute2(model: &RBF, x1: &[f64], x2: &[f64]) -> f64{
    assert_eq!(x1.len(), x2.len());
    let mut for_sum : Vec<f64> = vec![];
    for i in 0..model.w.len(){
        let mut vector_x = Vec::new();
        for k in 0..x1.len(){
            vector_x.push(x1[k] - x2[k])
        }
        let alpha = DMatrix::from_row_slice(x1.len(), 1, &vector_x);
        let toexp = -model.gamma * (alpha.norm() * alpha.norm());
        let result = model.w.row(i) * toexp.exp();
        for_sum.push(result.row(0)[0]);
    }
    let mut sum = 0.0;
    for i in 0..for_sum.len(){
        sum += for_sum[i];
    }
    return sum;
}

pub fn rbf_kernel_build(x: &[f64], y: &[f64], input_per_sample: usize, nb_sample: usize, gamma: f64 ) -> RBF{
    let xm =  DMatrix::from_row_slice(input_per_sample , nb_sample ,x);
    let ym = DMatrix::from_row_slice(input_per_sample , 1 as usize, y);
    let mut phi = Vec::new();
    
    for i in 0.. input_per_sample  {
        for j in  0.. input_per_sample  {
            let mut sum = Vec::new();
            for k in 0..nb_sample{
                sum.push(xm.row(i)[k] - xm.row(j)[k])
            }
            let alpha = DMatrix::from_row_slice(nb_sample,1,&sum);
            let toexp = -gamma * (alpha.norm() * alpha.norm());
            phi.push( toexp.exp());
        }
    }
    let phi_mat = DMatrix::from_row_slice(input_per_sample,input_per_sample,&phi);
    /*
    println!("{:?}", phi_mat);
    println!("{:?}", phi_mat.determinant());
    */
    let wm = phi_mat.try_inverse().unwrap() * ym;
    let rbf =  init_rbf(wm, xm, gamma);
    return rbf;
}

