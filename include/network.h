#ifndef NETWORK_H
#define NETWORK_H

#include <stdint.h>

typedef struct {
    uint8_t data[1500];
    uint32_t length;
    uint8_t source_mac[6];
    uint8_t dest_mac[6];
} __attribute__((packed)) wifi_packet_t;

void wifi_hardware_init();
void wifi_send_raw(wifi_packet_t* packet);

extern int8_t rust_validate_packet(wifi_packet_t* packet);
extern void rust_encrypt_payload(wifi_packet_t* packet);

#endif
