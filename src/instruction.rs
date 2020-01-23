/*
use crate::gate::Gate;

pub enum Instruction {
	SetSuperposition,
	Measure,
	Gate(Gate)
}

// 0 means that all length of parameter can passed
impl Instruction {
	pub fn parameter_length(&self) -> usize {
		match &*self {
			Instruction::SetSuperposition => 2,
			Instruction::Measure => 0,
			Instruction::Gate(gate) => gate.parameter_length()
		}
	}
}
*/
