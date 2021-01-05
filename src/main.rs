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
	let mut qvm = QVM::new(3);
	qvm.print_register();

	println!("set superposition...");
	qvm.set_superposition(0, 1);
	qvm.set_superposition(1, 1);
	qvm.set_superposition(2, 1);
	qvm.print_register();

	println!("pass hadamard...");
	qvm.pass_gate(Gate::H, vec![0]);
	qvm.pass_gate(Gate::H, vec![1]);
	qvm.pass_gate(Gate::H, vec![2]);
	qvm.print_register();

	println!("measure...");
	let value =  qvm.measure(0).unwrap();
	println!("value : {}",value);
	qvm.print_register();

	println!("set superposition...");
	qvm.set_superposition(0, value);
	qvm.print_register();

	println!("pass swap...");
	qvm.pass_gate(Gate::SWAP, vec![0, 2]);
	qvm.print_register();

	println!("measure...");
	let value =  qvm.measure(0).unwrap();
	println!("value : {}",value);
	qvm.print_register();
}
