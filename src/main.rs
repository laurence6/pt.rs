#[allow(non_snake_case)]

extern crate pt;
use pt::vector;

//use std::time::SystemTime;
use std::mem::size_of_val;

fn main() {
    //let v1 = vector::V(2.0, 3.0, 5.0);
    //let v2 = vector::V(6.0, 4.0, 2.0);
    let v1 = vector::RandomUnitVector();
    let v2 = vector::RandomUnitVector();
    println!("v1: {} {}\nv2: {} {}", v1, size_of_val(&v1), v2, size_of_val(&v2));

    println!("Add: \t {}", v1 + 2.0);
    println!("Add: \t {}", v1 + v2);

    println!("Length: \t {}", v1.Length());
    println!("Length2: \t {}", v1.LengthN(2.0));
    println!("Length3: \t {}", v1.LengthN(3.0));
    println!("Normalize: \t {}", v1.Normalize());
    println!("Neg: \t {}", -v1);
    println!("Abs: \t {}", (-v1).Abs());
    println!("Dot: \t {}", v1.Dot(&v2));
    println!("Cross: \t {}", v1.Cross(&v2));
    println!("Add: \t {}", v1 + v2);
    println!("Sub: \t {}", v1 - v2);
    println!("Mul: \t {}", v1 * v2);
    println!("Div: \t {}", v1 / v2);
    println!("Rem: \t {}", v1 % v2);
    println!("AddScalar: \t {}", v1 + 10.0);
    println!("SubScalar: \t {}", v1 - 10.0);
    println!("MulScalar: \t {}", v1 * 10.0);
    println!("DivScalar: \t {}", v1 / 10.0);
    println!("Max: \t {}", v1.Max(&v2));
    println!("Min: \t {}", v1.Min(&v2));
    println!("MinAxis: \t {}", v2.MinAxis());
    println!("MinComponent: \t {}", v1.MinComponent());
    println!("MaxComponent: \t {}", v1.MaxComponent());

    //println!("{}", bench1(&v1, &v2));
    //println!("{}", bench1(&v1, &v2));
}

//fn bench1(v1: &pt::vector::Vector, v2: &pt::vector::Vector) -> u64 {
//    let t0 = SystemTime::now();
//    for _ in 1..(1000000000i64) {
//        let v = v1.Add(v2);
//        //println!("{}", v);
//    }
//    return t0.elapsed().unwrap().as_secs();
// }
//
//fn bench2(v1: &pt::vector::Vector, v2: &pt::vector::Vector) -> u64 {
//    let t0 = SystemTime::now();
//    for _ in 1..(1000000000i64) {
//        let v = *v1 + *v2;
//        //println!("{}", v);
//    }
//    return t0.elapsed().unwrap().as_secs();
// }
