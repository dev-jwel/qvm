// TODOS
// 1. change address: usize parameter in measure function to addresses : Vec<usize>
// 2. change parameter to Vec<AddressedBit> in set_superposition too
// 3. implement pass_gate function
// 4. implement common gates
// 5. write tests for each functions or structs with its implementation
// 6. split source code file
// 7. make package


use std::option::Option;
use num::complex::Complex64;
use rand::random;

fn is_valid_addresses(bits: usize, addresses: & Vec<usize>) -> bool {
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

fn abs(c: Complex64) -> f64 {
	c.re * c.re + c.im * c.im
}

#[derive(Clone)]
enum State {
	ZERO,         // qubit is collapsed and its state is zero
	ONE,          // qubit is collapsed and its state is zero
	SUPERPOSITION // qubit is in superposition
}

impl State {
	fn is_superposition(&self) -> bool {
		match *self {
			State::SUPERPOSITION => true,
			_ => false,
		}
	}
}

impl PartialEq<u8> for State {
    fn eq(&self, other: &u8) -> bool {
		match *self {
			State::ZERO => *other == 0,
			State::ONE => *other == 1,
			_ => false,
		}
    }
}

enum Instruction {
	INITIALIZE,
	MEASURE,
	GATE(Gate)
}

// 0 means that all length of parameter can passed
impl Instruction {
	fn parameter_length(&self) -> usize {
		match &*self {
			Instruction::INITIALIZE => 0,
			Instruction::MEASURE => 0,
			Instruction::GATE(gate) => gate.parameter_length()
		}
	}
}

enum Gate {
	H,
	SWAP,
	CSWAP
}

impl Gate {
	fn parameter_length(&self) -> usize {
		match *self {
			Gate::H => 1,
			Gate::SWAP => 2,
			Gate::CSWAP => 3
		}
	}

	fn to_function(&self) -> (fn(Vec<Complex64>) -> Vec<Complex64>){
		match *self {
			Gate::H => Gate::hadamard,
			Gate::SWAP => Gate::swap,
			Gate::CSWAP => Gate::cswap
		}
	}


	fn hadamard(mut v: Vec<Complex64>) -> Vec<Complex64> {
		assert_eq!(v.len(), 1 << Gate::H.parameter_length());
		let square_root = 2.0f64.sqrt();
		let out0 = square_root * (v[0] + v[1]);
		let out1 = square_root * (v[0] - v[1]);
		v[0] = out0;
		v[1] = out1;
		v
	}

	fn swap(mut v: Vec<Complex64>) -> Vec<Complex64> {
		assert_eq!(v.len(), 1 << Gate::SWAP.parameter_length());
		let tmp = v[1];
		v[1] = v[2];
		v[2] = tmp;
		v
	}

	fn cswap(mut v: Vec<Complex64>) -> Vec<Complex64> {
		assert_eq!(v.len(), 1 << Gate::CSWAP.parameter_length());
		let tmp = v[5];
		v[5] = v[6];
		v[6] = tmp;
		v
	}
}

struct AddressedBit {
	address: usize,
	bit : u8
}

struct QubitCounter {
	bits : usize,
	pinned_bits: Vec<AddressedBit>,
	counter: usize,
	len_counter: usize
}

// this iteration used to apply gate to all basis related with specific address
impl QubitCounter {
	fn new(bits: usize, mut pinned_bits: Vec<AddressedBit>) -> QubitCounter {
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

struct QVM {
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
	fn new(n: usize) -> QVM {
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

	fn get_bits(&self) -> usize {
		self.bits
	}

	fn is_superposition(&self, n: usize) -> bool {
		if n < self.bits {
			self.states[n].is_superposition()
		} else {
			true
		}
	}

	// TODO
	fn execute(&self, ins: Instruction, params: Vec<u8>) -> Option<Vec<u8>> {
		match ins {
			_ => None
		}
	}
}

// TODOS : management, excution functions

// management
impl QVM {
	fn set_superposition(&mut self, address: usize, value: u8) -> Option<bool> {
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
	fn measure(&mut self, address: usize) -> Option<u8> {
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

fn main() {
	println!("generating new qvm...");
	let qvm = QVM::new(25);
	println!("hello qvm!!");
}
