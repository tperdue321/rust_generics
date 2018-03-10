use std::fmt::Display;

fn main() {
    let v = vec![1,4,41,100,25];
    let largest_num = largest_i32(&v);
    println!("the largest number is {}", largest_num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest_char(&char_list);
    println!("the largest char is {}", largest_char);

    let largest_num = largest(&v);
    println!("the largest number is {}", largest_num);
    let largest_char = largest(&char_list);
    println!("the largest char is {}", largest_char);


    let integer = Point { x: 4, y: 5 };
    let float = Point { x: 2.2, y: 3.3 };

    // x is a different type than y so this fails
    // let failed_point = Point { x: 2, y: 4.4};
    let both_integer = MultiTypePoint { x: 5, y: 10 };
    let both_float = MultiTypePoint { x: 1.0, y: 4.0 };
    let integer_and_float = MultiTypePoint { x: 5, y: 4.0 };
    println!("{:?}", integer_and_float.x);

    println!("x => {:?}", integer.x());
    println!("x => {:?}", float.x());
    println!("distance from origin: {:?}", float.distance_from_origin());


    let another_int_and_float = both_integer.mixup(both_float);

    // cannot use both_integer x or y values after move happens on line 65 
    // because MultiTypePoint does not implement the Copy Trait
    // println!("both_integer.x = {}, both_integer.y = {}",
    //           both_integer.x, both_integer.y);
    println!("another_int_and_float.x = {}, another_int_and_float.y = {}",
              another_int_and_float.x, another_int_and_float.y);


    // using summary from our Summarizable trait
    let tweet = Tweet {
      username: String::from("SomeGuyOnTheInternet"),
      content: String::from("some ridiculous opinion"),
      reply: false,
      retweet: true
    };

    println!("1 new tweet: {}", tweet.summary());
    notify(tweet);

    let string1 = String::from("string1");
    let str2 = "longer str2";
    let result = longest(string1.as_str(), str2);
    println!("longest string: {}", result);

    // using a ref in a struct (see struct def down below)
    let string = String::from("some long chunk of text");
    let  excerpt = ImportantExcerpt{ part: &string[0..4]};

    println!("{}", excerpt.part);

    longest_with_announcement(string1.as_str(), str2, str2);
}

// same body code as largest_char
fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];
  for &number in list {
    if number > largest {
      largest = number;
    }
  }
  largest
}

// same body code as largest_i32
fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];
  for &char in list {
    if char > largest {
      largest = char;
    }
  }
  largest
}

// lets solve duplicate code through generics
// won't work for now because of trait issues.
// come back later
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}


// explore generic structs
struct Point<T> {
  x: T,
  y: T 
}

struct MultiTypePoint<T, U> {
  x: T,
  y: U
}

// implement a function for a generic type
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

// implement a function for a specific type
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

// return a mixture of 2 types of generic points
// as an example of what can be done with generics
impl<T,U> MultiTypePoint<T,U> {
  fn mixup<V,W>(self, other_point: MultiTypePoint<V,W>) -> MultiTypePoint<T,W> {
    MultiTypePoint {
      x: self.x,
      y: other_point.y 
    }
  }
}


// explore traits and applying similar behavior to
// different types of structs
// note the trait or the type must be local to the crate to impl
// e.g. you can't impl Display on a Vec
// (both are external as parts of the std lib)
pub trait Summarizable {

  // default impl can call other methods that don't have a default impl
  // this limits how much has to be defined by the Type that uses this trait
  fn summary(&self) -> String {
    format!("read more from: {}", &self.author_summary())
  }

  fn author_summary(&self) -> String;
}

// one kind of struct that has text that could be summarized
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impliment summary for a news article
impl Summarizable for NewsArticle {
  // override default 
  fn summary(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }

  fn author_summary(&self) -> String {
    format!("written by {}", self.author)
  }
}

// another kind of struct that has text that could be summarized
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// use default summary for a tweet 
// still requires impl for author_summary
impl Summarizable for Tweet {
  fn author_summary(&self) -> String {
    format!("@{}", self.username)
  }
}


// can specify traits of generics passed to functions to avoid
// compile time errors if it isn't clear if it can run certain functions
// e.g. our fn largest<T>(list: &[T]), can T execute the > operator?
pub fn notify<T: Summarizable>(item: T) {
  println!("here is your summary: {}", item.summary());
}

// two ways to specify multiple traits
//fn some_func <T: Display + Clone> -> i32 {...}
// or
//fn some_func<T> -> i32
//  where T: Display + Clone {...} 



// working with generic lifetimes

// structs must have a lifetime annotation if it holds a reference
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 'a specifies a generice lifetime (similiar to specifying generic types)
// it says that the return value will last as long as the shortest lifetime
// between str1 and str2
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  let mut longest = str1;
  if str2.len() > str1.len() {
    longest = str2;
  }
  longest
}

// arbitrary function to demonstrate using lifetime generics, type generics
// etc
fn longest_with_announcement<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
