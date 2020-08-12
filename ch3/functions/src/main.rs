fn five() -> i32 {
    5
    // return 5;
    // 마지막에 있는 표현식은 return을 쓰지 않아도 반환된다.
}

fn main() {
    let x = five();

    println!("x의 값: {}", x);
}
