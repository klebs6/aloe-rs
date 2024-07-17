crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/containers/aloe_SIMDRegister_Impl.h]

/// Zeroes out the elements of a SIMDRegister.
/// The actual logic will depend on the internal details of SIMDRegister.
pub fn snap_to_zero<T: HasVSimdType>(reg: &mut SIMDRegister<T>)
where
    T: Default,
{
    todo!();
    // Implementation logic here; likely a loop setting elements to T::default()
    // Note: noexcept in C++ indicates that the function won't throw exceptions.
    // In Rust, functions do not throw exceptions, so there's no direct equivalent.
}

/// Minimum element-wise between two SIMDRegisters.
pub fn jmin<T: HasVSimdType + Copy>(a: SIMDRegister<T>, b: SIMDRegister<T>) -> SIMDRegister<T>
where
    T: PartialOrd, // Allow types that can be partially ordered
{
    SIMDRegister::min(a, b) // Assumes a class method `min` on SIMDRegister
}

/// Maximum element-wise between two SIMDRegisters.
pub fn jmax<T: HasVSimdType + Copy>(a: SIMDRegister<T>, b: SIMDRegister<T>) -> SIMDRegister<T>
where
    T: PartialOrd, // Allow types that can be partially ordered
{
    SIMDRegister::max(a, b) // Assumes a class method `max` on SIMDRegister
}
