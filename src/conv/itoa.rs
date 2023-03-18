pub fn itoa_bytes(num: isize, buf: &mut [u8; 12]) -> usize {
    let mut _buf: [u8; 12] = [0; 12];
    let mut n = num;
	let mut idx: usize = 0;
    let mut negative = false;

    if n < 0 {
        buf[0] = b'-';
        n = -n;
        negative = true;
    }

    while n > 0 {
    	_buf[idx] = b'0' + (n % 10) as u8;
    	idx += 1;
		n /= 10;
    }

    let mut ridx = idx;

    while idx > 0 {
    	buf[idx + if negative {1} else {0}] = _buf[ridx-idx];
    	idx -= 1;
    }
    
    ridx += 1;

   	buf[ridx] = _buf[0];

    ridx + if negative {1} else {0}
}