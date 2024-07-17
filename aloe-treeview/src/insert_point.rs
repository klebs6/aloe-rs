crate::ix!();

pub struct TreeViewInsertPoint<'a> {
    pos:          Point<i32>,
    item:         *mut TreeViewItem<'a>,
    insert_index: i32, // default = 0
}

impl<'a> TreeViewInsertPoint<'a> {

    pub fn new(
        view:                &mut TreeView,
        files:               &Vec<String>,
        drag_source_details: &DragAndDropTargetSourceDetails) -> Self {
    
        todo!();
        /*


            : pos (dragSourceDetails.localPosition),
              item (view.getItemAt (dragSourceDetails.localPosition.y))

            if (item != nullptr)
            {
                auto itemPos = item->getItemPosition (true);
                insertIndex = item->getIndexInParent();
                auto oldY = pos.y;
                pos.y = itemPos.getY();

                if (item->getNumSubItems() == 0 || ! item->isOpen())
                {
                    if (files.size() > 0 ? item->isInterestedInFileDrag (files)
                                         : item->isInterestedInDragSource (dragSourceDetails))
                    {
                        // Check if we're trying to drag into an empty group item..
                        if (oldY > itemPos.getY() + itemPos.getHeight() / 4
                             && oldY < itemPos.getBottom() - itemPos.getHeight() / 4)
                        {
                            insertIndex = 0;
                            pos.x = itemPos.getX() + view.getIndentSize();
                            pos.y = itemPos.getBottom();
                            return;
                        }
                    }
                }

                if (oldY > itemPos.getCentreY())
                {
                    pos.y += item->getItemHeight();

                    while (item->isLastOfSiblings() && item->getParentItem() != nullptr
                            && item->getParentItem()->getParentItem() != nullptr)
                    {
                        if (pos.x > itemPos.getX())
                            break;

                        item = item->getParentItem();
                        itemPos = item->getItemPosition (true);
                        insertIndex = item->getIndexInParent();
                    }

                    ++insertIndex;
                }

                pos.x = itemPos.getX();
                item = item->getParentItem();
            }
            else if (auto* root = view.getRootItem())
            {
                // If they're dragging beyond the bottom of the list, then insert at the end of the root item..
                item = root;
                insertIndex = root->getNumSubItems();
                pos = root->getItemPosition (true).getBottomLeft();
                pos.x += view.getIndentSize();
            }
        */
    }
}
