use std::collections::LinkedList;
use std::fs;
use regex::Regex;

#[derive(Clone,Debug,PartialEq, Eq)]
pub struct Item {
    name: String,
    value: String,
}

impl Item {
    pub fn new(name: String, value: String, array_size: String) -> Self {
        let mut name = name;
        let value = value;

        if !array_size.is_empty() {
            name = format!("{}{}", name, array_size);
        }
        Item { name, value }
    }
}

#[derive(Clone,Debug,PartialEq, Eq)]
pub struct StructElement{
    name: String,
    value: Vec<Item>,
}

impl StructElement {
    fn new(name: &str, value: Item) -> Self {
        let name = name.to_string();
        StructElement { name, value: vec![value] }
    }
    fn add_value(&mut self, value: Item) {
        self.value.push(value);
    }
    pub fn find_item_by_value(&self, value: &str) -> Option<&Item> {
        self.value.iter().find(|item| item.value == value)
    }
}

#[derive(Clone,Debug,PartialEq, Eq)]
pub struct StructSet{
    value: Vec<StructElement>,
    struct_value: Vec<Item>,
}

impl StructSet {
    pub fn new() ->Self{
        StructSet { value: vec![], struct_value: vec![] }
    }
    pub fn add_value(&mut self, value: StructElement) {
        self.value.push(value);
    }
    pub fn add_item(&mut self, value: Item) {
        self.struct_value.push(value);
    }
    pub fn find_struct_value_name(&self, name: &str) -> &str {
        self.struct_value.iter().find(|item| item.name == name).unwrap().value.as_str()
    }
    pub fn anlaysis(mut self, file_path: &str, file_path_c: &str) ->Self{
        let file_content = fs::read_to_string(file_path)
        .expect("无法读取文件");

        // 正则表达式，用于匹配结构体定义
        let struct_re = Regex::new(r"typedef\s+struct\s*\{([\s\S]*?)\}\s*(\w+);").unwrap();

        // 修改后的正则表达式，用于匹配结构体中的字段（包括数组）
        let field_re = Regex::new(r"(\w+)\s+(\w+)(\[\d+\])?;").unwrap();

        // 迭代所有匹配到的结构体
        for struct_cap in struct_re.captures_iter(&file_content) {
            // 提取结构体内部内容和结构体名称
            let struct_body = &struct_cap[1];
            let struct_name = &struct_cap[2];
            let mut struct_element: Option<StructElement> = None;
            // 迭代结构体内部的所有字段
            for field_cap in field_re.captures_iter(struct_body) {
                let field_name = &field_cap[1];
                let field_value = &field_cap[2];
                let array_size = field_cap.get(3).map_or("", |m| m.as_str());

                let item = Item::new(field_name.to_string(), field_value.to_string(), array_size.to_string());
                if struct_element.is_none() {
                    struct_element = Some(StructElement::new(struct_name, item.clone()));
                } else {
                    struct_element.as_mut().unwrap().add_value(item.clone());
                }
            }
            self.add_value(struct_element.unwrap());
        }

        let file_content_c = fs::read_to_string(file_path_c)
        .expect("无法读取文件");

        // 正则表达式，用于匹配结构体定义
        let re: Regex = Regex::new(r"(st\w+_t)\s+(\w+)\s*=").unwrap();
        for caps in re.captures_iter(&file_content_c) {
            let struct_type = &caps[1];
            let variable_name = &caps[2];
            let item = Item::new(struct_type.to_string(), variable_name.to_string(), String::new());
            self.add_item(item);
        }
        self
    }
}

#[derive(Clone,Debug,PartialEq, Eq)]
pub struct DbUnitLink{
    value: Vec<String>
}

impl DbUnitLink {
    fn new() -> Self {
        DbUnitLink { value: vec![] }
    }
    fn add_value(&mut self, value: String) {
        self.value.push(value);
    }
    fn get_value(&self) -> Vec<String> {
        self.value.clone()
    }
}

#[derive(Clone,Debug,PartialEq, Eq)]
pub struct DbDataLink{
    value: Vec<DbUnitLink>
}

impl DbDataLink {
    pub fn new() -> Self {
        DbDataLink { value: vec![] }
    }
    pub fn add_value(&mut self, value: DbUnitLink) {
        self.value.push(value);
    }
    pub fn analysis(mut self, file_path: &str) -> Self{
        let file_content = fs::read_to_string(file_path)
        .expect("无法读取文件");

        // 正则表达式，用于匹配结构体定义
        let re = Regex::new(r"\.pValue\s*=\s*&?(.*?)(?:\[.*?\])?,").unwrap();
        
        // 查找所有匹配的值
        for cap in re.captures_iter(&file_content) {
            let data = cap[1].replace("&", "");
            let mut db_unit = DbUnitLink::new();
            let parts: Vec<_> = data.split('.').map(String::from).collect();
            for part in parts{
                db_unit.add_value(part);
            }
            self.add_value(db_unit.clone());
        }
        self
    }
}

pub fn calculate_size(struct_set: StructSet, db_data_link: DbDataLink){
    for db_unit in db_data_link.value{
        let db_unit_value = db_unit.get_value();
        for db_unit_value_element in db_unit_value{
            
        }
    }
}

/// 在链表main_list中查找是否包含链表sub_list，如果包含则返回最后一个匹配的节点，否则返回None
pub fn find_sublist_last_node<T: PartialEq + Clone>(
    main_list: &LinkedList<T>,
    sub_list: &LinkedList<T>,
 ) -> Option<T> {
    if sub_list.is_empty() {
        return None; // 空链表不返回任何节点
    }
    let mut main_iter = main_list.iter();
    let mut sub_iter = sub_list.iter();
    // 获取子链表的第一个元素
    let first_sub_elem = sub_iter.next().unwrap();
    // 遍历主链表，寻找与子链表第一个元素匹配的元素
    while let Some(main_elem) = main_iter.next() {
        if main_elem == first_sub_elem {
            // 找到匹配的元素，开始比较后续元素
            let mut main_iter_clone = main_iter.clone();
            let mut sub_iter_clone = sub_iter.clone();
            let mut last_matched = main_elem.clone();
            let mut is_sublist = true;
            while let Some(sub_elem) = sub_iter_clone.next() {
                if let Some(main_elem) = main_iter_clone.next() {
                    if main_elem != sub_elem {
                        is_sublist = false;
                        break;
                    }
                    last_matched = main_elem.clone();
                } else {
                    return None; // 主链表遍历完，但子链表还有元素
                }
            }
            if is_sublist {
                return Some(last_matched); // 返回最后一个匹配的节点
            }
        }
    }
    None // 未找到匹配的子链表
 }