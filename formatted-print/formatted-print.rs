fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob" );

    println!("{subject} {verb} {object}", 
    object = "the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);
    
    println!("{number:>0width$}", number=1, width=6);
    
    println!("{width:#018b}", width=6u32);

    println!("My name is {0}, {1}, {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);
    let s = Structure(3);
    let Structure(x) = s;
    println!("This struct `{:?}` won't print...", x);
    println!("Pi is roughly {:.6}", std::f64::consts::PI);
}