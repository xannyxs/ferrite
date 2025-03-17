use crate::{println, PIC_1_OFFSET, PIC_2_OFFSET};

#[test_case]
fn test_pic_initialization() {
	use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

	// Constants for PIC ports
	const PIC1_COMMAND: u16 = 0x20;
	const PIC1_DATA: u16 = 0x21;
	const PIC2_COMMAND: u16 = 0xa0;
	const PIC2_DATA: u16 = 0xa1;

	unsafe {
		// First save the current mask values
		let mut pic1_data_port = Port::new(PIC1_DATA);
		let mut pic2_data_port = Port::new(PIC2_DATA);
		let original_mask1: u8 = pic1_data_port.read();
		let original_mask2: u8 = pic2_data_port.read();

		// Read the mask registers to verify they've been set properly
		let mask1: u8 = pic1_data_port.read();
		let mask2: u8 = pic2_data_port.read();

		// Verify that the masks match expected values
		// This will depend on your initialization function's behavior
		// For example, if your init function sets all interrupts to be masked:
		assert_eq!(
			mask1, 0xff,
			"PIC1 mask should be 0xFF after initialization"
		);
		assert_eq!(
			mask2, 0xff,
			"PIC2 mask should be 0xFF after initialization"
		);

		// Or, if you're expecting specific values:
		// assert_eq!(mask1, expected_mask1, "PIC1 mask doesn't match expected
		// value"); assert_eq!(mask2, expected_mask2, "PIC2 mask doesn't match
		// expected value");

		// Test if we can modify the mask and read it back
		let test_mask1 = 0xfe; // All masked except IRQ0
		pic1_data_port.write(test_mask1);
		let read_mask1: u8 = pic1_data_port.read();
		assert_eq!(read_mask1, test_mask1, "PIC1 mask wasn't properly updated");

		// Restore original masks
		pic1_data_port.write(original_mask1);
		pic2_data_port.write(original_mask2);
	}

	println!("PIC initialization test passed!");
}
