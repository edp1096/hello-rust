fn say(a: String) {
    println!("{}", a);
}
fn speak(b: &String) {
    println!("{}", b);
}
fn tell(c: &str) {
    println!("{}", &c);
}

fn main() {
    let hello: String = "Hello, world!".into();
    let world: &str = "Hello, world! 2";
    let help = String::from("Help!!");

    let s = hello.clone();

    say(hello);
    speak(&help);
    tell(&world);
    
    // println!("{}", hello);
    println!("{}", s);
    println!("{}", world);
    println!("{}", help);
}
