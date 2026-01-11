use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_result = File::open("hello.text");
    println!("{:?}", file_result);
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("panic {:?}", error),
    };
    let result = fs::read_to_string("");
   // FormResidual
}
enum MyError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
}
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> MyError {
        MyError::Io(error)
    }
}
impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> MyError {
        MyError::ParseInt(error)
    }
}
fn read_file() -> Result<String, MyError> {
    let mut result = String::new();
    File::open("hello.txt")?.read_to_string(&mut result)?;
    Ok(result)
}
