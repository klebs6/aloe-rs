crate::ix!();

pub fn cate_chunk_is_valid_tag(d: *const u8) -> bool {
    
    todo!();
        /*
            return CharacterFunctions::isLetterOrDigit (d[0]) && CharacterFunctions::isUpperCase (static_cast<aloe_wchar> (d[0]))
                    && CharacterFunctions::isLetterOrDigit (d[1]) && CharacterFunctions::isLowerCase (static_cast<aloe_wchar> (d[1]))
                    && CharacterFunctions::isLetterOrDigit (d[2]) && CharacterFunctions::isLowerCase (static_cast<aloe_wchar> (d[2]));
        */
}

pub fn cate_chunk_is_apple_genre(tag: &String) -> bool {
    
    todo!();
        /*
            static const char* appleGenres[] =
                {
                    "Rock/Blues",
                    "Electronic/Dance",
                    "Jazz",
                    "Urban",
                    "World/Ethnic",
                    "Cinematic/New Age",
                    "Orchestral",
                    "Country/Folk",
                    "Experimental",
                    "Other Genre"
                };

                for (int i = 0; i < numElementsInArray (appleGenres); ++i)
                    if (tag == appleGenres[i])
                        return true;

                return false;
        */
}

pub fn cate_chunk_read(
        input:  &mut dyn Read,
        length: u32) -> String {
    
    todo!();
        /*
            MemoryBlock mb;
                input.skipNextBytes (4);
                input.readIntoMemoryBlock (mb, (ssize_t) length - 4);

                StringArray tagsArray;

                auto* data = static_cast<const char*> (mb.getData());
                auto* dataEnd = data + mb.getSize();

                while (data < dataEnd)
                {
                    bool isGenre = false;

                    if (isValidTag (data))
                    {
                        auto tag = String (CharPointer_UTF8 (data), CharPointer_UTF8 (dataEnd));
                        isGenre = isAppleGenre (tag);
                        tagsArray.add (tag);
                    }

                    data += isGenre ? 118 : 50;

                    if (data < dataEnd && data[0] == 0)
                    {
                        if      (data + 52  < dataEnd && isValidTag (data + 50))   data += 50;
                        else if (data + 120 < dataEnd && isValidTag (data + 118))  data += 118;
                        else if (data + 170 < dataEnd && isValidTag (data + 168))  data += 168;
                    }
                }

                return tagsArray.joinIntoString (";");
        */
}
