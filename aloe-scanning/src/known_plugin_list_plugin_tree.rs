crate::ix!();

/**
  | A structure that recursively holds
  | a tree of plugins. @see KnownPluginList::createTree()
  |
  */
pub struct KnownPluginListPluginTree
{

    /**
      | The name of this folder in the tree
      |
      */
    folder:      String,

    sub_folders: Vec<Box<KnownPluginListPluginTree>>,
    plugins:     Vec<PluginDescription>,
}
