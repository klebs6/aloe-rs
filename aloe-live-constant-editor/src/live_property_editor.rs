crate::ix!();

pub struct LivePropertyEditor<'a, Type> {
    base: LivePropertyEditorBase<'a>,
    _p0:  PhantomData<Type>,
}

impl<'a, Type> LivePropertyEditor<'a, Type> {

    pub fn new<ValueType>(
        v: &mut ValueType,
        d: &mut CodeDocument) -> Self {
    
        todo!();
        /*
        : live_property_editor_base(v, d),

            customComp.reset (CustomEditor<Type>::create (*this));
                addAndMakeVisible (customComp.get());
        */
    }
}
