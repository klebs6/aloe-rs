crate::ix!();

pub struct MultiTouchMapperTouchInfo<'a,IDType> {
    touch_id: IDType,
    owner:    *mut ComponentPeer<'a>,
}

impl<'a,IDType> Default for MultiTouchMapperTouchInfo<'a,IDType> {
    
    fn default() -> Self {
        todo!();
    }
}

impl<'a,IDType> PartialEq<MultiTouchMapperTouchInfo<'a,IDType>> for MultiTouchMapperTouchInfo<'a,IDType> {
    
    #[inline] fn eq(&self, other: &MultiTouchMapperTouchInfo<'a,IDType>) -> bool {
        todo!();
        /*
            return (touchId == o.touchId);
        */

    }
}

impl<'a,IDType> Eq for MultiTouchMapperTouchInfo<'a,IDType> {}

impl<'a,IDType> MultiTouchMapperTouchInfo<'a,IDType> {

    pub fn new(
        id_to_use: IDType,
        peer:      *mut ComponentPeer<'a>

    ) -> Self {

        todo!();
    }
}
