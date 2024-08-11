mod algorithm;

use algorithm::say_xvectors;


fn main() {
    println!("Hello, world!");

    let x_vecs: Vec<String> = say_xvectors(5);

    let x: Vec<char> = vec!['1', '1', '0', '1', '0'];

    let xb: Vec<u8> = x.iter().map(|x| (*x as u8 - 0)).collect::<Vec<u8>>();

    // println!("{:?}", xb);
    // println!("{:?}", String::from_utf8(xb).unwrap());
    // println!("{:?}", x.join(""));

    let xb_str: String = String::from_utf8(xb).unwrap();

    for xi in 0..x_vecs.len() {
        let xi_str: &String = &x_vecs[xi];
        
        if *xb_str == *xi_str {
            println!("p{xi}: {}, {}", xi_str, xb_str)
        }
    }
}
