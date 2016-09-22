#![no_std]

fn foo() -> u64 {
	0
}

pub extern "win64" fn bar() -> u64 {
	foo()
}
