crate::ix!();

pub struct XValueHolder<XValueType> {
    value:        XValueType,
    cleanup_func: fn(_0: &mut XValueType) -> (),
}

impl<XValueType> Drop for XValueHolder<XValueType> {

    fn drop(&mut self) {
        todo!();
        /*
            cleanupFunc (value);
        */
    }
}

impl<XValueType> XValueHolder<XValueType> {

    pub fn new(
        xv:      XValueType,
        cleanup: &fn(_0: &mut XValueType) -> ()) -> Self {
    
        todo!();
        /*


            : value (std::move (xv)), cleanupFunc (cleanup)
        */
    }
}
