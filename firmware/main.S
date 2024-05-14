#define __SFR_OFFSET 0
#include <avr/io.h>

.equ GYRO_ADDR, 0b11010000

.equ GYRO_XL, 0x27
.equ GYRO_XH, 0x29
.equ GYRO_YL, 0x2A
.equ GYRO_YH, 0x2B
.equ GYRO_ZL, 0x2C
.equ GYRO_ZH, 0x2D

.global main

main:
    sbi DDRB, 0
    ldi r16, 1<<TXEN
    out UCSRB, r16
    ldi r16, 1<<UCSZ1 | 1<<UCSZ0 | 1<<URSEL
    out UCSRC, r16
    ldi r16, 0x33
    out UBRRL, r16

    ldi r16, 0
    out TWSR, r16
    ldi r16, 0x47
    out TWBR, r16
    ldi r16, 1<<TWEN
    out TWCR, r16

loop:
    
    ldi r20, GYRO_ADDR
    rcall i2c_write

    ldi r20, GYRO_YL
    rcall i2c_write

    rcall i2c_read
    mov r20, r31
    rcall log_letter

    ldi r20, '\n'
    rcall log_letter

    rjmp loop

i2c_start:
    ldi r31, (1<<TWINT)|(1<<TWSTA)|(1<<TWEN)
    out TWCR, r31

i2c_start_wait:
    in r31, TWCR
    sbrs r31, TWINT
    rjmp i2c_start_wait

    ret

i2c_write: ; (r20)
    out TWDR, r20
    ldi r31, (1<<TWINT)|(1<<TWEN)
    out TWCR, r31

i2c_write_wait:
    in r31, TWCR
    sbrs r31, TWINT
    rjmp i2c_write_wait

    ret

i2c_read: ; (out r31)
    ldi r31, (1<<TWINT)|(1<<TWEN)
    out TWCR, r31

i2c_read_wait:
    in r31, TWCR
    sbrs r31, TWINT
    rjmp i2c_read_wait

    in r31, TWDR
    ret

i2c_stop: 
    ldi r31, (1<<TWINT)|(1<<TWSTO)|(1<<TWEN)
    out TWCR, r31
    ret


log_letter: ; (r20)
log_letter_loop:
    sbis UCSRA, UDRE
    rjmp log_letter_loop

    out UDR, r20
    ret

log_string:
log_string_loop:
    lpm r20, Z+
    rcall log_letter
    cpi r20, 0
    brne log_string_loop

    ret

delay:
    ldi r25, 0xA0
    out TCNT0, r25
    ldi r25, 0x05
    out TCCR0, r25
timer_overflow_loop:
    in r25, TIFR
    sbrs r25, TOV0
    rjmp timer_overflow_loop
    ldi r25, 0
    out TCCR0, r25
    ldi r25, 1<<TOV0
    out TIFR, r25
    ret

start_str: .asciz "Starting...\n"

