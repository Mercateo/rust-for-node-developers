use std::fs::File;
use std::io::{Read, Write, Error};

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = try!(File::open(path));

    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn write_file(path: &str, data: &str) -> Result<(), Error> {
    let mut file = try!(File::create(path));

    try!(file.write_all(data.as_bytes()));
    Ok(())
}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

    let hello_world = format!("{} {}!", hello, world);

    write_file("hello-world.txt", &hello_world).expect("Couldn't write 'hello-world.txt'.");
    println!("Wrote file 'hello-world.txt' with content: {}", hello_world);
}