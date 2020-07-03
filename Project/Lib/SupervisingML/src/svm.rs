// SVM
#[no_mangle]
pub extern "C" fn init_svm_model(neurones_by_couche: &[usize]) -> usize {
    let mut size = 0;
    for i in 0..neurones_by_couche.len(){
        size += neurones_by_couche[i] + 1;
    }
    return size;
}