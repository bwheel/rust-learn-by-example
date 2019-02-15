fn main() {
    let logical: bool = true;
    println!("locigal:{}", logical);
    let a_float: f64 = 1.0;
    println!("a_float:{}", a_float);
    let an_integer = 5i32;
    println!("an_integer:{}", an_integer);
    let default_float = 3.0;
    println!("default_float:{}", default_float);
    let default_integer = 7;
    println!("default_integer:{}", default_integer);

    let mut inferred_type = 12;
    println!("inferred_type:{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("inferred_type:{}", inferred_type);

    let mut mutable = 12;
    println!("mutable:{}", mutable);

    mutable = 21;
    println!("mutable:{}", mutable);


    let mutable = true;
    println!("mutable:{}", mutable);
}