use std::option::Option;

pub struct AddressedBit {
	pub address: usize,
	pub bit : u8
}

pub fn is_valid_addresses(bits: usize, addresses: & Vec<usize>) -> bool {
	if bits == 0 {
		return false
	} else if addresses.len() > bits {
		return false
	}

	for i in 0 .. addresses.len() {
		if addresses[i] >= bits {
			return false
		}
		for j in i+1 .. addresses.len() {
			if addresses[i] == addresses[j] {
				return false
			}
		}
	}

	true
}


pub struct QubitCounter {
	bits : usize,
	pinned_bits: Vec<AddressedBit>,
	counter: usize,
	len_counter: usize
}

// this iteration used to apply gate to all basis related with specific address
impl QubitCounter {
	pub fn new(bits: usize, mut pinned_bits: Vec<AddressedBit>) -> QubitCounter {
		let mut addresses : Vec<usize> = Vec::new();
		for i in 0 .. pinned_bits.len() {
			assert!(pinned_bits[i].bit == 0 || pinned_bits[i].bit == 1);
			addresses.push(pinned_bits[i].address);
		}
		assert!(is_valid_addresses(bits, &addresses));
		pinned_bits.sort_by_key(|k| k.address);
		QubitCounter {bits: bits, counter: 0, len_counter: bits-pinned_bits.len(), pinned_bits: pinned_bits}
	}
}

impl Iterator for QubitCounter {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if self.counter >= (1 << self.len_counter) {
			return None;
		}

		let prev_counter = self.counter;
		self.counter += 1;

		let mut counter_index = 0;
		let mut pinned_index = 0;
		let mut basis : usize = 0;

		for i in 0 .. self.bits {
			if pinned_index < self.pinned_bits.len() && self.pinned_bits[pinned_index].address == i {
				basis += (self.pinned_bits[pinned_index].bit as usize) << i;
				pinned_index += 1;
			} else {
				basis += (prev_counter & (1 << counter_index)) << (i - counter_index);
				counter_index += 1;
			}
		}

		Some(basis)
	}
}

#[test]
fn qubit_counter_tester() {
	let mut addresses : Vec<AddressedBit> = Vec::new();
	addresses.push(AddressedBit{address: 1, bit: 0});
	addresses.push(AddressedBit{address: 2, bit: 1});

	let mut generated : Vec<usize> = Vec::new();
	let qc = QubitCounter::new(4, addresses);
	for addr in qc {
		generated.push(addr);
	}

	assert_eq!(generated, vec![4, 5, 12, 13]);
}
