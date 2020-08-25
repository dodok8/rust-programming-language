#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 첫 번째 인자가 &self 메쏘드
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    // associated functions 구조체 자체의 인스턴스(&self)를 전달 받지 않음
    fn square(size: u32) -> Rectangle {
        //생성자 구현에 사용할 수 있다.
        Rectangle {
            width: size,
            height: size,
        }
    }
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
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(20);

    println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));
    println!("rect1: {:#?}", rect1);
}

// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1;
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     //원본 객체에 대한 소유권을 아예 가져오지 않기 위해 빌려 쓴다.
//     return rectangle.width * rectangle.height;
// }
