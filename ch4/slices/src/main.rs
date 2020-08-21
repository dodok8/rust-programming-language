fn main() {
    let mut s = String::from("hello world");

    let word = fisrt_word(&s); //불변 참조 형성 -> 가변 참조가 형성될 수 없다.
    s.clear(); // 따라서 가변 참조가 필요한 부분에 에러가 발생하게 된다.

    println!("{}", word);
}

fn fisrt_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn first_word_slices(s: &str) -> &str { // 입력으로 문자열 스라이스를 받는다.  -> 슬라이스를 문자열로 바꿀 필요 없이 넣을 수 있다.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
