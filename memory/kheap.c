#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <kheap.h>

#define KERNEL_HEAP_START 0xD0000000
#define KERNEL_HEAP_SIZE  0x2000000
#define BLOCK_SIZE        32

static uint8_t heap_bitmap[KERNEL_HEAP_SIZE / BLOCK_SIZE / 8];

void set_bitmap(uint32_t bit) {
    heap_bitmap[bit / 8] |=(1 << (bit % 8));
  }

  void clear_bitmap(uint32_t bit) {
     heap_bitmap[bit / 8] &= ~(1 << (bit % 8));
  }

  void* kmalloc_ext(size_t size) {
     size_t blocks_needed = (size + sizeof(uint32_t) + BLOCK_SIZE - 1) / BLOCK_SIZE;

    for (size_t i = 0; i < (KERNEL_HEAP_SIZE / BLOCK_SIZE); i++) {

       bool found = true;
       for (size_t j = 0; j < blocks_needed; j++) {
          if (get_bitmap(i + j)) { found = false; break; }
          }

         if (found) {
           for (size_t j = 0; j < blocks_needed; j++) set_bitmap(i + j);
          uint32_t* addr = (uint32_t*)(KERNEL_HEAP_START + (i * BLOCK_SIZE));
          *addr = 0xDEADBEEF;
          return (void*)((uint32_t)addr + sizeof(uint32_t));
        }
      }
      return NULL;
  }  
  
          
