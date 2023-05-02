#[no_mangle]
pub unsafe extern "C" fn memset(pointer: *mut u8, value: u8, count: usize) {
    let mut c = count;

    while c > 0 {
        *pointer.offset(c as isize).as_mut().unwrap_unchecked() = value;
        c -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(destination: *mut u8, source: *const u8, count: usize) {
    let mut c = count;

    while c > 0 {
        *destination.offset(c as isize).as_mut().unwrap_unchecked() = *source.offset(c as isize);
        c -= 1;
    }
}