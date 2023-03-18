/// Converts NUMBER to any number system into [u8; 33]
pub fn itoa_bytes_universal(num: isize, buf: &mut [u8; 33], num_sys: u8) -> usize {
    let mut _buf: [u8; 33] = [0; 33];
    let alphabet: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
        b'A', b'B', b'C', b'D', b'E', b'F',
    ];

    let mut n = num;
    let mut idx: usize = 0;
    let mut negative = false;

    if n == 0 {
        unsafe {
            *buf.get_mut(0).unwrap_unchecked() = b'0';
            return 1;
        }
    }

    if n < 0 {
        buf[0] = b'-';
        n = -n;
        negative = true;
    }

    while n > 0 {
        unsafe {
            *_buf.get_mut(idx).unwrap_unchecked() = alphabet[(n % num_sys as isize) as usize];
        }

        idx += 1;
        n /= num_sys as isize;
    }

    let mut ridx = idx;

    while idx > 0 {
        unsafe {
            *buf.get_mut(idx + if negative { 1 } else { 0 })
                .unwrap_unchecked() = *_buf.get(ridx - idx).unwrap_unchecked();
        }

        idx -= 1;
    }

    ridx += 1;

    unsafe {
        *buf.get_mut(ridx).unwrap_unchecked() = *_buf.get(0).unwrap_unchecked();
    }

    ridx + if negative { 1 } else { 0 }
}
