extern crate rand;
extern crate nalgebra;
use rand::Rng;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
enum ActivationFunction {
    None = 0,
    Tanh = 1,
    Relu = 2,
    Sigmoid = 3
}

fn use_activation_function(value: f64, activation_function: ActivationFunction) -> f64 {
    match activation_function {
        ActivationFunction::Tanh => return value.tanh(),
        ActivationFunction::Relu => if value < 0.0 { return 0.0 } else {return value},
        ActivationFunction::Sigmoid => return 1.0 / (1.0 + (-value).exp()),
        ActivationFunction::None => return value
    }
}

fn value_to_activation_function(value: i32) -> ActivationFunction {
    match FromPrimitive::from_i32(value) {
        Some(ActivationFunction::None) => return ActivationFunction::None,
        Some(ActivationFunction::Tanh) => return ActivationFunction::Tanh,
        Some(ActivationFunction::Relu) => return ActivationFunction::Relu,
        Some(ActivationFunction::Sigmoid) => return ActivationFunction::Sigmoid,
        None            => return ActivationFunction::None,
    }
}

#[no_mangle]
pub extern "C" fn init_multicouche_model(neurones_by_couche_ptr: *mut f64, number_of_couches: usize) -> *mut f64 {
    let mut model: Vec<f64> = vec![];
    let mut rng = rand::thread_rng();
    let neurones_by_couche;
    unsafe{
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
    }
    
    for i in 0..neurones_by_couche.len() - 1{
        for _it in 0..neurones_by_couche[i + 1] as usize{
            model.push(rng.gen_range(-1.0, 1.0) as f64);
        }
        
        for _j in 0..neurones_by_couche[i] as usize{
            for _it in 0..neurones_by_couche[i + 1] as usize{
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
pub extern "C" fn predict_multicouche_model_classification(model_ptr: *mut f64, x_ptr: *mut f64, neurones_by_couche_ptr: *mut f64, number_of_couches: usize, activation_function_value: f64, output_function_value: f64) -> *const f64 {
    let model;
    let out;
    let x;
    let neurones_by_couche;

    let activation_function = value_to_activation_function(activation_function_value as i32);
    let output_function = value_to_activation_function(output_function_value as i32);

    unsafe {
        model = from_raw_parts(model_ptr, get_model_size(neurones_by_couche_ptr, number_of_couches));
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
        x = from_raw_parts(x_ptr , neurones_by_couche[0] as usize);
        out = from_raw_parts(init_out_neurone(model, &x, activation_function, output_function, neurones_by_couche), get_out_size(neurones_by_couche));
    }

    let last_couche_indice = get_correct_out_indice(neurones_by_couche, number_of_couches - 1, 1);
    let slice = &out[last_couche_indice..out.len()];
    let ptr = slice.as_ptr();
    return ptr;
}

#[no_mangle]
pub extern "C" fn predict_multicouche_model_regression(model_ptr: *mut f64, x_ptr: *mut f64, neurones_by_couche_ptr: *mut f64, number_of_couches: usize) -> *const f64 {
    let model;
    let out;
    let x;
    let neurones_by_couche;

    unsafe {
        model = from_raw_parts(model_ptr, get_model_size(neurones_by_couche_ptr, number_of_couches));
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
        x = from_raw_parts(x_ptr , neurones_by_couche[0] as usize);
        out = from_raw_parts(init_out_neurone(model, &x, ActivationFunction::Tanh, ActivationFunction::None, neurones_by_couche), get_out_size(neurones_by_couche));
    }

    let last_couche_indice = get_correct_out_indice(neurones_by_couche, number_of_couches - 1, 1);
    let slice = &out[last_couche_indice..out.len()];
    let ptr = slice.as_ptr();
    return ptr;
}

#[no_mangle]
pub extern "C" fn train_multicouche_model_classification(model_ptr: *mut f64, x_ptr: *mut f64, y_ptr: *mut f64, neurones_by_couche_ptr: *mut f64, number_of_couches: usize, number_exemples: usize, nb_iter:f64, alpha:f64,  activation_function_value: f64, output_function_value: f64) -> *mut f64{
    let mut rng = rand::thread_rng();
    let model;
    let neurones_by_couche;
    let mut out;
    let x;
    let y;

    let activation_function = value_to_activation_function(activation_function_value as i32);
    let output_function = value_to_activation_function(output_function_value as i32);

    unsafe {
        model = from_raw_parts_mut(model_ptr, get_model_size(neurones_by_couche_ptr, number_of_couches));
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
        x = from_raw_parts(x_ptr , neurones_by_couche[0] as usize * number_exemples);
        y = from_raw_parts(y_ptr, neurones_by_couche[neurones_by_couche.len() - 1] as usize * number_exemples);
    }

    for _it in 0..nb_iter as usize{
        let random = rng.gen_range(0, number_exemples);
        let mut delta : Vec<Vec<f64>> = vec![vec![]];
        unsafe {
            out = from_raw_parts(init_out_neurone(model, &x[random * neurones_by_couche[0] as usize .. random * neurones_by_couche[0] as usize + neurones_by_couche[0] as usize], activation_function, output_function, neurones_by_couche), get_out_size(neurones_by_couche));
        }
        

        for i in 1..neurones_by_couche[number_of_couches - 1] as usize + 1{
            delta[0].push(gradien_retropropagation_last_classification(y[random * neurones_by_couche[number_of_couches - 1] as usize + i - 1] as i8, out[get_correct_out_indice(neurones_by_couche, number_of_couches - 1, i)]));
        }
       
        //a partir d'ici
        for couche in (2..number_of_couches).rev(){
            let mut buff_vec : Vec<f64> = vec![];
            for neurone in 1..neurones_by_couche[couche - 1] as usize + 1{
                buff_vec.push(gradien_retropropagation(model, out, &delta[0], neurones_by_couche, couche - 1, neurone));
            }
            delta.insert(0, buff_vec);
        }
        //println!("delta : {:?}", delta);
        
        for couche in 1..number_of_couches{
            for neurone in 0..neurones_by_couche[couche - 1] as usize + 1{
                for next_neurone in 1..neurones_by_couche[couche] as usize + 1{
                    model[get_correct_model_indice(neurones_by_couche, couche - 1, neurone, next_neurone - 1)] = model[get_correct_model_indice(neurones_by_couche, couche - 1, neurone, next_neurone - 1)] - alpha * out[get_correct_out_indice(neurones_by_couche, couche - 1, neurone)] * delta[couche - 1][next_neurone - 1];
                }  
            }    
        }
    }
    let ptr = model.as_mut_ptr();
    return ptr;
}

#[no_mangle]
pub extern "C" fn train_multicouche_model_regression(model_ptr: *mut f64, x_ptr: *mut f64, y_ptr: *mut f64, neurones_by_couche_ptr: *mut f64, number_of_couches: usize, number_exemples: usize, nb_iter:f64, alpha:f64) -> *mut f64{
    let mut rng = rand::thread_rng();
    let model;
    let neurones_by_couche;
    let mut out;
    let x;
    let y;

    unsafe {
        model = from_raw_parts_mut(model_ptr, get_model_size(neurones_by_couche_ptr, number_of_couches));
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
        x = from_raw_parts(x_ptr , neurones_by_couche[0] as usize * number_exemples);
        y = from_raw_parts(y_ptr, neurones_by_couche[neurones_by_couche.len() - 1] as usize * number_exemples);
    }
    
    for _it in 0..nb_iter as usize{
        let random = rng.gen_range(0, number_exemples);
        let mut delta : Vec<Vec<f64>> = vec![vec![]];
        unsafe {
            out = from_raw_parts(init_out_neurone(model, &x[random * neurones_by_couche[0]  as usize .. random * neurones_by_couche[0] as usize + neurones_by_couche[0] as usize], ActivationFunction::Tanh, ActivationFunction::None, neurones_by_couche), get_out_size(neurones_by_couche));
        }
        //println!("out : {:?}", out);

        for i in 1..neurones_by_couche[number_of_couches - 1] as usize + 1{
            delta[0].push(gradien_retropropagation_last_regression(y[random * neurones_by_couche[number_of_couches - 1] as usize + i - 1], out[get_correct_out_indice(neurones_by_couche, number_of_couches - 1, i)]));
        }
       
        //a partir d'ici
        for couche in (2..number_of_couches).rev(){
            let mut buff_vec : Vec<f64> = vec![];
            for neurone in 1..neurones_by_couche[couche - 1] as usize + 1{
                buff_vec.push(gradien_retropropagation(model, out, &delta[0], neurones_by_couche, couche - 1, neurone));
            }
            delta.insert(0, buff_vec);
        }
        //println!("delta : {:?}", delta);
        
        for couche in 1..number_of_couches{
            for neurone in 0..neurones_by_couche[couche - 1] as usize + 1{
                for next_neurone in 1..neurones_by_couche[couche] as usize + 1{
                    model[get_correct_model_indice(neurones_by_couche, couche - 1, neurone, next_neurone - 1)] = model[get_correct_model_indice(neurones_by_couche, couche - 1, neurone, next_neurone - 1)] - alpha * out[get_correct_out_indice(neurones_by_couche, couche - 1, neurone)] * delta[couche - 1][next_neurone - 1];
                }  
            }    
        }
    }
    let ptr = model.as_mut_ptr();
    return ptr;
}

#[no_mangle]
pub extern "C" fn get_model_size(neurones_by_couche_ptr: *mut f64, number_of_couches: usize) -> usize {
    let neurones_by_couche;
    unsafe{
        neurones_by_couche = from_raw_parts(neurones_by_couche_ptr, number_of_couches);
    }
    let mut size = 0;
    for i in 0..number_of_couches - 1{
        size += (neurones_by_couche[i] as usize + 1) * neurones_by_couche[i + 1] as usize;
    }
    return size;
}

#[no_mangle]
pub extern "C" fn get_out_size(neurones_by_couche: &[f64]) -> usize {
    let mut size = 0;
    for i in 0..neurones_by_couche.len(){
        size += neurones_by_couche[i] as usize + 1;
    }
    return size;
}

//x couches indices : 0, 3, 7, 10
fn get_correct_out_indice(neurones_by_couche: &[f64], couche: usize, neurone: usize) -> usize{
    let mut indice = 0;
    assert!(couche <= neurones_by_couche.len() - 1, "couche not exist or not reachable");
    assert!(neurone <= neurones_by_couche[couche] as usize, "neurone not exist");

    for i in 0..couche {
        indice += neurones_by_couche[i] as usize + 1;
    }
    indice += neurone; 
    return indice;
}

//w  = [2,3,2,1] -> couches indices : 0, 9, 17 / neurones indices : 0 -> 0, 3, 6 / 1 -> 9, 11, 13, 15 / 2 -> 17,18,19
fn get_correct_model_indice(neurones_by_couche: &[f64], couche: usize, neurone: usize, next_neurone: usize) -> usize{
    let mut indice = 0;
    assert!(couche <= neurones_by_couche.len() - 2, "couche not exist or not reachable");
    assert!(neurone <= neurones_by_couche[couche] as usize, "neurone not exist");
    assert!(next_neurone <= neurones_by_couche[couche + 1] as usize - 1, "next neurone not exist");
    for i in 0..couche {
        indice += (neurones_by_couche[i] as usize + 1) * neurones_by_couche[i + 1] as usize;
    }
    indice += neurones_by_couche[couche + 1] as usize * neurone + next_neurone;
    return indice;
}

fn init_out_neurone(model: &[f64], x: &[f64], activation_function: ActivationFunction, output_function: ActivationFunction, neurones_by_couche: &[f64]) -> *mut f64{
    let mut x_vec : Vec<f64> = vec![];
    x_vec.push(1.0);
    for i in 0..x.len(){
        x_vec.push(x[i]);
    }

    for couche in 1..neurones_by_couche.len(){
        x_vec.push(1.0);
        for next_neurone in 1..neurones_by_couche[couche] as usize + 1{
            if couche != neurones_by_couche.len() - 1{
                x_vec.push(calculate_signal(model, &x_vec, activation_function, next_neurone - 1, neurones_by_couche, couche - 1));
            }
            else {
                x_vec.push(calculate_signal(model, &x_vec, output_function, next_neurone - 1, neurones_by_couche, couche - 1));
            }
        }
    }
    let mut slice = x_vec.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    Box::leak(slice);
    return ptr;
}

fn calculate_signal(model: &[f64], x: &[f64], activation_function: ActivationFunction, next_neurone:usize, neurones_by_couche: &[f64], couche: usize) -> f64 {
    let mut value = 0.0;
    for neurone in 0..neurones_by_couche[couche] as usize + 1{
        value += model[get_correct_model_indice(neurones_by_couche, couche, neurone, next_neurone)] * x[get_correct_out_indice(neurones_by_couche, couche, neurone)];
    }
    return use_activation_function(value, activation_function);
}

fn gradien_retropropagation (w : &[f64], x: &[f64], sigma: &[f64], neurones_by_couche: &[f64], couche: usize, neurone: usize) -> f64 {
    let mut sum = 0.0;
    for next_neurone in 1..neurones_by_couche[couche + 1] as usize + 1{
        sum += w[get_correct_model_indice(neurones_by_couche, couche, neurone, next_neurone - 1)] * sigma[next_neurone - 1];
    }
    return (1.0 - x[get_correct_out_indice(neurones_by_couche, couche, neurone)].powf(2.0) ) * sum;
}

fn gradien_retropropagation_last_classification (y: i8, xlj: f64 ) -> f64 {
    let result :f64 = (1.0 - xlj.powf(2.0)) * (xlj - y as f64);
    return result;
}

fn gradien_retropropagation_last_regression (y: f64, xlj: f64 ) -> f64 {
    return (xlj - y) as f64;
}