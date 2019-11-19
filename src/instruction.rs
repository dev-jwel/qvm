use crate::gate::Gate;

pub enum Instruction {
	INITIALIZE,
	MEASURE,
	GATE(Gate)
}

// 0 means that all length of parameter can passed
impl Instruction {
	pub fn parameter_length(&self) -> usize {
		match &*self {
			Instruction::INITIALIZE => 0,
			Instruction::MEASURE => 0,
			Instruction::GATE(gate) => gate.parameter_length()
		}
	}
}
