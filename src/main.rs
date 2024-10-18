use gen_db::data_analysis::{resolve_types, DbData, StructSet};

fn main() {
    // 分析头文件和C文件，构建StructSet
    let struct_set = StructSet::new().analyze_file("data_user_def.h.md", "data_user_def.c.md");
    //println!("Parsed StructSet:\n{:#?}", struct_set);

    // 分析C文件，构建DbData
    let db_data = DbData::new().analyze_file("data_user_def.c.md");
    println!("Parsed DbData:\n{:#?}", db_data.get_last_part());

    // 根据解析的数据，计算结构体类型
    let type_name = resolve_types(struct_set, db_data);
    println!("{:#?}", type_name);
}
