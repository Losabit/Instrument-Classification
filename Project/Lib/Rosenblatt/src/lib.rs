extern crate rand;
extern crate nalgebra;
use rand::Rng;
use nalgebra::DMatrix;

/*
#[no_mangle]
pub extern fn init_linear_model(size: usize, start: f32, end: f32) -> Vec<f32>{

    let mut vector: Vec<f32> = vec![];
    unsafe{
    let mut rng = rand::thread_rng();

        vector.push(1.0);

        for _it in 0..size {
            vector.push(rng.gen_range(start, end));
        }
    }
    return vector;
}
*/

#[no_mangle]
pub extern fn init_linear_model_tab(size: usize, start: f32, end: f32) -> *mut f64 {
    let mut vector: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    vector.push(1.0);

    for _it in 0..size {
        vector.push(rng.gen_range(start, end));
    }
    let mut slice = vector.into_boxed_slice();
    return slice.as_mut_ptr();
}

#[no_mangle]
pub extern fn predict_linear_model_regression(w:&Vec<f32>, xk:&Vec<f32>)-> f32{
    let mut sum = w[0];
    for i in 0..xk.len(){
        sum += w[i + 1] * xk[i];
    }
    return sum;
}
#[no_mangle]
pub extern fn predict_linear_model_regression_tab(w:&[f32], xk:&[f32])-> f32{
    let mut sum = w[0];
    for i in 0..xk.len(){
        sum += w[i + 1] * xk[i];
    }
    return sum;
}

#[no_mangle]
pub extern fn predict_linear_model_classification(w:&Vec<f32>, xk:&Vec<f32>)-> i8{
    return if predict_linear_model_regression(w,xk) >= 0.0 { 1 } else { -1 }
}
#[no_mangle]
pub extern fn predict_linear_model_classification_tab(w:&[f32], xk:&[f32] )-> i8{
    return if predict_linear_model_regression_tab(w,xk) >= 0.0 { 1 } else { -1 }
}

//Vec à une dimension, ajouter alors la taille pour chaque dimension et chaque dimension
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
pub extern fn train_linear_model_classification_tab(w:&mut [f32], x:&[[f32]], y:&[i8], nb_iter:i32, alpha:f32) {
    for _it in 0..nb_iter {
        let mut rng = rand::thread_rng();
        let k = rng.gen_range(0, x[0].len());
        let gxk = predict_linear_model_classification_tab(w,&x[k]);
        for i in 0..x[1].len() {
            w[i + 1] += alpha * (y[k] - gxk as i8) as f32 * x[k][i];
        }
        w[0] += alpha * (y[k] - gxk as i8) as f32;
    }
}

#[no_mangle]
pub extern fn train_linear_model_regression(x: Vec<f32>, y: Vec<f32>) -> Vec<f32>{
    assert_eq!(x.len()/2, y.len());
    let xm = DMatrix::from_row_slice(y.len(),2,&x);
    let ym = DMatrix::from_row_slice(y.len(),1,&y);
    let w_matrix = (((xm.transpose() * &xm).try_inverse()).unwrap() * xm.transpose()) * ym;
    return w_matrix.data.as_vec().to_vec();
}


/**
* Function to labal random y for X0
* Fonction qui permet d'étiqueté au hasard
**/
#[warn(dead_code)]
fn init_random_y_xo(x:Vec<f32>) -> Vec<f32> {
    let mut vector_y: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    vector_y.push(1.0);
    for _it in 0.. x.len(){
        vector_y.push(rng.gen_range(0.0, 1.0));
    }
    return vector_y;
}

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