crate::ix!();

pub const PLUGIN_TREE_MENU_ID_BASE: usize = 0x324503f4;

pub fn plugin_tree_build_tree_by_folder(
    tree:        &mut KnownPluginListPluginTree,
    all_plugins: &[PluginDescription])  {
    
    todo!();
    /*
        for (auto& pd : allPlugins)
        {
            auto path = pd.fileOrIdentifier.replaceCharacter ('\\', '/')
                                           .upToLastOccurrenceOf ("/", false, false);

            if (path.substring (1, 2) == ":")
                path = path.substring (2);

            addPlugin (tree, pd, path);
        }

        optimiseFolders (tree, false);
    */
}

pub fn plugin_tree_optimise_folders(
    tree:             &mut KnownPluginListPluginTree,
    concatenate_name: bool

) {
    
    todo!();
    /*
        for (int i = tree.subFolders.size(); --i >= 0;)
        {
            auto& sub = *tree.subFolders.getUnchecked(i);
            optimiseFolders (sub, concatenateName || (tree.subFolders.size() > 1));

            if (sub.plugins.isEmpty())
            {
                for (auto* s : sub.subFolders)
                {
                    if (concatenateName)
                        s->folder = sub.folder + "/" + s->folder;

                    tree.subFolders.add (s);
                }

                sub.subFolders.clear (false);
                tree.subFolders.remove (i);
            }
        }
    */
}

pub fn plugin_tree_build_tree_by_category(
    tree:        &mut KnownPluginListPluginTree,
    sorted:      &[PluginDescription],
    sort_method: KnownPluginListSortMethod

) {
    
    todo!();
    /*
        String lastType;
        auto current = std::make_unique<KnownPluginList::KnownPluginListPluginTree>();

        for (auto& pd : sorted)
        {
            auto thisType = (sortMethod == KnownPluginList::sortByCategory ? pd.category
                                                                           : pd.manufacturerName);

            if (! thisType.containsNonWhitespaceChars())
                thisType = "Other";

            if (! thisType.equalsIgnoreCase (lastType))
            {
                if (current->plugins.size() + current->subFolders.size() > 0)
                {
                    current->folder = lastType;
                    tree.subFolders.add (std::move (current));
                    current = std::make_unique<KnownPluginList::KnownPluginListPluginTree>();
                }

                lastType = thisType;
            }

            current->plugins.add (pd);
        }

        if (current->plugins.size() + current->subFolders.size() > 0)
        {
            current->folder = lastType;
            tree.subFolders.add (std::move (current));
        }
    */
}

pub fn plugin_tree_add_plugin(
    tree: &mut KnownPluginListPluginTree,
    pd:   PluginDescription,
    path: String)  {
    
    todo!();
    /*
        #if ALOE_MAC
        if (path.containsChar (':'))
            path = path.fromFirstOccurrenceOf (":", false, false); // avoid the special AU formatting nonsense on Mac..
       #endif

        if (path.isEmpty())
        {
            tree.plugins.add (pd);
        }
        else
        {
            auto firstSubFolder = path.upToFirstOccurrenceOf ("/", false, false);
            auto remainingPath  = path.fromFirstOccurrenceOf ("/", false, false);

            for (int i = tree.subFolders.size(); --i >= 0;)
            {
                auto& subFolder = *tree.subFolders.getUnchecked (i);

                if (subFolder.folder.equalsIgnoreCase (firstSubFolder))
                {
                    addPlugin (subFolder, pd, remainingPath);
                    return;
                }
            }

            auto* newFolder = new KnownPluginList::KnownPluginListPluginTree();
            newFolder->folder = firstSubFolder;
            tree.subFolders.add (newFolder);
            addPlugin (*newFolder, pd, remainingPath);
        }
    */
}

pub fn plugin_tree_contains_duplicate_names(
    plugins: &[PluginDescription],
    name:    &String) -> bool {
    
    todo!();
    /*
        int matches = 0;

        for (auto& p : plugins)
            if (p.name == name && ++matches > 1)
                return true;

        return false;
    */
}

pub fn plugin_tree_add_to_menu(
    tree:                      &KnownPluginListPluginTree,
    m:                         &mut PopupMenu,
    all_plugins:               &[PluginDescription],
    currently_ticked_pluginid: &String) -> bool {
    
    todo!();
    /*
        bool isTicked = false;

        for (auto* sub : tree.subFolders)
        {
            PopupMenu subMenu;
            auto isItemTicked = addToMenu (*sub, subMenu, allPlugins, currentlyTickedPluginID);
            isTicked = isTicked || isItemTicked;

            m.addSubMenu (sub->folder, subMenu, true, nullptr, isItemTicked, 0);
        }

        auto getPluginMenuIndex = [&] (const PluginDescription& d)
        {
            int i = 0;

            for (auto& p : allPlugins)
            {
                if (p.isDuplicateOf (d))
                    return i + menuIdBase;

                ++i;
            }

            return 0;
        };

        for (auto& plugin : tree.plugins)
        {
            auto name = plugin.name;

            if (containsDuplicateNames (tree.plugins, name))
                name << " (" << plugin.pluginFormatName << ')';

            auto isItemTicked = plugin.matchesIdentifierString (currentlyTickedPluginID);
            isTicked = isTicked || isItemTicked;

            m.addItem (getPluginMenuIndex (plugin), name, true, isItemTicked);
        }

        return isTicked;
    */
}
