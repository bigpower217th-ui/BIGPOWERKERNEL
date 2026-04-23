#ifndef KERNEL_H
#define KERNEL_H

#include <stdint.h>

// --- 1. Temiz Veri Tipleri ---
// uint32_t vb. tipler stdint.h'den gelir, ama garantiye alalım
typedef unsigned char  uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int   uint32_t;

// --- 2. CPU Kayıtçı Yapısı (Interrupt Handler İçin) ---
// Bu yapı idt.asm'deki pusha sırasıyla tam uyumlu olmalıdır.
struct registers {
    uint32_t ds;                                     // Data segment selector
    uint32_t edi, esi, ebp, esp, ebx, edx, ecx, eax; // Pushed by pusha
    uint32_t int_no, err_code;                       // Interrupt number and error code
    uint32_t eip, cs, eflags, useresp, ss;           // Pushed by the processor automatically
};

// --- 3. Donanım Port Erişimi (Inline Assembly) ---
// outb: İşlemciden dış dünyaya (WiFi, VGA, Klavye) veri gönderir.
static inline void outb(uint16_t port, uint8_t val) {
    asm volatile ( "outb %0, %1" : : "a"(val), "Nd"(port) );
}

// inb: Dış dünyadan işlemciye veri okur.
static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile ( "inb %1, %0" : "=a"(ret) : "Nd"(port) );
    return ret;
}

// --- 4. Fonksiyon Prototipleri ---
void kprint(char *message);
void init_idt();

#endif
