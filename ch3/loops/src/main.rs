fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("요소의 값: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사");

    for number in 1..4 {
        println!("{}!", number);
    }
    println!("발사");
}
