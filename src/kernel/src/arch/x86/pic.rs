//! The 8259 Programmable Interrupt Controller (PIC) is one of the most
//! important chips making up the x86 architecture. Without it, the x86
//! architecture would not be an interrupt driven architecture. The function of
//! the 8259A is to manage hardware interrupts and send them to the appropriate
//! system interrupt. This allows the system to respond to devices needs without
//! loss of time (from polling the device, for instance).

//! It is important to note that APIC has replaced the 8259 PIC in more modern
//! systems, especially those with multiple cores/processors.

use super::io::{inb, io_wait, outb};

const PIC1: u16 = 0x20; /* IO base address for master PIC */
const PIC2: u16 = 0xa0; /* IO base address for slave PIC */
const PIC1_COMMAND: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_COMMAND: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

const ICW1_ICW4: u8 = 0x01; /* Indicates that ICW4 will be present */
const ICW1_SINGLE: u8 = 0x02; /* Single (cascade) mode */
const ICW1_INTERVAL: u8 = 0x04; /* Call address interval 4 (8) */
const ICW1_LEVEL: u8 = 0x08; /* Level triggered (edge) mode */
const ICW1_INIT: u8 = 0x10; /* Initialization - required! */

const ICW4_8086: u8 = 0x01; /* 8086/88 (MCS-80/85) mode */
const ICW4_AUTO: u8 = 0x02; /* Auto (normal) EOI */
const ICW4_BUF_SLAVE: u8 = 0x08; /* Buffered mode/slave */
const ICW4_BUF_MASTER: u8 = 0x0c; /* Buffered mode/master */
const ICW4_SFNM: u8 = 0x10; /* Special fully nested (not) */

const PIC_EOI: u8 = 0x20; /* End-of-interrupt command code */

#[doc(hidden)]
#[no_mangle]
pub fn pic_remap(offset1: u8, offset2: u8) {
	// let a1 = inb(PIC1_DATA);
	// let a2 = inb(PIC2_DATA);

	// starts the initialization sequence (in cascade mode)
	outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
	io_wait();

	outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
	io_wait();

	outb(PIC1_DATA, offset1); // ICW2: Master PIC vector offset
	io_wait();
	outb(PIC2_DATA, offset2); // ICW2: Slave PIC vector offset
	io_wait();
	outb(PIC1_DATA, 4); // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
	io_wait();
	outb(PIC2_DATA, 2); // ICW3: tell Slave PIC its cascade identity (0000 0010)
	io_wait();

	outb(PIC1_DATA, ICW4_8086); // ICW4: have the PICs use 8086 mode (and not 8080 mode)
	io_wait();
	outb(PIC2_DATA, ICW4_8086);
	io_wait();

	outb(PIC1_DATA, 0xff);
	outb(PIC2_DATA, 0xff);

	// Unmask both PICs.
	// outb(PIC1_DATA, a1);
	// outb(PIC2_DATA, a2);
}

/// Perhaps the most common command issued to the PIC chips is the end of
/// interrupt (EOI) command (code 0x20). This is issued to the PIC chips at the
/// end of an IRQ-based interrupt routine. If the IRQ came from the Master PIC,
/// it is sufficient to issue this command only to the Master PIC; however if
/// the IRQ came from the Slave PIC, it is necessary to issue the command to
/// both PIC chips.
pub fn send_eoi(irq: u8) {
	if irq >= 8 {
		outb(PIC2_COMMAND, PIC_EOI);
	}

	outb(PIC1_COMMAND, PIC_EOI);
}
