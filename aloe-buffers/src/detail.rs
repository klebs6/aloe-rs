crate::ix!();

/** 
  | The contents of this namespace are used to
  | implement AudioBuffer and should
  | not be used elsewhere. Their interfaces (and
  | existence) are liable to change!
  */
pub(crate) mod sample_buffer_detail
{
    use super::*;

    /** 
      | On iOS/arm7 the alignment of `double` is
      | greater than the alignment of
      | `std::max_align_t`, so we can't trust
      | max_align_t. Instead, we query lots of
      | primitive types and use the maximum
      | alignment of all of them.
      |
      | We're putting this stuff outside
      | AudioBuffer itself to avoid creating
      | unnecessary copies for each distinct
      | template instantiation of AudioBuffer.
      |
      | MSVC 2015 doesn't like when we write
      | getMaxAlignment as a loop which
      | accumulates the max alignment
      | (declarations not allowed in constexpr
      | function body) so instead we use this
      | recursive version which instantiates
      | a zillion templates.
      */
    pub struct Type<T> {
        p: PhantomData<T>,
    }

    pub fn get_max_alignment_recursion_base() -> usize {
        
        todo!();
        /*
            return 0;
        */
    }

    pub fn get_max_alignment<Head, Tail>(
            _0:   Type<Head>,
            tail: Type<Tail>) -> usize {

        todo!();
        /*
            return jmax (alignof (Head), getMaxAlignment (tail...));
        */
    }

    lazy_static!{
        /*
        constexpr size_t maxAlignment = getMaxAlignment (Type<std::max_align_t>{},
                                                             Type<void*>{},
                                                             Type<float>{},
                                                             Type<double>{},
                                                             Type<long double>{},
                                                             Type<short int>{},
                                                             Type<int>{},
                                                             Type<long int>{},
                                                             Type<long long int>{},
                                                             Type<bool>{},
                                                             Type<char>{},
                                                             Type<char16_t>{},
                                                             Type<char32_t>{},
                                                             Type<wchar_t>{});
        */
    }
}
