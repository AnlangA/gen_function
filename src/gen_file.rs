use crate::data_analysis::*;
use crate::time::*;
use std::fs::OpenOptions;
use regex::Regex;
use std::io::{BufWriter, Write};

const DB_GEN_C_HEAD: &str = 
r#"#include "db_gen.h"

#define cLogLevel eEnLogLevelDebug
static const char *TAG = "db_gen";"#;

const DB_GEN_H_HEAD: &str =
r#"#pragma once
#include "db_data_def.h"
#include "en_log.h"

u16 sDbGetDataSize(stDbUnit_t* pData);
void sDbSetDataSize(stDbUnit_t* pData, u32 u32Index);"#;

const DB_GET_DATA_SIZE: &str =
r#"/**
 * @brief 获取数据的长度，单位是8bit
 * 
 * @param pData 数据库元素指针
 * @return u16 
 * @version 0.1
 * @author anlada (bo.hou@en-plus.com.cn)
 * @date [time]
 */
u16 sDbGetDataSize(stDbUnit_t* pData)
{
    u16 u16Size = 0;
    u16Size = pData->u16Bytes;
    return u16Size;
}"#;

const DB_SET_DATA_SIZE_0: &str =
r#"/**
 * @brief 设置数据长度，单位8bit
 * 
 * @param pData 数据库元素指针
 * @param u32Index 
 * @version 0.1
 * @author anlada (bo.hou@en-plus.com.cn)
 * @date [time]
 */
void sDbSetDataSize(stDbUnit_t* pData, u32 u32Index)
{
    switch(u32Index)
    {"#;

const DB_SET_DATA_SIZE_1: &str =
r#"        default:
            pData->u16Bytes = 0;
            break;
    }
}"#;

const DB_DATA_API_C_HEAD: &str =
r#"#include "db_data_api.h""#;

const DB_DATA_API_H_HEAD: &str =
r#"#pragma once
#include "en_common.h"
#include "db_data_def.h"
"#;

const DB_DATA_API_C_GET_DATA: &str =
r#"
/**
 * @brief get [<name>] Value
 */
void sDbGet<name>(<value_retrun_type> pData)
{
    stDbUnit_t* pDb = NULL;
    pDb = sDbGetData();
    
    sDbTakeMutex(&pDb[<key>]);
    memcpy(pData, (<value_type>*)sDbGetDataValue(&pDb[<key>]), pDb[<key>].u16Bytes);
    sDbGiveMutex(&pDb[<key>]);
}"#;

const DB_DATA_API_C_SET_DATA: &str = r#"
/**
 * @brief set [<key>] Value
 */
void sDbSet<name>(<value_type>* pDataNew)
{
    stDbUnit_t* pDb = NULL;
    <value_type> <value> = <init_value>;
    pDb = sDbGetData();
    
    sDbTakeMutex(&pDb[<key>]);
    <value> = *(<value_type>*)sDbGetDataValue(&pDb[<key>]);
    sDbGiveMutex(&pDb[<key>]);

    if(memcmp(&pDataNew, &<value>, pDb[<key>].u16Bytes) != 0)
    {
        sDbTakeMutex(&pDb[<key>]);
        memcpy((<value_type>*)sDbGetDataValue(&pDb[<key>]), pDataNew, pDb[<key>].u16Bytes);
        sDbSetNeedWriteFlag(&pDb[<key>]);
        sDbGiveMutex(&pDb[<key>]);
    }
}"#;

const DB_DATA_API_H: &str = r#"
void sDbGet<name>(<value_retrun_type> pData);
void sDbSet<name>(<value_type>* pDataNew);"#;

/// 生成db_gen.c文件
pub fn db_gen(data: Vec<DbInfoUnit>){
    
    db_gen_c(data);
    db_gen_h();
    
}

pub fn db_api(data: Vec<DbInfoUnit>){
    db_api_c(data.clone());
    db_api_h(data);
}

fn db_gen_c(data: Vec<DbInfoUnit>){
    let db_gen_c = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("db_gen.c")
    .unwrap();

    let time = get_current_time();
    let mut write_db_gen_c = BufWriter::new(db_gen_c);
    let db_gen_0 = DB_GEN_C_HEAD.as_bytes();
    let db_gen_1 = DB_GET_DATA_SIZE.replace("[time]", time.as_str());
    
    let _ = write_db_gen_c.write_all(db_gen_0);
    let _ = write_db_gen_c.write_all(db_gen_1.as_bytes());

    let mut db_set_size = DB_SET_DATA_SIZE_0.to_string();

    for (index, data_unit) in data.iter().enumerate() {
        let case_context = format!("\n        case {}:\n            pData->u16Bytes = sizeof({});\n            break;",index , data_unit.type_name());
        db_set_size = format!("{}{}", db_set_size, case_context);
    }
    db_set_size = format!("{}\n{}", db_set_size, DB_SET_DATA_SIZE_1);
    let _ = write_db_gen_c.write_all(db_set_size.as_bytes());
}

fn db_gen_h(){
    let db_gen_h = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("db_gen.h")
    .unwrap();

    let mut write_db_gen_h = BufWriter::new(db_gen_h);

    let db_gen_h = DB_GEN_H_HEAD.as_bytes();
    let _ = write_db_gen_h.write_all(db_gen_h);
}

fn db_api_c(data: Vec<DbInfoUnit>){
    let db_api_c = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("db_data_api.c")
    .unwrap();

    let mut write_db_api_c = BufWriter::new(db_api_c);

    let db_api_c_0 = DB_DATA_API_C_HEAD.as_bytes();
    let _ = write_db_api_c.write_all(db_api_c_0);

    let re = Regex::new(r"(\w+)(?:\[(\d+)\])?").unwrap();

    for (index, data_unit) in data.iter().enumerate(){

        let mut value_retrun_type = String::new();
        let mut value_type = String::new();
        let mut init_value = String::new();
        if let Some(captures) = re.captures(data_unit.type_name().as_str()) {
            // 提取类型
            let array_type = &captures[1];
            // 判断是否匹配到数组大小
            if let Some(_array_size) = captures.get(2) {
                init_value = String::from("0");
            } else {
                
                init_value = String::from("{0}");
            }
            value_retrun_type = format!("{}*", array_type);
            value_type = format!("{}", array_type);
        }

        let name = data_unit.name();
        let value = data_unit.value();
        //let value = remove_leading_lowercase_and_digits(&data_unit.value());

        let get_data = DB_DATA_API_C_GET_DATA.replace("<key>", index.to_string().as_str())
        .replace("<value_retrun_type>", value_retrun_type.as_str())
        .replace("<name>", name.as_str())
        .replace("<value_type>", value_type.as_str());

        let set_data = DB_DATA_API_C_SET_DATA.replace("<key>", index.to_string().as_str())
        .replace("<name>", name.as_str())
        .replace("<value>", value.as_str())
        .replace("<value_type>", value_type.as_str())
        .replace("<init_value>", init_value.as_str());

        let _ = write_db_api_c.write_all(get_data.as_bytes());
        let _ = write_db_api_c.write_all(set_data.as_bytes());
    }

    
}

fn db_api_h(data: Vec<DbInfoUnit>){
    let db_api_h = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("db_data_api.h")
    .unwrap();

    let mut write_db_api_h = BufWriter::new(db_api_h);

    let db_api_h_head = DB_DATA_API_H_HEAD.as_bytes();

    let _ = write_db_api_h.write_all(db_api_h_head);

    let re = Regex::new(r"(\w+)(?:\[(\d+)\])?").unwrap();

    for (_index, data_unit) in data.iter().enumerate(){

        let mut value_retrun_type = String::new();
        let mut value_type = String::new();
        if let Some(captures) = re.captures(data_unit.type_name().as_str()) {
            // 提取类型
            let array_type = &captures[1];
            // 判断是否匹配到数组大小
            if let Some(_array_size) = captures.get(2) {
                value_retrun_type = format!("{}*", array_type);
            } else {
                value_retrun_type = format!("{}*", array_type);
            }
            value_type = format!("{}", array_type);
        }

        let name = data_unit.name();

        let get_data = DB_DATA_API_H
        .replace("<value_retrun_type>", value_retrun_type.as_str())
        .replace("<name>", name.as_str())
        .replace("<value_type>", value_type.as_str());

        let _ = write_db_api_h.write_all(get_data.as_bytes());
    }
}