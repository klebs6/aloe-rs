crate::ix!();
    
pub struct ValueList<'a> {
    base:           AsyncUpdater<'a>,
    base2:          DeletedAtShutdown,
    values:         Vec<Box<LiveValueBase>>,
    documents:      Vec<Box<CodeDocument<'a>>>,
    document_files: Vec<File>,
    editor_window:  ComponentSafePointer<'a, ValueListEditorWindow<'a>>,
    lock:           CriticalSection,
}

aloe_declare_singleton!{
    ValueList, false
}

aloe_implement_singleton!{
    ValueList
}

impl<'a> Drop for ValueList<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

impl<'a> ValueList<'a> {
    
    pub fn get_value<Type>(&mut self, 
        file:          *const u8,
        line:          i32,
        initial_value: &Type) -> &mut LiveValue<Type> {
    
        todo!();
        /*
            const ScopedLock sl (lock);
                using ValueType = LiveValue<Type>;

                for (auto* v : values)
                    if (v->sourceLine == line && v->sourceFile == file)
                        return *static_cast<ValueType*> (v);

                auto v = new ValueType (file, line, initialValue);
                addValue (v);
                return *v;
        */
    }
    
    pub fn add_value(&mut self, v: *mut LiveValueBase)  {
        
        todo!();
        /*
            values.add (v);
        triggerAsyncUpdate();
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (editorWindow == nullptr)
            editorWindow = new ValueListEditorWindow (*this);

        editorWindow->updateItems (*this);
        */
    }
    
    pub fn get_document(&mut self, file: &File) -> &mut CodeDocument {
        
        todo!();
        /*
            const int index = documentFiles.indexOf (file.getFullPathName());

        if (index >= 0)
            return *documents.getUnchecked (index);

        auto* doc = documents.add (new CodeDocument());
        documentFiles.add (file);
        doc->replaceAllContent (file.loadFileAsString());
        doc->clearUndoHistory();
        return *doc;
        */
    }
}
