// TODOS
// 1. change address: usize parameter in measure function to addresses : Vec<usize>
// 2. change parameter to Vec<AddressedBit> in set_superposition too
// 3. implement pass_gate function
// 4. implement common gates
// 5. write tests for each functions or structs with its implementation

use qvm::QVM;
use qvm::gate::Gate;

fn main() {
	println!("generating new qvm...");
	let mut qvm = QVM::new(8);

	println!("set superposition...");
	qvm.set_superposition(0, 1);
	for i in 1 .. 8 {
		qvm.set_superposition(i, 1);
	}

	println!("pass hadamard...");
	qvm.pass_gate(Gate::H, vec![0; 1]);

	qvm.measure(0);
	println!("hello qvm!!");
}
