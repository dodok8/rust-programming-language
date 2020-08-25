fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));

    let rect1 = (30, 50); // tuple structs 활용

    println!("사각형의 면적: {} 제곱 픽셀", area(rect1));
}

// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

fn area(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}
