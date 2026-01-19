use std::fs::File;
use std::io::{Error, Read};
use std::{fs, io};

fn main() {
    let file_result = File::open("hello.text");
    println!("{:?}", file_result);
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("panic {:?}", error),
    };
    let _result = fs::read_to_string("");
    // FormResidual
    let _result = read_file();
    let a = 5;
    let b: Result<(), MyError> = if a > 5 {
        Err(MyError::Io(Error::new(
            io::ErrorKind::InvalidInput,
            "输入为空",
        )))
    } else {
        Err(MyError::Io(Error::new(
            io::ErrorKind::InvalidInput,
            "输入为空2",
        )))
    };
    match b {
        Err(MyError::Io(error)) => {
            println!("{}", error);
        }
        Err(MyError::ParseInt(error)) => {
            println!("{}", error);
        }
        _ => {}
    }
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
    if false {
        return Err(MyError::Io(Error::new(
            io::ErrorKind::InvalidInput,
            "输入为空",
        )));
    }
    Ok(result)
}
