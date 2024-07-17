crate::ix!();

pub type FloatCacheFlagType = u32;

pub const FLOAT_CACHE_NUM_FLAG_BITS: usize = 8 * size_of::<FloatCacheFlagType>();

#[derive(Default)]
pub struct FloatCache {
    values: Vec<Atomic<f32>>,
    flags:  Vec<Atomic<FloatCacheFlagType>>,
}

impl FloatCache {

    pub fn new(size_in: usize) -> Self {
    
        todo!();
        /*
        : values(sizeIn),
        : flags(divCeil (sizeIn, numFlagBits)),

            std::fill (values.begin(), values.end(), 0.0f);
            std::fill (flags.begin(), flags.end(), 0);
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return values.size();
        */
    }
    
    pub fn set_without_notifying(&mut self, 
        index: usize,
        value: f32)  {
        
        todo!();
        /*
            jassert (index < size());
            values[index].store (value, std::memory_order_relaxed);
        */
    }
    
    pub fn set(&mut self, 
        index: usize,
        value: f32)  {
        
        todo!();
        /*
            jassert (index < size());
            const auto previous = values[index].exchange (value, std::memory_order_relaxed);
            const auto bit = previous == value ? ((FloatCacheFlagType) 0) : ((FloatCacheFlagType) 1 << (index % numFlagBits));
            flags[index / numFlagBits].fetch_or (bit, std::memory_order_acq_rel);
        */
    }
    
    pub fn get(&self, index: usize) -> f32 {
        
        todo!();
        /*
            jassert (index < size());
            return values[index].load (std::memory_order_relaxed);
        */
    }

    /**
      | Calls the supplied callback for any
      | entries which have been modified since
      | the last call to this function.
      |
      */
    pub fn if_set<Callback>(&mut self, callback: Callback)  {
    
        todo!();
        /*
            for (size_t flagIndex = 0; flagIndex < flags.size(); ++flagIndex)
            {
                const auto prevFlags = flags[flagIndex].exchange (0, std::memory_order_acq_rel);

                for (size_t bit = 0; bit < numFlagBits; ++bit)
                {
                    if (prevFlags & ((FloatCacheFlagType) 1 << bit))
                    {
                        const auto itemIndex = (flagIndex * numFlagBits) + bit;
                        callback (itemIndex, values[itemIndex].load (std::memory_order_relaxed));
                    }
                }
            }
        */
    }
    
    pub fn div_ceil(
        a: usize,
        b: usize) -> usize {
        
        todo!();
        /*
            return (a / b) + ((a % b) != 0);
        */
    }
}
