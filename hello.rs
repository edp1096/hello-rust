fn say(a: String) {
    println!("{}", a);
}
fn speak(b: &mut String) {
    println!("{}", b);

    b.push_str("me!!");
}
fn tell(c: &str) {
    println!("{}", &c);
}

fn main() {
    let hello: String = "Hello, world!".into();
    let world: &str = "Hello, world! 2";
    let mut help = String::from("Help!!");

    let s = hello.clone();

    say(hello);
    speak(&mut help);
    tell(&world);
    
    // println!("{}", hello);
    println!("{}", s);
    println!("{}", world);
    println!("{}", help);
}
