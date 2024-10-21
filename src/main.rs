use gen_db::data_analysis::*;
use gen_db::gen_file::*;

fn main() {
    // 分析头文件和C文件，构建StructSet
    let struct_set = StructSet::new().analyze_file("data_user_def.h.md", "data_user_def.c.md");

    // 分析C文件，构建DbData
    let db_data = DbData::new().analyze_file("data_user_def.c.md");

    // 根据解析的数据，计算结构体类型
    let type_name = resolve_types(struct_set, db_data.clone());

    //获取生成代码的必要数据
    let db_info = db_info(db_data.get_part_name(), db_data.get_last_part_name(), type_name);

    db_gen(db_info.clone());

    db_api(db_info);
}
