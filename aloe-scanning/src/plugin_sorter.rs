crate::ix!();

pub struct PluginSorter {
    method:    KnownPluginListSortMethod,
    direction: i32,
}

impl PluginSorter {

    pub fn new(
        sort_method: KnownPluginListSortMethod,
        forwards:    bool) -> Self {
    
        todo!();
        /*
            : method (sortMethod), direction (forwards ? 1 : -1)
        */
    }
    
    pub fn invoke(&self, 
        first:  &PluginDescription,
        second: &PluginDescription) -> bool {
        
        todo!();
        /*
            int diff = 0;

            switch (method)
            {
                case KnownPluginList::sortByCategory:           diff = first.category.compareNatural (second.category, false); break;
                case KnownPluginList::sortByManufacturer:       diff = first.manufacturerName.compareNatural (second.manufacturerName, false); break;
                case KnownPluginList::sortByFormat:             diff = first.pluginFormatName.compare (second.pluginFormatName); break;
                case KnownPluginList::sortByFileSystemLocation: diff = lastPathPart (first.fileOrIdentifier).compare (lastPathPart (second.fileOrIdentifier)); break;
                case KnownPluginList::sortByInfoUpdateTime:     diff = compare (first.lastInfoUpdateTime, second.lastInfoUpdateTime); break;
                case KnownPluginList::sortAlphabetically:
                case KnownPluginList::defaultOrder:
                default: break;
            }

            if (diff == 0)
                diff = first.name.compareNatural (second.name, false);

            return diff * direction < 0;
        */
    }
    
    pub fn last_path_part(path: &String) -> String {
        
        todo!();
        /*
            return path.replaceCharacter ('\\', '/').upToLastOccurrenceOf ("/", false, false);
        */
    }
    
    pub fn compare(a: Time, b: Time) -> i32 {
        
        todo!();
        /*
            if (a < b)   return -1;
            if (b < a)   return 1;

            return 0;
        */
    }
}
