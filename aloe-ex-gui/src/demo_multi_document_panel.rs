crate::ix!();

/**
  | Simple MultiDocumentPanel that just
  | tries to save our notes when they are
  | closed.
  |
  */
#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct DemoMultiDocumentPanel<'a> {
    base: MultiDocumentPanel<'a>,
}

impl<'a> DemoMultiDocumentPanel<'a> {
    
    pub fn try_to_close_document_async(&mut self, 
        component: *mut Component,
        callback:  fn(_0: bool) -> ())  {
        
        todo!();
        /*
            if (auto* note = dynamic_cast<Note*> (component))
            {
                SafePointer<DemoMultiDocumentPanel> parent { this };
                note->saveIfNeededAndUserAgreesAsync ([parent, callback] (FileBasedDocument::SaveResult result)
                {
                    if (parent != nullptr)
                        callback (result == FileBasedDocument::savedOk);
                });
            }
        */
    }
}
