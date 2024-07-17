#[macro_export] macro_rules! generic_float {
    ($trait_name:ident, $fn_name:ident) => {

        pub trait MaxDepth {

            #[inline(always)] fn max_depth() -> Self;
        }

        impl<T: num::Float> MaxDepth for T { 

            #[inline(always)] fn max_depth() -> Self { 
                T::from(0.70710678118654752440).unwrap() 
            }
        }

        #[inline(always)] pub fn max_depth<F: num::Float>() -> F {
            <F as InverseRootTwo>::max_depth()
        }

    }
}

