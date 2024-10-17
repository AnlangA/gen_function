use gen_db::data_analysis::*;
fn main() {
    let struct_set = StructSet::new().anlaysis("data_user_def.h.md");
    println!("{:#?}", struct_set);
    let file_path = "data_user_def.c.md";
    let db_data_link = DbDataLink::new().analysis(file_path);
    println!("{:#?}",db_data_link);
}
