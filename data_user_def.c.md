#include "db_user_def.h"
#include "db_data_def.h"

stPcuData_t stPcuData = {0};
stBootloaderData_t stBootloaderData = {0};
stDbUnit_t stDb[] = {
    { .pValue = &stBootloaderData.u16Scu0, .eRWType = eR_},
    { .pValue = &stPcuData.stPcuSn.data, .eRWType = eR_},
};
u16 cDbMax = (sizeof(stDb) / sizeof(stDbUnit_t));

