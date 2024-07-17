crate::ix!();

pub struct GenericVariantConverter<Wrapped> {
    _0:    PhantomData<Wrapped>,
}

impl<Wrapped> GenericVariantConverter<Wrapped> {

    pub fn from_var(v: &Var) -> Wrapped {
        
        todo!();
        /*
            auto cast = dynamic_cast<ReferenceCountingAdapter<Wrapped>*> (v.getObject());
            jassert (cast != nullptr);
            return cast->get();
        */
    }
    
    pub fn to_var(range: Wrapped) -> Var {
        
        todo!();
        /*
            return { make_reference_counted<Wrapped> (std::move (range)).release() };
        */
    }
}

lazy_static!{
    /*
    template <typename Numeric>
    struct VariantConverter<Range<Numeric>>  : GenericVariantConverter<Range<Numeric>> {};

    template<>
    struct VariantConverter<MPEZoneLayout>  : GenericVariantConverter<MPEZoneLayout> {};

    template<>
    struct VariantConverter<std::shared_ptr<AudioFormatReaderFactory>>
        : GenericVariantConverter<std::shared_ptr<AudioFormatReaderFactory>>
    {};
    */
}
