#[test]
fn test(){
    let x=5;
    let y=&x;
    assert_eq!(5,x);
    assert_eq!(5,*y);
    // drop(x);
}