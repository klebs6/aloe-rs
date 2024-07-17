crate::ix!();

pub struct InParameterChangedCallbackSetter<'a> {
    inner: ScopedValueSetter<'a, bool>,
}

impl<'a> InParameterChangedCallbackSetter<'a> {

    pub fn new(ref_: &mut bool) -> Self {
    
        todo!();
        /*

            : inner ([&]() -> auto& { jassert (! ref); return ref; }(), true, false)
        */
    }
}
