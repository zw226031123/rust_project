use rust_project::models::enums::YesNo;

fn main() {
    crate::test::test_mod::method();
    let _y = YesNo::YES;
    let _x = rust_project::models::structs::HousePrice {
        price: 100,
        area: String::from("xxx"),
        bed_rooms: 5,
        main_road: YesNo::YES,
    };
}

mod test {
    pub mod test_mod {
        pub fn method() {
            println!("method");
        }
    }
}

mod test_v2 {
    #[test]
    fn method_1() {
        test_mod_v2::method();
        self::test_mod_v2::method();
    }
    pub mod test_mod_v2 {
        #[test]
        pub fn method() {
            super::super::test::test_mod::method();
        }
    }
}
