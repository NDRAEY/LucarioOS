#[inline]
pub fn align(value: usize, align: usize) -> usize {
    ((value) + ((!value) & ((align) - 1))) as usize
}

#[inline]
pub fn is_aligned(value: usize, align: usize) -> bool {
    value % align == 0
}
