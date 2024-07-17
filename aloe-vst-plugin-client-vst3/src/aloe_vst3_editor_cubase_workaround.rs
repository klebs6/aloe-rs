crate::ix!();

/**
  | On macOS Cubase 10 resizes the host
  | window after calling onSize() resulting
  | in the peer bounds being a step behind
  | the plug-in. Calling updateBounds()
  | asynchronously seems to fix things...
  */
#[cfg(target_os="macos")]
pub struct AloeVst3EditorCubase10WindowResizeWorkaround<'a> {
    base:  AsyncUpdater<'a>,
    owner: &'a mut AloeVst3Editor<'a>,
}

impl<'a> AloeVst3EditorCubase10WindowResizeWorkaround<'a> {

    pub fn new(o: &mut AloeVst3Editor) -> Self {
    
        todo!();
        /*
        : owner(o),

        
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (owner.component != nullptr)
                        if (auto* peer = owner.component->getPeer())
                            peer->updateBounds();
        */
    }
}
