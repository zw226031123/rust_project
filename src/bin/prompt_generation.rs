/*提示词生成*/
fn main() {
    //
    // println!(
    //     "{}",
    //     result_map_gen(
    //         "cn.publink.servicemobile.base.entity.FieldInfo",
    //         "MiddlewareFieldInfoDao.xml",
    //     )
    // );
    println!(
        "{}",
        result_map_gen_v2(
            "cn.publink.servicemobile.base.entity.card.CardField",
            "MiddlewareMapper.xml",
            Some(String::from("getListByFieldNameList")),
        )
    );
}
/*  resultmap 生成*/
#[allow(dead_code)]
fn result_map_gen(entity: &str, xml: &str) -> String {
    result_map_gen_v2(entity, xml, None)
}
/*  resultmap 生成 并修改使用*/
fn result_map_gen_v2(entity: &str, xml: &str, method: Option<String>) -> String {
    let entity_string = format!("为{}生成resultmap放在{}", entity, xml);
    // println!("{}", string);
    if let Some(method) = method {
        let method_string = format!("{}返回修改resultMap", method);
        // println!("{}", string1);
        return format!("{}\n{}", entity_string, method_string);
    }
    entity_string
}
