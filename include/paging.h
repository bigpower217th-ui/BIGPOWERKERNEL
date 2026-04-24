#ifndef PAGING_H
#define PAGING_H

#include <stdint.h>

void* request_physical_page(void);
void security_panic(const char *msg, uint32_t address);

#endif
