use std::option::Option;
use num::complex::Complex64;
use rand::random;

use crate::gate::Gate;
use crate::instruction::Instruction;
use crate::state::State;
use crate::qubit_counter::{QubitCounter, AddressedBit, is_valid_addresses};

fn abs(c: Complex64) -> f64 {
	c.re * c.re + c.im * c.im
}


pub struct QVM {
	// size of qubits
	bits: usize,

	// states of each qubits
	states: Vec<State>,

	// coefficient of each states and its absolute value is its probability
	// size of state is 2^bits
	// sum of all probability of basis must be 1
	basises: Vec<Complex64>
}

// basic methods
impl QVM {
	pub fn new(n: usize) -> QVM {
		let default_state = State::ZERO;
		let default_basis = Complex64 {re:0.0, im:0.0};

		let mut qvm = QVM {
			bits: n,
			states: vec![default_state; n],
			basises: vec![default_basis; 1 << n]
		};

		qvm.basises[0] = Complex64 {re:1.0, im:0.0};

		qvm
	}

	pub fn get_bits(&self) -> usize {
		self.bits
	}

	pub fn is_superposition(&self, n: usize) -> bool {
		if n < self.bits {
			self.states[n].is_superposition()
		} else {
			true
		}
	}

	// TODO
	pub fn execute(&self, ins: Instruction, params: Vec<u8>) -> Option<Vec<u8>> {
		match ins {
			_ => None
		}
	}
}

// TODOS : management, excution functions

// management
impl QVM {
	pub fn set_superposition(&mut self, address: usize, value: u8) -> Option<bool> {
		if (value != 0 && value != 1) || address >= self.bits {
			None
		} else if self.states[address].is_superposition() {
			Some(false)
		} else if self.states[address] == value {
			Some(true)
		} else {
			let mask = 1 << address;
			let pinned = vec![AddressedBit{address:address, bit:value}];
			let counter = QubitCounter::new(self.bits, pinned);

			for c in counter {
				let zero : Complex64 = Complex64 {re:0.0, im:0.0};

				// probability of diffirent state of qubit must be zero
				assert_eq!(self.basises[c], zero);
				self.basises[c] = self.basises[c ^ mask];
				self.basises[c ^ mask] = zero;
			}

			self.states[address] = State::SUPERPOSITION;
			Some(true)
		}
	}

	// TODO : make run for vector of address
	pub fn measure(&mut self, address: usize) -> Option<u8> {
		if address >= self.bits {
			None
		} else if ! self.states[address].is_superposition() {
			None
		} else {
			// pick basis randomly

			let mut rand : f64 = random();
			let mut raw_measurement : usize = 0;

			if rand > 1.0 {
				panic!();
			}

			for i in 0 ..  {
				rand -= abs(self.basises[i]);
				if rand < 0.0 {
					raw_measurement = i;
					break;
				}
			}

			if rand > 0.0 {
				panic!();
			}

			// collapse qubit statement

			// remove coefficient of opposite basis with measured value

			let mask = 1 << address;
			let measured = if raw_measurement & mask == 0 {0} else {1};
			let pinned = vec![AddressedBit{address:address, bit:measured}];
			let counter = QubitCounter::new(self.bits, pinned);
			let mut removed_probability : f64 = 0.0;

			for c in counter {
				let zero : Complex64 = Complex64 {re:0.0, im:0.0};
				removed_probability += abs(self.basises[c ^ mask]);
				self.basises[c ^ mask] = zero;
			}

			// multiply removed value to make sum of probability 1

			let pinned = vec![AddressedBit{address:address, bit:measured}];
			let counter = QubitCounter::new(self.bits, pinned);
			let weight = 1.0 / (1.0 - removed_probability).sqrt();

			for c in counter {
				self.basises[c] *= weight;
			}

			// change statement

			match measured {
				0 => self.states[address] = State::ZERO,
				1 => self.states[address] = State::ONE,
				_ => panic!()

			}

			Some(measured)
		}
	}
}

/**
 * To calculate all qubit, we need (2^n)x(2^n) size huge matrix and computation time is
 * O(4^n) mathmetically. However, we need only use fucntion that performs linear transformation
 * for better performance. Computation time of this implement is O(2^n).
 */
impl QVM {
	fn pass_gate(&self, gate: Gate, addresses: Vec<usize>) -> Option<bool> {
		assert!(is_valid_addresses(self.bits, &addresses));
		assert!(gate.parameter_length() == addresses.len());


		// apply gate for all qubits which are in superposition

		None
	}
}
