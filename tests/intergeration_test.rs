use rust_project::models::enums::YesNo;
use rust_project::models::structs::HousePrice;

#[test]
fn test() {
    let price = HousePrice {
        price: 100,
        area: String::from("zz"),
        bed_rooms: 3,
        main_road: YesNo::YES,
    };
}
