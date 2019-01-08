use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

fn main() {
    let mut file = match File::open("hello.txt") {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err.description()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => (),
    };

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
        Ok(value) => value,
    };

    println!("Content is: {}", data);
}

// or a little bit simpler:

// use std::error::Error;
// use std::fs::read_to_string;

// fn main() {
//     match read_to_string("hello.txt") {
//         Err(err) => panic!("Couldn't read: {}", err.description()),
//         Ok(data) => println!("Content is: {}", data),
//     };
// }