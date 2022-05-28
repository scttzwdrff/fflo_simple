use rand::random;

pub fn random_u32_vector(n : usize) -> Vec<u32> {
	let mut output =vec![0u32;n];
	for x in output.iter_mut() {
		*x = random::<u32>();
	}
	output
}