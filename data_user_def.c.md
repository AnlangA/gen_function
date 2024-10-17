#include "db_user_def.h"
#include "db_data_def.h"

stPcuData_t stPcuData = {0};
stBootloaderData_t stBootloaderData = {0};
stDbUnit_t stDb[] = {
    { .pValue = &stBootloaderData.u16Scu0.a.b.c.e.f[12], .eRWType = eR_},
    { .pValue = &stBootloaderData.u16Scu1, .eRWType = eR_},
};
u16 cDbMax = (sizeof(stDb) / sizeof(stDbUnit_t));

