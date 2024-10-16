use gen_db::data_analysis::*;
fn main() {
    let struct_set = StructSet::new().anlaysis("data_user_def.md");
    println!("{:#?}",struct_set);
}
