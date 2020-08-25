fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("사각형의 면적: {} 제곱픽셀", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}
