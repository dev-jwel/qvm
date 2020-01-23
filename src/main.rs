// TODOS
// 1. change address: usize parameter in measure function to addresses : Vec<usize>
// 2. change parameter to Vec<AddressedBit> in set_superposition too
// 3. implement pass_gate function
// 4. implement common gates
// 5. write tests for each functions or structs with its implementation

use qvm::QVM;

fn main() {
	println!("generating new qvm...");
	let mut qvm = QVM::new(25);
	qvm.measure(0);
	println!("hello qvm!!");
}
