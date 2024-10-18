use regex::Regex;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    name: String,
    value: String,
}

impl Field {
    pub fn new(name: String, value: String, array_size: Option<&str>) -> Self {
        let name = match array_size {
            Some(size) => format!("{}{}", name, size),
            None => name,
        };
        Field { name, value }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructDefinition {
    name: String,
    fields: Vec<Field>,
}

impl StructDefinition {
    pub fn new(name: &str, field: Field) -> Self {
        StructDefinition {
            name: name.to_string(),
            fields: vec![field],
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn find_field_by_value(&self, value: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.value == value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructSet {
    definitions: Vec<StructDefinition>,
    fields: Vec<Field>,
}

impl StructSet {
    pub fn new() -> Self {
        StructSet {
            definitions: vec![],
            fields: vec![],
        }
    }

    pub fn add_definition(&mut self, definition: StructDefinition) {
        self.definitions.push(definition);
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn find_field_by_value(&self, value: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.value == value)
    }

    pub fn find_field_name_in_definition(
        &self,
        definition_name: &str,
        field_value: &str,
    ) -> Option<&str> {
        self.definitions
            .iter()
            .find(|def| def.name == definition_name)
            .and_then(|def| def.find_field_by_value(field_value))
            .map(|field| field.get_name())
    }

    pub fn analyze_file(mut self, struct_file: &str, usage_file: &str) -> Self {
        let struct_content = fs::read_to_string(struct_file).expect("Cannot read struct file");

        // 正则表达式，用于匹配结构体定义
        let struct_re = Regex::new(r"typedef\s+struct\s*\{([\s\S]*?)\}\s*(\w+);").unwrap();
        let field_re = Regex::new(r"([a-zA-Z_]\w*(?:\s*\*)?)\s+(\w+)(\[\d+\])?;").unwrap();

        // 解析 typedef struct
        for struct_cap in struct_re.captures_iter(&struct_content) {
            let struct_body = &struct_cap[1];
            let struct_name = &struct_cap[2];

            // 创建空的结构体定义，不再初始化时添加空字段
            let mut struct_def = StructDefinition {
                name: struct_name.to_string(),
                fields: Vec::new(),
            };

            // 解析每个字段
            for field_cap in field_re.captures_iter(struct_body) {
                let field_type = &field_cap[1];
                let field_name = &field_cap[2];
                let array_size = field_cap.get(3).map(|m| m.as_str());

                // 避免添加空字段
                if !field_type.is_empty() && !field_name.is_empty() {
                    let field =
                        Field::new(field_type.to_string(), field_name.to_string(), array_size);
                    struct_def.add_field(field);
                }
            }

            // 仅在字段不为空时添加该结构体定义
            if !struct_def.fields.is_empty() {
                self.add_definition(struct_def);
            }
        }

        let usage_content = fs::read_to_string(usage_file).expect("Cannot read usage file");
        let usage_re = Regex::new(r"(st\w+_t)\s+(\w+)\s*=").unwrap();

        for cap in usage_re.captures_iter(&usage_content) {
            let struct_type = &cap[1];
            let variable_name = &cap[2];
            self.add_field(Field::new(
                struct_type.to_string(),
                variable_name.to_string(),
                None,
            ));
        }

        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbLink {
    parts: Vec<String>,
}

impl DbLink {
    pub fn new() -> Self {
        DbLink { parts: Vec::new() }
    }

    pub fn add_part(&mut self, part: String) {
        self.parts.push(part);
    }

    pub fn get_parts(&self) -> Vec<String> {
        self.parts.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbData {
    links: Vec<DbLink>,
}

impl DbData {
    pub fn new() -> Self {
        DbData { links: Vec::new() }
    }

    pub fn add_link(&mut self, link: DbLink) {
        self.links.push(link);
    }

    pub fn analyze_file(mut self, file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Cannot read file");
        let re = Regex::new(r"\.pValue\s*=\s*&?(.*?)(?:\[.*?\])?,").unwrap();

        for cap in re.captures_iter(&content) {
            let raw_data = cap[1].replace("&", "");
            let mut db_link = DbLink::new();
            let parts: Vec<_> = raw_data.split('.').map(String::from).collect();
            for part in parts {
                db_link.add_part(part);
            }
            self.add_link(db_link);
        }

        self
    }

    pub fn get_part_name(&self) -> Vec<String> {
        self.links
            .iter()
            .map(|link| {
                let mut part_name: String = String::new();
                for part in link.get_parts() {
                    let part_string = remove_leading_lowercase_and_digits(&part);
                    part_name = format!("{}{}", part_name, part_string);
                }
                part_name
            })
            .collect()
    }
}

pub fn resolve_types(struct_set: StructSet, db_data: DbData) -> Vec<String>{
    let mut type_name = vec![];
    for mut link in db_data.links {
        let mut current_field_name = struct_set
            .find_field_by_value(&link.parts[0])
            .map(|field| field.get_name().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        link.parts.remove(0);

        for part in link.get_parts() {
            if let Some(field_name) =
                struct_set.find_field_name_in_definition(&current_field_name, &part)
            {
                current_field_name = field_name.to_string();
            }
        }
        type_name.push(current_field_name.clone());
    }
    type_name
}

pub fn remove_leading_lowercase_and_digits(input: &str) -> String {
    // 匹配开头的小写字母和数字
    let re = Regex::new(r"^[a-z0-9]+").unwrap();
    
    // 替换开头的小写字母和数字为空字符串
    re.replace(input, "").to_string()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbInfoUnit {
    name: String,
    type_name: String,
}

pub fn db_info(db_data: Vec<String>, type_name: Vec<String>) -> Vec<DbInfoUnit> {
    db_data.into_iter()
        .zip(type_name.into_iter())
        .map(|(name, type_name)| DbInfoUnit { name, type_name })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_set_analysis() {
        let struct_file_content = r#"
        typedef struct {
            int field1;
            char field2[10];
        } MyStruct;
        "#;
        let usage_file_content = r#"
        MyStruct myStructVar = {};
        "#;

        // 模拟文件读写
        let struct_file_path = "test_struct.h";
        let usage_file_path = "test_struct.c";
        fs::write(struct_file_path, struct_file_content).unwrap();
        fs::write(usage_file_path, usage_file_content).unwrap();

        let struct_set = StructSet::new().analyze_file(struct_file_path, usage_file_path);

        // 添加调试输出
        println!("{:#?}", struct_set); // 打印struct_set，查看解析结果

        // 验证解析结果
        assert_eq!(struct_set.definitions.len(), 1);
        assert_eq!(struct_set.definitions[0].name, "MyStruct");
        assert_eq!(struct_set.definitions[0].fields.len(), 2);
        assert_eq!(struct_set.definitions[0].fields[0].name, "int");
        assert_eq!(struct_set.definitions[0].fields[1].name, "char[10]");
        println!("{:#?}", struct_set.definitions[0].fields[1].name);
    }

    #[test]
    fn test_db_data_analysis() {
        let c_file_content = r#"
        .pValue = &myStructVar.field1,
        "#;

        let file_path = "test_data.c";
        fs::write(file_path, c_file_content).unwrap();

        let db_data = DbData::new().analyze_file(file_path);

        // 验证解析结果
        assert_eq!(db_data.links.len(), 1);
        assert_eq!(db_data.links[0].parts[0], "myStructVar");
        assert_eq!(db_data.links[0].parts[1], "field1");
    }

    #[test]
    fn test_resolve_types() {
        let struct_set = StructSet::new().analyze_file("data_user_def.h.md", "data_user_def.c.md");
        let db_data = DbData::new().analyze_file("data_user_def.c.md");

        // 不验证实际输出，仅确保不会出现panic
        resolve_types(struct_set, db_data);
    }
}
