use crate::data_analysis::*;
use crate::time::*;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufRead, BufReader, BufWriter, Write};
use std::path;

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


/// 生成db_gen.c文件
pub fn db_gen(data: Vec<DbInfoUnit>){
    
    db_gen_c(data);
    db_gen_h();
    
}

fn db_gen_c(data: Vec<DbInfoUnit>){
    let mut db_gen_c = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("db_gen.c")
    .unwrap();

    let time = get_current_time();
    let mut write_db_gen_c = BufWriter::new(db_gen_c);
    let db_gen_0 = DB_GEN_C_HEAD.as_bytes();
    let db_gen_1 = DB_GET_DATA_SIZE.replace("[time]", time.as_str());
    
    write_db_gen_c.write_all(db_gen_0);
    write_db_gen_c.write_all(db_gen_1.as_bytes());

    let mut db_set_size = DB_SET_DATA_SIZE_0.to_string();

    for (index, data_unit) in data.iter().enumerate() {
        let case_context = format!("\n        case {}:\n            pData->u16Bytes = sizeof({});\n            break;",index , data_unit.type_name());
        db_set_size = format!("{}{}", db_set_size, case_context);
    }
    db_set_size = format!("{}\n{}", db_set_size, DB_SET_DATA_SIZE_1);
    write_db_gen_c.write_all(db_set_size.as_bytes());
}

fn db_gen_h(){
    let mut db_gen_h = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("db_gen.h")
    .unwrap();

    let mut write_db_gen_h = BufWriter::new(db_gen_h);

    let db_gen_h = DB_GEN_H_HEAD.as_bytes();
    write_db_gen_h.write_all(db_gen_h);
}