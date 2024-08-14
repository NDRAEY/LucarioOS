/// Converts NUMBER to any number system into [u8; 33]
pub fn itoa_bytes_universal(num: isize, buf: &mut [u8; 33], num_sys: u8) -> usize {
    let mut _buf: [u8; 33] = [0; 33];
    let alphabet: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        b'F',
    ];

    let mut n = num;
    let mut idx: usize = 0;
    let mut negative = false;

    if n == 0 {
        *buf.get_mut(0).unwrap() = b'0';
        return 1;
    }

    if n < 0 {
        n = -n;
        negative = true;
    }

    while n > 0 {
        *_buf.get_mut(idx).unwrap() = *alphabet.get((n % num_sys as isize) as usize).unwrap();

        idx += 1;
        n /= num_sys as isize;
    }

    if negative {
        _buf[idx] = b'-';
        idx += 1;
    }

    for i in 0..idx {
        *buf.get_mut(i).unwrap() = *_buf.get(idx - i - 1).unwrap();
    }

    idx
}

pub fn itoa_bytes_universal_unsigned(num: usize, buf: &mut [u8; 33], num_sys: u8) -> usize {
    let mut _buf: [u8; 33] = [0; 33];
    let alphabet: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        b'F',
    ];

    let mut n = num;
    let mut idx: usize = 0;

    if n == 0 {
        *buf.get_mut(0).unwrap() = b'0';
        return 1;
    }

   while n > 0 {
        *_buf.get_mut(idx).unwrap() = *alphabet.get(n % num_sys as usize).unwrap();

        idx += 1;
        n /= num_sys as usize;
    }

    for i in 0..idx {
        *buf.get_mut(i).unwrap() = *_buf.get(idx - i - 1).unwrap();
    }

    idx

}
