use tiny_keccak::Keccak;

fixed_hash::construct_fixed_hash!{
	/// Uninterpreted 32 byte (256 bit) large hash type.
	pub struct H256(32);
}

/// Returns the Keccak hash (256-bits) of the given byte slice.
pub fn keccak(bytes: &[u8]) -> H256 {
    let mut keccak = Keccak::new_keccak256();
    let mut res = H256::zero();
    keccak.update(bytes);
    keccak.finalize(res.as_mut());
    res
}
