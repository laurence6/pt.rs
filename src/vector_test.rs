use vector::Vector;

#[test]
fn vecs() {
    let v1 = Vector::New(2.0, 3.0, 5.0);
    let v2 = Vector::New(6.0, 4.0, 2.0);
    println!("v1: { : ? }\nv2: { : ? }", v1, v2);

    println!("Add: \t { : ? }", v1 + 2.0);
    println!("Add: \t { : ? }", v1 + v2);

    println!("Length: \t { : ? }", v1.Length());
    println!("Normalize: \t { : ? }", v1.Normalize());
    println!("Neg: \t { : ? }", -v1);
    println!("Abs: \t { : ? }", (-v1).Abs());
    println!("Dot: \t { : ? }", v1.Dot(&v2));
    println!("Cross: \t { : ? }", v1.Cross(&v2));
    println!("Add: \t { : ? }", v1 + v2);
    println!("Sub: \t { : ? }", v1 - v2);
    println!("AddScalar: \t { : ? }", v1 + 10.0);
    println!("SubScalar: \t { : ? }", v1 - 10.0);
    println!("MulScalar: \t { : ? }", v1 * 10.0);
    println!("DivScalar: \t { : ? }", v1 / 10.0);
    println!("Max: \t { : ? }", v1.Max(&v2));
    println!("Min: \t { : ? }", v1.Min(&v2));
    println!("MinAxis: \t { : ? }", v2.MinAxis());
    println!("MinComponent: \t { : ? }", v1.MinComponent());
    println!("MaxComponent: \t { : ? }", v1.MaxComponent());
}
