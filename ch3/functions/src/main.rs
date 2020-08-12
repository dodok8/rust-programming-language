fn main() {
    let x = plus_one(5);
    println!("x의 값 {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x+1; 이라 쓰면 에러 발생, 표현식과 구문을 구별하자
}
