#pragma once
#include "db_config.h"
#include "en_common.h"


typedef struct
{
    //可存储32个字符，留一个空白字符，用于判断字符串结束
    u8 u8Data[33];
}stChar_16_t;


/**
 * @brief pcu数据
 */
typedef struct
{
    stChar_16_t stPcuSn;
}stPcuData_t;


/**
 * @brief bootloader数据
 */
typedef struct
{
    u16 u16Scu0;
    u16 u16Scu1;
}stBootloaderData_t;

