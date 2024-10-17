use gen_db::data_analysis::*;
fn main() {
    let struct_set = StructSet::new().anlaysis("data_user_def.h.md", "data_user_def.c.md");
    println!("{:#?}", struct_set);
    let db_data_link = DbDataLink::new().analysis("data_user_def.c.md");
}
