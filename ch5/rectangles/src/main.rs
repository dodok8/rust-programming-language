#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn main() {
    /*
    let width1 = 30;
    let height1 = 50;

    println!("사각형의 면적: {} 제곱 픽셀", area(width1, height1));
    */
    /*
         let rect1 = (30, 50); // tuple structs 활용

         println!("사각형의 면적: {} 제곱 픽셀", area(rect1));
    */

    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("사각형의 면적: {} 제곱 픽셀", area(&rect1));
    println!("rect1: {:#?}", rect1);
}

// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1;
// }

fn area(rectangle: &Rectangle) -> u32 {
    //원본 객체에 대한 소유권을 아예 가져오지 않기 위해 빌려 쓴다.
    return rectangle.width * rectangle.heigth;
}
