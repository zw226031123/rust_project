use std::rc::Rc;

#[test]
fn test(){
    let x = Rc::new(Example);
    let y = Rc::clone(&x);
    println!("A");
    drop(x);
    println!("B");
    drop(y);
    println!("C");

}

struct Example;
impl Drop for Example {
    fn drop(&mut self) {
        println!("Drop");
    }
}