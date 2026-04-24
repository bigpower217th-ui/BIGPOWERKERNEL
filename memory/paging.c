#include <stdint.h>
#include <stdbool.h>
#include "paging.h"

#define PAGE_SIZE 4096
#define PT_INDEX(vaddr) ((vaddr) >> 22)
#define PD_INDEX(vaddr) (((vaddr) >> 22) & 0x03FF)

#define PAGE_PRESENT 0x1
#define PAGE_RW      0x2
#define PAGE_USER    0X4
#define PAGE_NX      (1ULL << 63)

uint32_t kernel_page_directory[1024] __attribute__((aligned(4096)));

void map_page(uint32_t phys, uint32_t virt, uint32_t flags) {
      uint32_t pd_idx = PD_INDEX(virt);
      uint32_t pt_idx = PD_INDEX(virt);

      if(!(kernel_page_directory[pd_idx] & PAGE_PRESENT)) {
	  

         uint32_t new_pt = (uint32_t)request_physical_page();
	 kernel_page_directory[pd_idx] = new_pt | flags | PAGE_PRESENT;
}

        uint32_t* table = (uint32_t*)(kernel_page_directory[pd_idx] & ~0xFFF);
        table[pt_idx] = phys | flags | PAGE_PRESENT;


        asm volatile("mov %%cr3, %%eax\n\tmov %%eax, %%cr3" ::: "eax");
}


void page_fault_handler(uint32_t faulting_address, uint32_t error_code) {

    if (error_code & 0x1) {

      security_panic("Cyber ​​armor Unauthorized writing attempt", faulting_address);
   } else {

       security_panic("Cyber ​​armor Invalid memory access", faulting_address);
   }
}
