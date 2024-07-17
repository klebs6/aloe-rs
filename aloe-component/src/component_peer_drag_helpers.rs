crate::ix!();

pub fn is_file_drag(info: &ComponentPeerDragInfo) -> bool {
    
    todo!();
        /*
            return ! info.files.isEmpty();
        */
}

pub fn is_suitable_target<'a>(
    info:   &ComponentPeerDragInfo,
    target: *mut Component<'a>) -> bool {
    
    todo!();
        /*
            return isFileDrag (info) ? dynamic_cast<FileDragAndDropTarget*> (target) != nullptr
                                     : dynamic_cast<TextDragAndDropTarget*> (target) != nullptr;
        */
}

pub fn is_interested<'a>(
    info:   &ComponentPeerDragInfo,
    target: *mut Component<'a>) -> bool {
    
    todo!();
        /*
            return isFileDrag (info) ? dynamic_cast<FileDragAndDropTarget*> (target)->isInterestedInFileDrag (info.files)
                                     : dynamic_cast<TextDragAndDropTarget*> (target)->isInterestedInTextDrag (info.text);
        */
}

pub fn find_drag_and_drop_target<'a>(
    c:        *mut Component<'a>,
    info:     &ComponentPeerDragInfo,
    last_one: *mut Component<'a>) -> *mut Component<'a> {
    
    todo!();
        /*
            for (; c != nullptr; c = c->getParentComponent())
                if (isSuitableTarget (info, c) && (c == lastOne || isInterested (info, c)))
                    return c;

            return nullptr;
        */
}
