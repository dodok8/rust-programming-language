fn main() {
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

    // Ownership and Scope
    let s = String::from("hello, world"); // 변수 s는 여기서 부터 유효하다

    takes_ownership(s); //  s는 takes_ownership 안으로 들어 갔으므로 유효하지 않다.

    makes_copy(x); // 스택 데이터는 복사가 일어나므로 여전히 유효하다.

    let s3 = gives_ownership(); // 리턴값이 변수 s3으로 옮겨진다.

    let s4 = String::from("hello"); // 변수 s4가 생성된다.

    let s5 = takes_and_gives_back(s4); //이후 리턴값이 s5로 옯겨진다.
} // 이 곳은 변수 s의 스코프 밖이다.
// 이곳은 s2의 범위 밖이지만, 이미 takes_and_gives_back으로 옮겨졌기 때문에 아무 일도 일어나지 않는다.


fn takes_ownership(some_string: String) {
    // some_string 변수가 범위 내에 생성되고
    println!("{}", some_string);
} // 여기를 벗어나면 some_string의 범위를 벗어나고, 'drop'이 호출된다.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
} // 클론된 변수는 범위를 벗어나도 아무일도 생기지 않는다.

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // 여기서 선언된 some_string의 소유권은

    return some_string; // return되고 난 후, 호출된 함수로 소유권이 옮겨진다.
}

fn takes_and_gives_back(a_string: String) -> String {
    //변수 a_string이 호출되는 순간
    a_string // 범위 내에 생성된다.
} // 이후 리턴하면 소유권은 호출한 함수로 옮겨진다.
