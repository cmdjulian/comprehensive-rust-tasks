use std::iter::Iterator;

fn main() {
    it();
    into_it();
    for_loop();
}

fn it() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");
}

fn into_it() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");
}

fn for_loop() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }
}
