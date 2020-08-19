fn main() {
    // Borrowing Example
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("'{}'의 길이는 {}입니다.", s1, len);

    let mut s = String::from("hello");

    // change(&s);
    // change(&mut s1);
    // 이 2개 다 mutability와 관련해서 에러가 나게 된다.

    change(&mut s);

    println!("{}", s);

    let r1 = &mut s;
    // let r2 = &mut s; 대여는 한번에 여러 번 할 수 없다.
    let r2 = &s;

    println!("{}, {}", r1, r2);

    // let reference_to_nothing = dangle();
    let correctly_refernce = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    return s;
}
