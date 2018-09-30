extern crate machine_id;

#[test]
fn test_get() {
	let id = machine_id::get().unwrap();
	for _ in 1..100 {
		let id2 = machine_id::get().unwrap();
		assert_eq!(id, id2);
	}
}
