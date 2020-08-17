fn main() {
    let mut s = String::from("hello"); // 변수 s는 여기서 부터 유효하다
    s.push_str(", world");
    println!("{}", s);

    // Move Example
    let s1 = String::from("hello");
    let s2 = s1; // 복사본을 저장할 것 같지만 복사본을 저장하지 않는다.
                 // string은 스택 영역에 3가지(포인터, 길이, 용량)이 저장되고, 포인터가 가리키는 힙 영역에 문자열 콘텐츠가 저장된다.
                 // s2에는 길이, 용량은 복사되어 스택에 들어가지만, 힙에 또 문자열 콘텐츠를 넣어서 그 새로운 주소를 반환하는 게 아니라 s1의 힙 포인터 주소를 반환한다.
                 // s2와 s1 둘 다 범위를 벗어나려고 하면 2번 할당 해제하려 해서 오류가 발생한다.
                 // rust의 경우, 이 경우 메모리를 복사하는 것이 아니라, s1이 유효하지 않다고 판단하여 s1에 연결된 할당을 해제한다. 이를 Move라고 표현한다.
    println!("{}, world", s1); //이미 Moved 되었기 컴파일 에러가 발생한다.
    println!("{}, world", s2);
    
    // Clone Example
    // 스택에만 새로 값을 할당하는 것이 아니라 heap 영역에 값을 복사하고 싶을 때 사용한다.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world", s1); //Clone은 새로 힙 영역에 할당하였기 때문에 
    println!("{}, world", s2);

    // Copy Exmaple

    let x = 5;
    let y = x;

    // 스택의 데이터는 컴파일 시점에 할당되는 크기를 할 수 있기 때문에 값이 2개 생긴다.(Copy)

    println!("x = {}", x);
    println!("y = {}", y);

} // 이 곳은 변수 s의 스코프 밖이다.
