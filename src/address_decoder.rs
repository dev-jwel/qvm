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


pub struct AddressDecoder {
	bits : usize,
	pinned_bits: Vec<AddressedBit>,
	counter: usize,
	len_counter: usize
}

// this iteration used to apply gate to all basis related with specific address
impl AddressDecoder {
	pub fn new(bits: usize, mut pinned_bits: Vec<AddressedBit>) -> AddressDecoder {
		let mut addresses : Vec<usize> = Vec::new();
		for i in 0 .. pinned_bits.len() {
			assert!(pinned_bits[i].bit == 0 || pinned_bits[i].bit == 1);
			addresses.push(pinned_bits[i].address);
		}
		assert!(is_valid_addresses(bits, &addresses));
		pinned_bits.sort_by_key(|k| k.address);
		AddressDecoder {bits: bits, counter: 0, len_counter: bits-pinned_bits.len(), pinned_bits: pinned_bits}
	}
}

impl Iterator for AddressDecoder {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if self.counter >= (1 << self.len_counter) {
			return None;
		}


		let mut decoded_address = self.counter;
		for i in 0 .. self.pinned_bits.len() {
			let address = self.pinned_bits[i].address;
			let bit_mask : usize = 1 << address;
			let low_mask : usize = bit_mask - 1;

			decoded_address = ((decoded_address & !low_mask) << 1) + (decoded_address & low_mask);
			if self.pinned_bits[i].bit == 1 {
				decoded_address += bit_mask;
			}
		}

		self.counter += 1;

		Some(decoded_address)
	}
}

#[test]
fn address_decoder_tester() {
	let mut addresses : Vec<AddressedBit> = Vec::new();
	addresses.push(AddressedBit{address: 1, bit: 0});
	addresses.push(AddressedBit{address: 2, bit: 1});

	let mut generated : Vec<usize> = Vec::new();
	let ad = AddressDecoder::new(4, addresses);
	for addr in ad {
		generated.push(addr);
	}

	assert_eq!(generated, vec![4, 5, 12, 13]);
}
