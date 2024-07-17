crate::ix!();

/**
  | Type which is returned when using the
  | subscript operator.
  | 
  | The returned type should be used just
  | like the type ElementType.
  |
  */
pub struct SIMDRegisterElementAccess<'a, T> {
    simd:  &'a mut SIMDRegister<T>,
    idx:   usize,
    value: Option<T>,
}

impl<'a, T: Copy> SIMDRegisterElementAccess<'a, T> {

    pub fn new(simd: &'a mut SIMDRegister<T>, idx: usize) -> Self {
        let value = Some(*simd.get(idx));
        SIMDRegisterElementAccess { simd, idx, value }
    }
}

impl<'a, T: Copy> Deref for SIMDRegisterElementAccess<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Directly return a reference to the element in the SIMD register
        self.simd.get(self.idx)
    }
}

impl<'a, T: Copy> DerefMut for SIMDRegisterElementAccess<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Directly return a mutable reference to the element in the SIMD register
        self.simd.get_mut(self.idx)
    }
}

