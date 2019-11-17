use std::option::Option;
use num::complex::Complex64;

type Matrix = Vec<Vec<Complex64>>;

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

struct QVM {
	bits: usize,
	states: Vec<State>,
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
}

// management
impl QVM {
	fn setSuperposition(&self, n: usize) -> Option<bool> {
		None
	}

	fn measure(&self, n: usize) -> Option<u8> {
		None
	}
}

// basic gates

impl QVM {
	fn H(&self, n: usize) -> Option<bool> {
		None
	}

	fn SWAP(&self, a: usize, b: usize) -> Option<bool> {
		None
	}

	fn CSWAP(&self, a: usize, b: usize, c: usize) -> Option<bool> {
		None
	}
}

/**
 * To calculate all qubit, we need (2^n)x(2^n) size huge matrix and computation time is
 * O(4^n) mathmetically. However, we need only (2^m)x(2^m) size matrix where m is number of bit
 * which gate's input/output size. Computation time of this implement is O(2^n * 2^m).
 */
impl QVM {
	fn pass_gate(&self, addresses: Vec<usize>, gate: Matrix) -> Option<bool> {
		// check whether all address is valid
		// check addresses has same addresses
		// check size of matrix
		// ? check whether matrix is unitary

		// apply matrix for all qubits which are in superposition

		None
	}
}

fn main() {
	println!("generating new qvm...");
	let mut qvm = QVM::new(25);
	println!("hello qvm!!");
}
