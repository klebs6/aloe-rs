crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/containers/aloe_SIMDRegister.h]

/**
  | A wrapper around the platform's native
  | SIMD register type.
  | 
  | This class is only available on SIMD
  | machines. Use
  | 
  | ALOE_USE_SIMD to query if SIMD is available
  | for your system.
  | 
  | SIMDRegister<ElementType> is a templated
  | class representing the native vectorized
  | version of FloatingType. SIMDRegister
  | supports all numerical primitive types
  | and std:complex<float> and std::complex<double>
  | supports and most operations of the
  | corresponding primitive type. Additionally,
  | SIMDRegister can be accessed like an
  | array to extract the individual elements.
  | 
  | If you are using SIMDRegister as a pointer,
  | then you must ensure that the memory
  | is sufficiently aligned for
  | 
  | SIMD vector operations. Failing to
  | do so will result in crashes or very slow
  | code. Use
  | 
  | SIMDRegister::isSIMDAligned to query
  | if a pointer is sufficiently aligned
  | for SIMD vector operations.
  | 
  | -----------
  | @note
  | 
  | using SIMDRegister without enabling
  | optimizations will result in code with
  | very poor performance.
  | 
  | @tags{DSP}
  |
  */
pub struct SIMDRegister<ElementType: HasVSimdType> {
    value: <Self as HasVSimdType>::VSimdType,
    _0:    PhantomData<ElementType>,
}

impl<ElementType> Index<usize> for SIMDRegister<ElementType> {

    type Output = ElementType;
    
    /**
      | Returns the idx-th element of the receiver.
      | Note that this does not check if idx is
      | larger than the native register size.
      |
      */
    fn index(&self, idx: usize) -> &Self::Output {
        todo!();
        /*
            return get (idx);
        */
    }
}

impl<ElementType> IndexMut<usize> for SIMDRegister<ElementType> {
    
    /**
      | Returns the idx-th element of the receiver.
      | Note that this does not check if idx is
      | larger than the native register size.
      |
      */
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        todo!();
        /*
            jassert (idx < SIMDNumElements);
            return ElementAccess (*this, idx);
        */
    }
}

//---------------------------
pub trait HasValueType { type ValueType; }

impl<ElementType: HasVSimdType> HasValueType for SIMDRegister<ElementType> {

    /**
      | STL compatible value_type definition
      | (same as ElementType).
      |
      */
    type ValueType = ElementType;
}

//---------------------------
pub trait HasMaskType { type MaskType; }

impl<ElementType: HasMaskType + HasVSimdType> HasMaskType for SIMDRegister<ElementType> {

    /**
      | The corresponding primitive integer
      | type, for example, this will be int32_t
      | if type is a float.
      |
      */
    type MaskType = <ElementType as HasMaskType>::MaskType;
}

//---------------------------
pub trait HasPrimitiveType { type PrimitiveType; }

impl<ElementType: HasPrimitiveType + HasVSimdType> HasPrimitiveType for SIMDRegister<ElementType> {

    /**
      | The native primitive type (used internally).
      |
      */
    type PrimitiveType = <ElementType as HasPrimitiveType>::PrimitiveType;
}

//---------------------------
pub trait HasNativeOps { type NativeOps; }

/*
impl<ElementType, T> HasNativeOps for SIMDRegister<ElementType> 
where T: SIMDNativeOps<Scalar = <ElementType as HasPrimitiveType>::PrimitiveType, VSimdType = <ElementType as HasVSimdType>::VSimdType>,
      ElementType: HasPrimitiveType + HasVSimdType
{

    /**
      | The native operations for this platform
      | and type combination (used internally)
      |
      */
    type NativeOps = T;
}
*/

//---------------------------
pub trait HasVMaskType { type vMaskType; }

impl<ElementType: HasVSimdType> HasVMaskType for SIMDRegister<ElementType> 
where SIMDRegister<ElementType>: HasMaskType
{

    /**
      | The corresponding integer SIMDRegister
      | type (used internally).
      |
      */
    type vMaskType = SIMDRegister<<Self as HasMaskType>::MaskType>;
}

//---------------------------
pub trait HasVMaskSimdType { type VMaskSimdType; }

impl<ElementType: HasVSimdType> HasVMaskSimdType for SIMDRegister<ElementType> {

    /**
      | The internal native type for the corresponding
      | mask type (used internally).
      |
      */
    type VMaskSimdType = <Self as HasVSimdType>::VSimdType;
}

//---------------------------
pub trait HasComplexOps { type CmplxOps; }

impl<ElementType: HasComplexOps + HasVSimdType> HasComplexOps for SIMDRegister<ElementType> {

    /**
      | Wrapper for operations which need to
      | be handled differently for complex
      | and scalar types (used internally).
      |
      */
    type CmplxOps = CmplxSIMDOps<ElementType, <ElementType as HasComplexOps>::CmplxOps>;
}

//---------------------------
impl<ElementType: HasVSimdType> SIMDRegister<ElementType> {

    /**
      | The size in bytes of this register.
      |
      */
    const SIMD_REGISTER_SIZE: usize = size_of::<<Self as HasVSimdType>::VSimdType>();

    /**
      | The number of elements that this vector
      | can hold.
      |
      */
    const SIMD_NUM_ELEMENTS: usize = Self::SIMD_REGISTER_SIZE / size_of::<ElementType>();

    /**
      | Constructs an object from the native
      | SIMD type.
      |
      */
    pub fn new(a: <Self as HasVSimdType>::VSimdType) -> Self {
    
        todo!();
        /*
        : value(a),

        
        */
    }

    /**
      | Constructs an object from a scalar type
      | by broadcasting it to all elements.
      |
      */
    pub fn new_from_element_type(s: ElementType) -> Self {
    
        todo!();
        /*


            *this = s;
        */
    }

    /** Returns the number of elements in this vector. */
    pub fn size() -> usize {
        
        todo!();
        /*
            return SIMDNumElements;
        */
    }

    /**
      | Creates a new SIMDRegister from the
      | corresponding scalar primitive.
      | 
      | The scalar is extended to all elements
      | of the vector.
      |
      */
    pub fn expand(s: ElementType) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return {CmplxExpand::expand (s)};
        */
    }

    /**
      | Creates a new SIMDRegister from the
      | internal SIMD type (for example __mm128
      | for single-precision floating point
      | on SSE architectures).
      |
      */
    pub fn from_native(a: <Self as HasVSimdType>::VSimdType) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return {a};
        */
    }

    /**
      | Creates a new SIMDRegister from the
      | first SIMDNumElements of a scalar array.
      |
      */
    pub fn from_raw_array(a: *const ElementType) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            jassert (isSIMDAligned (a));
            return {CmplxExpand::load (a)};
        */
    }

    /**
      | Copies the elements of the SIMDRegister
      | to a scalar array in memory.
      |
      */
    #[inline] pub fn copy_to_raw_array(&self, a: *mut ElementType)  {
        
        todo!();
        /*
            jassert (isSIMDAligned (a));
            CmplxExpand::store (value, a);
        */
    }

    /**
      | Returns the idx-th element of the receiver.
      | Note that this does not check if idx is
      | larger than the native register size.
      |
      */
    #[inline] pub fn get(&self, idx: usize) -> ElementType {
        
        todo!();
        /*
            jassert (idx < SIMDNumElements);
            return CmplxExpand::get (value, idx);
        */
    }

    /**
      | Sets the idx-th element of the receiver.
      | Note that this does not check if idx is
      | larger than the native register size.
      |
      */
    #[inline] pub fn set(&mut self, 
        idx: usize,
        v:   ElementType)  {
        
        todo!();
        /*
            jassert (idx < SIMDNumElements);
            value = CmplxExpand::set (value, idx, v);
        */
    }

    /**
      | Returns a SIMDRegister of the corresponding
      | integral type where each element has
      | each bit set if the corresponding element
      | of a is equal to the corresponding element
      | of b, or zero otherwise.
      | 
      | The result can then be used in bit operations
      | defined above to avoid branches in vector
      | SIMD code.
      |
      */
    pub fn equal(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            return toMaskType (NativeOps::equal (a.value, b.value));
        */
    }

    /**
      | Returns a SIMDRegister of the corresponding
      | integral type where each element has
      | each bit set if the corresponding element
      | of a is not equal to the corresponding
      | element of b, or zero otherwise.
      | 
      | The result can then be used in bit operations
      | defined above to avoid branches in vector
      | SIMD code.
      |
      */
    pub fn not_equal(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            return toMaskType (NativeOps::notEqual (a.value, b.value));
        */
    }

    /**
      | Returns a SIMDRegister of the corresponding
      | integral type where each element has
      | each bit set if the corresponding element
      | of a is less than to the corresponding
      | element of b, or zero otherwise.
      | 
      | The result can then be used in bit operations
      | defined above to avoid branches in vector
      | SIMD code.
      |
      */
    pub fn less_than(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            return toMaskType (NativeOps::greaterThan (b.value, a.value));
        */
    }

    /**
      | Returns a SIMDRegister of the corresponding
      | integral type where each element has
      | each bit set if the corresponding element
      | of a is than or equal to the corresponding
      | element of b, or zero otherwise.
      | 
      | The result can then be used in bit operations
      | defined above to avoid branches in vector
      | SIMD code.
      |
      */
    pub fn less_than_or_equal(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            return toMaskType (NativeOps::greaterThanOrEqual (b.value, a.value));
        */
    }

    /**
      | Returns a SIMDRegister of the corresponding
      | integral type where each element has
      | each bit set if the corresponding element
      | of a is greater than to the corresponding
      | element of b, or zero otherwise.
      | 
      | The result can then be used in bit operations
      | defined above to avoid branches in vector
      | SIMD code.
      |
      */
    pub fn greater_than(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            return toMaskType (NativeOps::greaterThan (a.value, b.value));
        */
    }

    /**
      | Returns a SIMDRegister of the corresponding
      | integral type where each element has
      | each bit set if the corresponding element
      | of a is greater than or equal to the corresponding
      | element of b, or zero otherwise.
      | 
      | The result can then be used in bit operations
      | defined above to avoid branches in vector
      | SIMD code.
      |
      */
    pub fn greater_than_or_equal(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            return toMaskType (NativeOps::greaterThanOrEqual (a.value, b.value));
        */
    }

    /**
      | Returns a new vector where each element
      | is the minimum of the corresponding
      | element of a and b.
      |
      */
    pub fn min(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return { NativeOps::min (a.value, b.value) };
        */
    }

    /**
      | Returns a new vector where each element
      | is the maximum of the corresponding
      | element of a and b.
      |
      */
    pub fn max(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return { NativeOps::max (a.value, b.value) };
        */
    }

    /**
      | Multiplies b and c and adds the result
      | to a.
      |
      */
    pub fn multiply_add(
        a: SIMDRegister<ElementType>,
        b: SIMDRegister<ElementType>,
        c: SIMDRegister<ElementType>) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return { CmplxExpand::muladd (a.value, b.value, c.value) };
        */
    }

    /**
      | Returns a scalar which is the sum of all
      | elements of the receiver.
      |
      */
    #[inline] pub fn sum(&self) -> ElementType {
        
        todo!();
        /*
            return CmplxExpand::sum (value);
        */
    }

    /**
      | Truncates each element to its integer
      | part.
      | 
      | Effectively discards the fractional
      | part of each element. A.k.a. round to
      | zero.
      |
      */
    pub fn truncate(a: SIMDRegister<ElementType>) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return { NativeOps::truncate (a.value) };
        */
    }

    /**
      | Returns the absolute value of each element.
      |
      */
    pub fn abs(a: SIMDRegister<ElementType>) -> SIMDRegister<ElementType> {
        
        todo!();
        /*
            return a - (a * (expand (ElementType (2)) & lessThan (a, expand (ElementType (0)))));
        */
    }

    /**
      | Checks if the given pointer is sufficiently
      | aligned for using SIMD operations.
      |
      */
    pub fn is_simd_aligned(ptr: *const ElementType) -> bool {
        
        todo!();
        /*
            uintptr_t bitmask = SIMD_REGISTER_SIZE - 1;
            return (reinterpret_cast<uintptr_t> (ptr) & bitmask) == 0;
        */
    }

    /**
      | Returns the next position in memory
      | where isSIMDAligned returns true.
      | 
      | If the current position in memory is
      | already aligned then this method will
      | simply return the pointer.
      |
      */
    pub fn get_next_simd_aligned_ptr(ptr: *mut ElementType) -> *mut ElementType {
        
        todo!();
        /*
            return snapPointerToAlignment (ptr, SIMD_REGISTER_SIZE);
        */
    }
    
    pub fn to_mask_type(a: <Self as HasVSimdType>::VSimdType) -> <Self as HasVMaskType>::vMaskType {
        
        todo!();
        /*
            union
            {
                vSIMDType in;
                vMaskSIMDType out;
            } u;

            u.in = a;
            return vMaskType::fromNative (u.out);
        */
    }
    
    pub fn to_vec_type_from_vmask(a: <Self as HasVMaskSimdType>::VMaskSimdType) 
        -> <Self as HasVSimdType>::VSimdType 
    {
        todo!();
        /*
            union
            {
                vMaskSIMDType in;
                vSIMDType out;
            } u;

            u.in = a;
            return u.out;
        */
    }
    
    pub fn to_vec_type(a: <Self as HasMaskType>::MaskType) 
        -> <Self as HasVSimdType>::VSimdType 
    {
        todo!();
        /*
            union
            {
                vMaskSIMDType in;
                vSIMDType out;
            } u;

            u.in = CmplxSIMDOps<MaskType>::expand (a);
            return u.out;
        */
    }
}

/*
impl<T, N> AddAssign for SIMDRegister<T>
where
    T: Add<Output = T>,
    N: NativeOps<T>,
{
    fn add_assign(&mut self, other: Self) {
        self.value = N::add(self.value, other.value);
    }
}

impl<T, N> SubAssign for SIMDRegister<T>
where
    T: Sub<Output = T>,
    N: NativeOps<T>,
{
    fn sub_assign(&mut self, other: Self) {
        self.value = N::sub(self.value, other.value);
    }
}

impl<T, C> MulAssign for SIMDRegister<T>
where
    T: Mul<Output = T>,
    C: CmplxMul<T>,
{
    fn mul_assign(&mut self, other: Self) {
        self.value = C::mul(self.value, other.value);
    }
}

impl<T, S, N, C> AddAssign<S> for SIMDRegister<T>
where
    T: Add<Output = T>,
    S: Clone,
    N: NativeOps<T>,
    C: CmplxExpand<T, S>,
{
    fn add_assign(&mut self, scalar: S) {
        self.value = N::add(self.value, C::expand(scalar.clone()));
    }
}

impl<T, S, N, C> SubAssign<S> for SIMDRegister<T>
where
    T: Sub<Output = T>,
    S: Clone,
    N: NativeOps<T>,
    C: CmplxExpand<T, S>,
{
    fn sub_assign(&mut self, scalar: S) {
        self.value = N::sub(self.value, C::expand(scalar.clone()));
    }
}

impl<T, S, C> MulAssign<S> for SIMDRegister<T>
where
    T: Mul<Output = T>,
    S: Clone,
    C: CmplxExpand<T, S>,
{
    fn mul_assign(&mut self, scalar: S) {
        self.value = C::mul(self.value, C::expand(scalar.clone()));
    }
}

impl<T, M, N> BitAndAssign<M> for SIMDRegister<T>
where
    T: BitAnd<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn bitand_assign(&mut self, mask: M) {
        self.value = N::bit_and(self.value, N::to_vec_type(mask.clone()));
    }
}

impl<T, M, N> BitOrAssign<M> for SIMDRegister<T>
where
    T: BitOr<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn bitor_assign(&mut self, mask: M) {
        self.value = N::bit_or(self.value, N::to_vec_type(mask.clone()));
    }
}

impl<T, M, N> BitXorAssign<M> for SIMDRegister<T>
where
    T: BitXor<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn bitxor_assign(&mut self, mask: M) {
        self.value = N::bit_xor(self.value, N::to_vec_type(mask.clone()));
    }
}

impl<T, S, M, N, C> SIMDRegister<T>
where
    T: Clone,
    S: Clone,
    M: Clone,
    N: NativeOps<T, M>,
    C: CmplxExpand<T, S>,
{
    /// Broadcasts a scalar to all elements of the SIMD register.
    pub fn broadcast(&mut self, scalar: S) {
        self.value = C::expand(scalar.clone());
    }
}

impl<T, M, N> BitAndAssign<M> for SIMDRegister<T>
where
    T: BitAnd<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn bitand_assign(&mut self, scalar_mask: M) {
        self.value = N::bit_and(self.value, N::to_vec_type_scalar(scalar_mask.clone()));
    }
}

impl<T, M, N> BitOrAssign<M> for SIMDRegister<T>
where
    T: BitOr<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn bitor_assign(&mut self, scalar_mask: M) {
        self.value = N::bit_or(self.value, N::to_vec_type_scalar(scalar_mask.clone()));
    }
}

impl<T, M, N> BitXorAssign<M> for SIMDRegister<T>
where
    T: BitXor<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn bitxor_assign(&mut self, scalar_mask: M) {
        self.value = N::bit_xor(self.value, N::to_vec_type_scalar(scalar_mask.clone()));
    }
}

impl<T, N> Add for SIMDRegister<T>
where
    T: Add<Output = T>,
    N: NativeOps<T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: N::add(self.value, other.value),
        }
    }
}

impl<T, N> Sub for SIMDRegister<T>
where
    T: Sub<Output = T>,
    N: NativeOps<T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: N::sub(self.value, other.value),
        }
    }
}

impl<T, C> Mul for SIMDRegister<T>
where
    T: Mul<Output = T>,
    C: CmplxMul<T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: C::mul(self.value, other.value),
        }
    }
}

impl<T, S, N, C> Add<S> for SIMDRegister<T>
where
    T: Add<Output = T>,
    S: Clone,
    N: NativeOps<T>,
    C: CmplxExpand<T, S>,
{
    type Output = Self;

    fn add(self, scalar: S) -> Self::Output {
        Self {
            value: N::add(self.value, C::expand(scalar.clone())),
        }
    }
}

impl<T, S, N, C> Sub<S> for SIMDRegister<T>
where
    T: Sub<Output = T>,
    S: Clone,
    N: NativeOps<T>,
    C: CmplxExpand<T, S>,
{
    type Output = Self;

    fn sub(self, scalar: S) -> Self::Output {
        Self {
            value: N::sub(self.value, C::expand(scalar.clone())),
        }
    }
}

impl<T, S, C> Mul<S> for SIMDRegister<T>
where
    T: Mul<Output = T>,
    S: Clone,
    C: CmplxExpand<T, S>,
{
    type Output = Self;

    fn mul(self, scalar: S) -> Self::Output {
        Self {
            value: C::mul(self.value, C::expand(scalar.clone())),
        }
    }
}

impl<T, M, N> BitAnd for SIMDRegister<T>
where
    T: BitAnd<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    type Output = Self;

    fn bitand(self, mask: M) -> Self::Output {
        Self {
            value: N::bit_and(self.value, N::to_vec_type(mask.clone())),
        }
    }
}

impl<T, M, N> BitOr for SIMDRegister<T>
where
    T: BitOr<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    type Output = Self;

    fn bitor(self, mask: M) -> Self::Output {
        Self {
            value: N::bit_or(self.value, N::to_vec_type(mask.clone())),
        }
    }
}

impl<T, M, N> BitXor for SIMDRegister<T>
where
    T: BitXor<Output = T>,
    M: Clone,
    N: NativeOps<T, M>,
{
    type Output = Self;

    fn bitxor(self, mask: M) -> Self::Output {
        Self {
            value: N::bit_xor(self.value, N::to_vec_type(mask.clone())),
        }
    }
}

impl<T, S, M, N, C> BitAnd<S> for SIMDRegister<T>
where
    T: BitAnd<Output = T>,
    S: Clone,
    M: Clone,
    N: NativeOps<T, M>,
    C: CmplxExpand<T, S>,
{
    type Output = Self;

    fn bitand(self, scalar_mask: S) -> Self::Output {
        Self {
            value: N::bit_and_scalar(self.value, C::expand(scalar_mask.clone())),
        }
    }
}

impl<T, S, M, N, C> BitOr<S> for SIMDRegister<T>
where
    T: BitOr<Output = T>,
    S: Clone,
    M: Clone,
    N: NativeOps<T, M>,
    C: CmplxExpand<T, S>,
{
    type Output = Self;

    fn bitor(self, scalar_mask: S) -> Self::Output {
        Self {
            value: N::bit_or_scalar(self.value, C::expand(scalar_mask.clone())),
        }
    }
}

impl<T, S, M, N, C> BitXor<S> for SIMDRegister<T>
where
    T: BitXor<Output = T>,
    S: Clone,
    M: Clone,
    N: NativeOps<T, M>,
    C: CmplxExpand<T, S>,
{
    type Output = Self;

    fn bitxor(self, scalar_mask: S) -> Self::Output {
        Self {
            value: N::bit_xor_scalar(self.value, C::expand(scalar_mask.clone())),
        }
    }
}

impl<T, M, N> PartialEq for SIMDRegister<T>
where
    T: PartialEq,
    M: Clone,
    N: NativeOps<T, M>,
{
    fn eq(&self, other: &Self) -> bool {
        N::all_equal(self.value, other.value)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T, S, M, N, C> PartialEq<S> for SIMDRegister<T>
where
    T: PartialEq + Clone,
    S: Clone,
    M: Clone,
    N: NativeOps<T, M>,
    C: CmplxExpand<T, S>,
{
    fn eq(&self, scalar: &S) -> bool {
        let other = SIMDRegister::expand(scalar.clone());
        self.eq(&other)
    }

    fn ne(&self, scalar: &S) -> bool {
        !self.eq(scalar)
    }
}
*/
