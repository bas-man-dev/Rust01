#[derive(Debug)]


// trying out structs in Rust. Currently have to make functions outside, but it's not a problem.

struct Dog{
    name: String,
    weight: u8,
    breed: String,
}

fn bark(weight: u8){
    if weight > 15 {
        println!("Woof Woof");
    } else {
        println!("Yip Yip Yip");
    }
}
    

fn main() {
    let fido = String::from("Fido");
    let germ_shep = String::from("German Shephard");
    let tik = String::from("Tik");
    let chih = String::from("Chihuahua");
    let fido_stats = Dog{name: fido, weight: 35, breed: germ_shep};
    let tik_stats = Dog{name: tik, weight: 10, breed: chih};
    println!("{:?}", fido_stats);
    println!("{} is my {} dog, he weighs {} kg", fido_stats.name, fido_stats.breed, fido_stats.weight);
    bark(fido_stats.weight);
    println!("{:?}", tik_stats);
    println!("{} is my {} dog, he weighs {} kg", tik_stats.name, tik_stats.breed, tik_stats.weight);
    bark(tik_stats.weight);
}
