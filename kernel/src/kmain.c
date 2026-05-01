#include "../include/kernel.h"


extern void init_gdt();
extern void kprint(char *message);

extern void wifi_encrypt_layer(void* data, int len);
void kmain() {
   init_gdt();
   init_gdt();
kprint("CyberArmor OS Loading...");

 char test_packet[] = "DATA";
wifi_encrypt_layer(test_packet, 4);



 while(1);
}
