fn say(a: String) {
    println!("{}", a);
}
fn tell(b: &str) {
    println!("{}", &b);
}

fn main() {
    let hello: String = "Hello, world!".into();
    let world: &str = "Hello, world! 2";

    let s = hello.clone();

    say(hello);
    tell(&world);
    
    // println!("{}", hello);
    println!("{}", s);
    println!("{}", world);
}
