#define __SFR_OFFSET 0
#include <avr/io.h>

.global main

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
    sbis UCSRA, UDRE
    rjmp loop
    out UDR, r17
    #rcall delay
    rjmp loop


delay:
    ldi r25, 0
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

