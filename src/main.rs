// TODOS
// 1. change address: usize parameter in measure function to addresses : Vec<usize>
// 2. change parameter to Vec<AddressedBit> in set_superposition too
// 3. implement pass_gate function
// 4. implement common gates
// 5. write tests for each functions or structs with its implementation

use qvm::QVM;
use qvm::gate::Gate;

fn main() {
	let n = 3;
	println!("generating new qvm...");
	let mut qvm = QVM::new(n);
	qvm.print_register();

	println!("set superposition...");
	for i in 0 .. n {
		qvm.set_superposition(i, 1);
	}
	qvm.print_register();

	println!("pass hadamard...");
	for i in 0 .. n {
		qvm.pass_gate(Gate::H, vec![i]);
	}
	qvm.print_register();

	println!("measure...");
	qvm.measure(0);
	qvm.print_register();
}
