fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
            // 뭔가 return을 써도 될 것 같지만 안 된다.
        }
    };

    println!("The result is {}", result);
}
