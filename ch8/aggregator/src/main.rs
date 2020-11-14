pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}, ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{} : {}", self.username, self.content);
    }
}

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         return NewsArticle {
//             headline: String::from("대한민국, 독일 이기다"),
//             location: String::from("카잔 아레나, 러시아"),
//             author: String::from("위키백과"),
//             content: String::from("2:0"),
//         };
//     } else {
//         return Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("러스트 공부 시작"),
//             reply: false,
//             retweet: false,
//         };
//     }
// }

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_by_references<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list.iter() {
        // When you write for &item, this destructures each reference returned by the iterator, making the type of item T.
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    // let text = returns_summarizable(true);

    let news = NewsArticle {
        headline: String::from("대한민국, 독일 이기다"),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from("2:0"),
    };
    // println!("새 트윗 1개: {}", text.summarize());

    let number_list = vec![34, 50, 25, 100, 65];
    println!("가장 큰 숫자: {}", largest_by_references(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("가장 큰 문자: {}", largest(&char_list));
}
