#[derive(Clone)]
pub enum State {
	ZERO,         // qubit is collapsed and its state is zero
	ONE,          // qubit is collapsed and its state is zero
	SUPERPOSITION // qubit is in superposition
}

impl State {
	pub fn is_superposition(&self) -> bool {
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
