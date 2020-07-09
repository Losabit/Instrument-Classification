extern crate rand;
use rand::Rng;
use std::slice::{from_raw_parts};

mod mlp;
mod linear;
mod svm;

fn main(){
//linear
    //classification
    /*
    let good_y;
    let mut y : Vec<f64> = vec![
        0.0,0.0,0.0,
        0.0,0.0,1.0,
        0.0,1.0,0.0,
        1.0,0.0,0.0,
        0.0,0.0,1.0,
        0.0,1.0,0.0,
        1.0,0.0,0.0
        ];

    unsafe {
        good_y = from_raw_parts(linear::output2d_to_1d(y.as_mut_ptr(),7, 3), 7);
    }    
    println!("{:?}", good_y);

    let mut w = linear::init_linear_model(3, 0.0, 1.0);
    let x = vec![vec![0.2, 0.4], vec![0.73, 0.94]];
    let y = vec![-1,1];
    
    println!("{:?}",w);
    linear::train_linear_model_classification(&mut w, &x, &y, 15, 0.1);
    println!("{:?}",w);
*/
    //regression
/*
    let mut x : Vec<f64> = vec![];
    let mut y : Vec<f64> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..20{
        x.push(1.0);
        x.push((i + 1) as f64);
        y.push(1.5 * ((i + 1) as f64) + 0.75 + (( - 0.35) * 2.0));
    }
*/
/*
    let mut x = vec![1.0,1.0,2.0,2.0, 3.0, 1.0];
    let mut y = vec![2.0,3.0,2.5];

    let model;
    let model_ptr = linear::train_linear_model_regression(x.as_mut_ptr(),y.as_mut_ptr(),x.len());

    unsafe{
        model = from_raw_parts(model_ptr, 3);
    }
    println!("{:?}",&model);
    println!("{:?}", linear::predict_linear_model_regression(model_ptr, x.as_mut_ptr(), 2));

*/
//multicouche 
         
    // let mut neurone_by_couche = [1.0,6.0,1.0];
    // let mut rng = rand::thread_rng();
    // let size = mlp::get_model_size(neurone_by_couche.as_mut_ptr(), neurone_by_couche.len());
    // let mut model_ptr = mlp::init_multicouche_model(neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
    // let mut model;
    // unsafe{
    //     model = from_raw_parts(model_ptr, size);



    //classification  
    /* 
    let mut x = vec![];
    for _ in 0..1000{
        x.push(rng.gen_range(0.0,1.0) * 2.0 - 1.0)
    }
    let mut y = vec![];
    for i in 0..500{
        if -x[i * 2] - x[i * 2 + 1] - 0.5 > 0.0 && x[i * 2 + 1] < 0.0 && x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0{
            y.push(1.0);
            y.push(0.0);
            y.push(0.0);
        }
        else if -x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0 && x[i * 2 + 1] > 0.0 && x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0{
            y.push(0.0);
            y.push(1.0);
            y.push(0.0);
        }
        else if -x[i * 2] - x[i * 2 + 1] - 0.5 < 0.0 && x[i * 2 + 1] < 0.0 && x[i * 2] - x[i * 2 + 1] - 0.5 > 0.0{
            y.push(0.0);
            y.push(0.0);
            y.push(1.0);
        }
        else{
            y.push(0.0);
            y.push(0.0);
            y.push(0.0);
        }
    }
    
    model_ptr = mlp::train_multicouche_model_classification(model_ptr, x.as_mut_ptr(), y.as_mut_ptr(), neurone_by_couche.as_mut_ptr(), neurone_by_couche.len(), y.len() / 3, 10000.0, 0.1);
    unsafe{
        model = from_raw_parts(model_ptr, size);
    }
    println!("{:?}",model);

    let mut predict_value;
    for i in 0..20 {
        let predict_value_ptr = mlp::predict_multicouche_model_classification(model_ptr,  x[i * 2..(i + 1) * 2].as_mut_ptr(), neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
        unsafe{
            predict_value = from_raw_parts(predict_value_ptr, 3); 
        }
        println!("predict value {:?} = {:?} for {:?} / {:?}", i + 1, predict_value, &x[i * 2..(i + 1) * 2], &y[i * 3..(i + 1) * 3]);
    }
    */

    //regression
    // let mut x = vec![1.0,2.0,3.0];
    // let mut y = vec![2.0,3.0,2.5];
    //
    // model_ptr = mlp::train_multicouche_model_regression(model_ptr, x.as_mut_ptr(), y.as_mut_ptr(), neurone_by_couche.as_mut_ptr(), neurone_by_couche.len(), y.len(), 60000.0, 0.1);
    // unsafe{
    //     model = from_raw_parts(model_ptr, size);
    // }
    // println!("{:?}",model);
    //
    // let mut predict_value;
    // for i in 0..3 {
    //     let predict_value_ptr = mlp::predict_multicouche_model_regression(model_ptr,  [x[i]].as_mut_ptr(), neurone_by_couche.as_mut_ptr(),  neurone_by_couche.len());
    //     unsafe{
    //         predict_value = from_raw_parts(predict_value_ptr,1);
    //     }
    //     println!("predict value {:?} = {:?} for {:?} / {:?}", i + 1, predict_value, &x[i], &y[i]);
    // }
    

    
    //SVM
    let mut x = vec![1.5707647905927653, 1.393501598372334, 1.7989497505683756, 1.3713961477756846, 1.2526871036639957, 1.060847158667422, 
    1.4550641085434537, 1.660334235274881, 1.14605929530493, 1.5976854930473243, 1.2136453235660374, 1.7675906834380721, 1.1763313142548253, 
    1.6019203364534886, 1.1322723384172007, 1.7947682570755479, 1.3187237242188705, 1.8081678682829194, 1.2097109699718362, 1.2450741566229135, 
    1.4423932265849095, 1.838552444965723, 1.0143961401859762, 1.1738420950683064, 1.8866120252305167, 1.5762953803210216, 1.2181841203542012, 
    1.2145236907101171, 1.2332135981108059, 1.1004975162216633, 1.7300274891565879, 1.7775251889504018, 1.5041192577919178, 1.1454688090216643, 
    1.2676474891296423, 1.5067682906219209, 1.0165903268748788, 1.6916634075638626, 1.1283312657732107, 1.509317661903001, 1.5335650976423798, 
    1.5112436395702535, 1.4112357980784302, 1.8478787003935149, 1.415767737737723, 1.0136855399099292, 1.5800689056021848, 1.0943782030427287, 
    1.3048615858649586, 1.8564449275336943, 1.5672215745062772, 1.7041256421269362, 1.560207103494398, 1.0333021024219318, 1.6947544353572186, 
    1.4269277799047788, 1.8957216536994403, 1.5598145417375047, 1.6420984990776017, 1.7785914488028802, 1.466441495200745, 1.3954511964747938, 
    1.4474897715454331, 1.8994444798312877, 1.5861191112318507, 1.3194846871187689, 1.7479351225508415, 1.8824662932329246, 1.7352986725477026, 
    1.284837598993809, 1.659612740538448, 1.5114328489293078, 1.2618884972633573, 1.376632622671131, 1.6394050135220581, 1.1513591325742913, 
    1.8000302589517332, 1.44827497401532, 1.8235180205566985, 1.5440656663821613, 1.351726158008479, 1.8031829147636385, 1.5885787602698072, 
    1.3930550199561509, 1.276236704705255, 1.8073838169245287, 1.6074600368839183, 1.2977630243094174, 1.0126264152849909, 1.685677137556298, 
    1.8639260528118595, 1.4272762814955, 1.8230108726200793, 1.131431293847853, 1.4111178771147044, 1.6575762421475357, 1.0370372286962493, 
    1.8194279272230272, 1.8381054817098608, 1.4930143494685673, 2.2266759526769633, 2.5557020342395536, 2.807672639775521, 2.2119685500886135, 
    2.630661658277581, 2.4204395434586656, 2.1660033274108224, 2.5961476897358775, 2.4776129405968024, 2.480663522490471, 2.3624801767561827, 
    2.5368433545423774, 2.0712628265304907, 2.091904648238852, 2.6164483905861777, 2.0558340732728837, 2.5918215857782902, 2.355558284239507, 
    2.5705810923502064, 2.6169349950050873, 2.234145240993642, 2.133102437969705, 2.603862224044155, 2.194788767880338, 2.8195722985752787, 
    2.2529098267958, 2.309118131928005, 2.583836459855793, 2.8058898023885654, 2.392620206877952, 2.1795143506309893, 2.7969489579298257, 
    2.1246584193361255, 2.162143547677379, 2.3391150472332622, 2.106774698366268, 2.4373089988990584, 2.483686659721285, 2.6113171437575513, 
    2.623653078605731, 2.4113709463232684, 2.582664004342371, 2.590673495558188, 2.6192986920445396, 2.7228623515310195, 2.6022855779850715, 
    2.6894886783542287, 2.3388498813749887, 2.35419532511145, 2.1018371752387686, 2.073966541376879, 2.456475447873114, 2.7746408574696844, 
    2.7282972939840215, 2.713119278121925, 2.672849229656263, 2.062042762434001, 2.277994269764033, 2.1229248008791775, 2.871992431553946, 
    2.2063597996074575, 2.880104181562682, 2.481261985200711, 2.0771166159400085, 2.7901330420743387, 2.098597381871213, 2.7291912397678635, 2.3585259049503735, 2.410604292168028, 2.8589693963217218, 2.359671874256364, 2.198835984269053, 2.55152007101578, 2.8964081889013333, 2.2174913868471173, 2.019255230647742, 2.155823882280005, 2.6270029905614933, 2.238589044579118, 2.1616509294574775, 2.0820755751638917, 2.5165971955487505, 2.391800994335936, 2.3896710449636087, 2.4584065122262846, 2.5483676563038964, 2.245474309138567, 2.7812389251740806, 2.7236113460475684, 2.552600460170816, 2.3759850478263296, 2.3121650928584527, 2.603031437513735, 2.013757633024543, 2.3569255967935394, 2.576837846374191, 2.4575023584583278, 2.4144310100955915, 2.8932499250483144, 2.1000235703991934];
    let mut y = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]; 
    let model_ptr = svm::train_svm_model_rbf_kernel(x.as_mut_ptr(), y.as_mut_ptr(), 2, 100, 10.0, 1);
    let model;
    unsafe{
        model = from_raw_parts(model_ptr, 3);
    }
    println!("model : {:?}",model);

    for i in 0..60{
        println!("predict : {:?} / result : {:?}", svm::predict_svm_model(model_ptr, x[i * 2..(i + 1) * 2].as_mut_ptr(), 3), y[i]);
    }
    
}