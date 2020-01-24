use num::complex::Complex64;

pub enum Gate {
	H,
	SWAP,
	CSWAP
}

impl Gate {
	pub fn parameter_length(&self) -> usize {
		match *self {
			Gate::H => 1,
			Gate::SWAP => 2,
			Gate::CSWAP => 3
		}
	}

	pub fn to_function(&self) -> (fn(Vec<Complex64>) -> Vec<Complex64>){
		match *self {
			Gate::H => hadamard,
			Gate::SWAP => swap,
			Gate::CSWAP => cswap
		}
	}
}

fn hadamard(mut v: Vec<Complex64>) -> Vec<Complex64> {
	assert_eq!(v.len(), 1 << Gate::H.parameter_length());
	let square_root = 2.0f64.sqrt();
	let out0 = (1.0 / square_root) * (v[0] + v[1]);
	let out1 = (1.0 / square_root) * (v[0] - v[1]);
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
