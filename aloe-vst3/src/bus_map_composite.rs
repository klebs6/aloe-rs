crate::ix!();

pub struct Vst3FloatAndDoubleBusMapCompositeHelper<FloatType: num::Float> { 
    _p0: PhantomData<FloatType>,
}

pub struct Vst3FloatAndDoubleBusMapComposite {
    float_version:  Vst3BufferExchangeBusMap<f32>,
    double_version: Vst3BufferExchangeBusMap<f64>,
}

impl Vst3FloatAndDoubleBusMapComposite {
    
    #[inline] pub fn get<FloatType>(&mut self) -> &mut Vst3BufferExchangeBusMap<FloatType> {
    
        todo!();
        /*
            return Vst3FloatAndDoubleBusMapCompositeHelper<FloatType>::get (*this);
        */
    }
}

lazy_static!{
    /*
    template <> struct Vst3FloatAndDoubleBusMapCompositeHelper<float>
    {
        static Vst3BufferExchange<float>::BusMap& get (Vst3FloatAndDoubleBusMapComposite& impl)  { return impl.floatVersion; }
    };

    template <> struct Vst3FloatAndDoubleBusMapCompositeHelper<double>
    {
        static Vst3BufferExchange<double>::BusMap& get (Vst3FloatAndDoubleBusMapComposite& impl) { return impl.doubleVersion; }
    };
    */
}
