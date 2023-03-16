use crate::{debug, debug_str, log::log::debug_write_string};

#[macro_export]
macro_rules! fault {
	() => {
		__fault_kernel("<no message>");
	};

	($msg:expr) => {
		crate::log::fault::__fault_kernel($msg);
	};

	($($msgs:expr),*) => {
		$(
		__fault_kernel($msg);
		)*
	};
}

pub fn __fault_kernel(message: &str) -> ! {
    debug!("Kernel Fault: ", message);
    loop {}
}
