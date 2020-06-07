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
    return w_matrix.data.as_vec().to_vec().into_boxed_slice().as_mut_ptr();
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
        dataset_outputs = from_raw_parts(y, sample_size)
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

//Multi-couche
#[no_mangle]
pub extern "C" fn init_multicouche_model(neurones_by_couche_ptr: *const usize, number_of_couches: usize) -> *mut f64 {
    let mut model: Vec<f64> = vec![];
    let mut rng = rand::thread_rng();
    let neurones_by_couche;
    unsafe{
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
    }
    
    for i in 0..neurones_by_couche.len() - 1{
        
        for _it in 0..neurones_by_couche[i + 1]{
            model.push(1.0)
        }
        
        for _j in 0..neurones_by_couche[i]{
            for _it in 0..neurones_by_couche[i + 1]{
                model.push(rng.gen_range(-1.0, 1.0) as f64);
            }
        }
    }
    let mut slice = model.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr;
}

#[no_mangle]
pub extern "C" fn get_model_size(neurones_by_couche_ptr: *const usize, number_of_couches: usize) -> usize {
    let neurones_by_couche;
    unsafe{
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
    }
    let mut size = 0;
    for i in 0..number_of_couches - 1{
        size += (neurones_by_couche[i] + 1) * neurones_by_couche[i + 1];
    }
    return size;
}

#[no_mangle]
pub extern "C" fn get_out_size(neurones_by_couche: &[usize]) -> usize {
    let mut size = 0;
    for i in 0..neurones_by_couche.len(){
        size += neurones_by_couche[i] + 1;
    }
    return size;
}

#[no_mangle]
pub extern "C" fn train_multicouche_model_classification(model_ptr: *mut f64, x_ptr: *mut f64, y_ptr: *mut i8, neurones_by_couche_ptr: *const usize, number_of_couches: usize, nb_iter:usize, alpha:f64 ){
    let model;
    let neurones_by_couche;
    let mut x;
    let y;
    unsafe {
        model = from_raw_parts_mut(model_ptr, get_model_size(neurones_by_couche_ptr, number_of_couches));
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
        x = from_raw_parts_mut(x_ptr , neurones_by_couche[0] + 1);
        x = from_raw_parts_mut(init_out_neurone(model, &x, neurones_by_couche), get_out_size(neurones_by_couche));
        y = from_raw_parts(y_ptr, neurones_by_couche[neurones_by_couche.len() - 1]);
    }

    //ajouter toutes les couches de sigma
    println!("Sigma Print : ");
    let mut sigma : Vec<Vec<f64>> = vec![vec![]];
    for i in 0..y.len(){
        sigma[0].push(gradien_retropropagation_last_classification(y[i], x[get_correct_out_indice(neurones_by_couche, number_of_couches - 1, i + 1)]));
    }
    println!("{:?}",sigma);

    for couche in (0..number_of_couches - 1).rev(){
        let mut buff_vec : Vec<f64> = vec![];
        for neurone in 0..neurones_by_couche[couche]{
            println!("couche = {:?} / neurone = {:?}", couche, neurone);
            buff_vec.push(gradien_retropropagation(model, x, sigma[0][neurone], neurones_by_couche, couche, neurone));
        }
        sigma.insert(0, buff_vec);
        println!("{:?}",sigma);
    }
    

    for _it in 0..nb_iter{
        for couche in 0..number_of_couches{
            for neurone in 0..neurones_by_couche[couche] + 1{
                let sub_value = alpha * x[get_correct_out_indice(neurones_by_couche, couche, neurone)] * sigma[couche][neurone];
                for next_neurone in 0..neurones_by_couche[couche + 1] + 1{
                    model[get_correct_model_indice(neurones_by_couche, couche, neurone, next_neurone)] = model[get_correct_model_indice(neurones_by_couche, couche, neurone, next_neurone)] - sub_value;
                }  
            }    
        }

        // est ce que ca doit bien aller ici tout ces refresh ??
        refresh_out_neurone(model, &mut x, neurones_by_couche);
        /*for i in 0..sigma[last_couche].len() {
            sigma[last_couche][i] = gradien_retropropagation_last_classification(y[i], x[get_correct_out_indice(neurones_by_couche, number_of_couches - 1, i)]);
        }
        for couche in (0..last_couche).rev() {
            for neurone in 0..sigma[couche].len() {
                //sigma[couche][neurone] = gradien_retropropagation()
            }
        }
        */
    }
}

//x couches indices : 0, 3, 7, 10
fn get_correct_out_indice(neurones_by_couche: &[usize], couche: usize, neurone: usize) -> usize{
    let mut indice = 0;
    assert!(couche <= neurones_by_couche.len() - 1, "couche not exist or not reachable");
    assert!(neurone <= neurones_by_couche[couche], "neurone not exist");

    for i in 0..couche {
        indice += neurones_by_couche[i] + 1;
    }
    indice += neurone; 
    return indice;
}

//w  = [2,3,2,1] -> couches indices : 0, 9, 17 / neurones indices : 0 -> 0, 3, 6 / 1 -> 9, 11, 13, 15 / 2 -> 17,18,19
fn get_correct_model_indice(neurones_by_couche: &[usize], couche: usize, neurone: usize, next_neurone: usize) -> usize{
    let mut indice = 0;
    assert!(couche <= neurones_by_couche.len() - 2, "couche not exist or not reachable");
    assert!(neurone <= neurones_by_couche[couche], "neurone not exist");
    assert!(next_neurone <= neurones_by_couche[couche + 1] - 1, "next neurone not exist");

    for i in 0..couche {
        indice += (neurones_by_couche[i] + 1) * neurones_by_couche[i + 1];
    }
    indice += neurones_by_couche[couche + 1] * neurone + next_neurone; 
    return indice;
}

pub fn init_out_neurone(model: &[f64], x: &[f64], neurones_by_couche: &[usize]) -> *mut f64{
    let mut x_vec : Vec<f64> = x.to_vec();
    for couche in 0..neurones_by_couche.len() - 1{
        x_vec.push(1.0);
        for neurone in 0..neurones_by_couche[couche + 1]{
            x_vec.push(calculate_signal(model, &x_vec, neurone, neurones_by_couche, couche));
        }
    }
    let mut slice = x_vec.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr;
}

fn refresh_out_neurone(model: &[f64], x: &mut [f64], neurones_by_couche: &[usize]){
    for couche in 1..neurones_by_couche.len(){
        for neurone in 1..neurones_by_couche[couche]{
            x[get_correct_out_indice(neurones_by_couche, couche, neurone)] = calculate_signal(model, x, neurone - 1, neurones_by_couche, couche);
        }
    }
}

fn calculate_signal(model: &[f64], x: &[f64], next_neurone:usize, neurones_by_couche: &[usize], couche: usize) -> f64 {
    let mut value = 0.0;
    for neurone in 0..neurones_by_couche[couche]{
        value += model[get_correct_model_indice(neurones_by_couche, couche, neurone, next_neurone)] * x[get_correct_out_indice(neurones_by_couche, couche, neurone)];
    }
    return value.tanh();
}

fn gradien_retropropagation (w : &[f64], x: &[f64], sigma:f64, neurones_by_couche: &[usize], couche: usize, neurone: usize) -> f64 {
    let mut sum = 0.0;
    for i in 1..neurones_by_couche[couche]{
        sum += w[get_correct_model_indice(neurones_by_couche, couche, neurone, i)] * sigma;
    }
    let sig = (1.0 - x[get_correct_out_indice(neurones_by_couche, couche, neurone)].powf(2.0) ) * sum;
    return sig;
}

fn gradien_retropropagation_last_classification (y: i8, xlj: f64 ) -> f64 {
    let result :f64 = (1.0 - xlj.powf(2.0)) * (xlj - y as f64);
    return result;
}

fn gradien_retropropagation_last_regression (y: f64, xlj: f64 ) -> f64 {
    return (xlj - y) as f64;
}