crate::ix!();

/// A trait to delineate the contract for memory operations associated with specific data samples.
///
/// Classes implementing this trait are assumed to manage or interact with data samples that may
/// be stored in various memory hierarchies (e.g., cache, RAM, disk). The `touch_sample` method
/// is designed to provoke the loading of a given data sample into a more immediately accessible
/// memory tier, essentially serving as a memory paging mechanism.
///
/// # Examples
///
/// ```
/// struct DataSampler;
///
/// impl TouchSample for DataSampler {
///     fn touch_sample(&self, sample: i64) {
///         // Implement logic to bring `sample` into an active memory tier
///     }
/// }
/// ```
///
pub trait TouchSample {
    /// Invokes a memory operation to actively load the specified data sample into a readily
    /// accessible memory tier.
    ///
    /// The method ensures that the data sample corresponding to the provided index (`sample`) is
    /// brought into a memory level where it can be efficiently accessed, thereby mitigating
    /// latencies induced by memory fetch operations.
    ///
    /// # Parameters
    ///
    /// * `sample`: i64 - An index or identifier corresponding to the data sample to be touched.
    fn touch_sample(&self, sample: i64);
}
