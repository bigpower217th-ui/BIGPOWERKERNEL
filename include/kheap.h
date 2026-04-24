#ifndef KHEAP_H
#define KHEAP_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#define KERNEL_HEAP_START 0xD0000000
#define KERNEL_HEAP_SIZE 0x2000000
#define BLOCK_SIZE       32

void set_bitmap(uint32_t bit);
void clear_bitmap(uint32_t bit);
bool get_bitmap(uint32_t bit);
void* kmalloc_ext(size_t size);

#endif
