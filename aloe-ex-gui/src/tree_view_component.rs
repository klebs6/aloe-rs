crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TreeViewComponent<'a> {
    base: Component<'a>,
    tree: TreeView<'a>,
    root: TreeViewComponentRootItem<'a>,
}

impl<'a> Default for TreeViewComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            tree.setRootItem (&root);
                tree.setRootItemVisible (false);

                addAndMakeVisible (tree)
        */
    }
}

impl<'a> Resized for TreeViewComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            tree.setBounds (getLocalBounds());
        */
    }
}
