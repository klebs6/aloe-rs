crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_freetype_Fonts.cpp]

#[no_copy]
#[leak_detector]
pub struct FTLibWrapper {
    base:    ReferenceCountedObject,
    library: FT_Library,
}

pub type FTLibWrapperPtr = ReferenceCountedObjectPtr<FTLibWrapper>;

impl Default for FTLibWrapper {
    
    fn default() -> Self {
        todo!();
        /*


            if (FT_Init_FreeType (&library) != 0)
            {
                library = {};
                DBG ("Failed to initialize FreeType");
            }
        */
    }
}

impl Drop for FTLibWrapper {
    fn drop(&mut self) {
        todo!();
        /*
            if (library != nullptr)
                FT_Done_FreeType (library);
        */
    }
}

///----------------------------
#[no_copy]
#[leak_detector]
pub struct FTFaceWrapper {
    base:            ReferenceCountedObject,
    face:            FT_Face,
    library:         FTLibWrapperPtr,
    saved_face_data: MemoryBlock,
}

pub type FTFaceWrapperPtr = ReferenceCountedObjectPtr<FTFaceWrapper>;

impl Drop for FTFaceWrapper {
    fn drop(&mut self) {
        todo!();
        /*
            if (face != nullptr)
                FT_Done_Face (face);
        */
    }
}

impl FTFaceWrapper {

    pub fn new(
        ft_lib:     &FTLibWrapperPtr,
        file:       &File,
        face_index: i32) -> Self {
    
        todo!();
        /*
        : library(ftLib),

            if (FT_New_Face (ftLib->library, file.getFullPathName().toUTF8(), faceIndex, &face) != 0)
                face = {};
        */
    }
    
    pub fn new_with_data(
        ft_lib:     &FTLibWrapperPtr,
        data:       *const c_void,
        data_size:  usize,
        face_index: i32) -> Self {
    
        todo!();
        /*
        : library(ftLib),
        : saved_face_data(data, dataSize),

            if (FT_New_Memory_Face (ftLib->library, (const FT_Byte*) savedFaceData.getData(),
                                    (FT_Long) savedFaceData.getSize(), faceIndex, &face) != 0)
                face = {};
        */
    }
}

///--------------------
#[no_copy]
#[leak_detector]
pub struct FTTypefaceList {
    base:    DeletedAtShutdown,
    library: FTLibWrapperPtr,
    faces:   Vec<Box<KnownTypeface>>,
}

aloe_declare_singleton_singlethreaded_minimal!{
    FTTypefaceList
}


#[no_copy]
#[leak_detector]
pub struct KnownTypeface {
    file:          File,
    family:        String,
    style:         String,
    face_index:    i32,
    is_monospaced: bool,
    is_sans_serif: bool,
}

impl KnownTypeface {

    pub fn new(
        f:     &File,
        index: i32,
        face:  &FTFaceWrapper) -> Self {
    
        todo!();
        /*
            : file (f),
                 family (face.face->family_name),
                 style (face.face->style_name),
                 faceIndex (index),
                 isMonospaced ((face.face->face_flags & FT_FACE_FLAG_FIXED_WIDTH) != 0),
                 isSansSerif (isFaceSansSerif (family))
        */
    }
}

impl Default for FTTypefaceList {
    
    fn default() -> Self {
        todo!();
        /*


            : library (new FTLibWrapper())
            scanFontPaths (getDefaultFontDirectories());
        */
    }
}

impl Drop for FTTypefaceList {
    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

impl FTTypefaceList {

    pub fn select_unicode_charmap(face: *mut FTFaceWrapper) -> FTFaceWrapperPtr {
        
        todo!();
        /*
            if (face != nullptr)
                if (FT_Select_Charmap (face->face, ft_encoding_unicode) != 0)
                    FT_Set_Charmap (face->face, face->face->charmaps[0]);

            return face;
        */
    }
    
    pub fn create_face_with_data(
        &mut self, 
        data:      *const c_void,
        data_size: usize,
        index:     i32) -> FTFaceWrapperPtr {
        
        todo!();
        /*
            return selectUnicodeCharmap (new FTFaceWrapper (library, data, dataSize, index));
        */
    }
    
    pub fn create_face_from_file(
        &mut self, 
        file:  &File,
        index: i32) -> FTFaceWrapperPtr {
        
        todo!();
        /*
            return selectUnicodeCharmap (new FTFaceWrapper (library, file, index));
        */
    }
    
    pub fn create_face_from_font_name(
        &mut self, 
        font_name:  &String,
        font_style: &String) -> FTFaceWrapperPtr {
        
        todo!();
        /*
            auto ftFace = matchTypeface (fontName, fontStyle);

            if (ftFace == nullptr)  ftFace = matchTypeface (fontName, "Regular");
            if (ftFace == nullptr)  ftFace = matchTypeface (fontName, {});

            if (ftFace != nullptr)
                return createFace (ftFace->file, ftFace->faceIndex);

            return nullptr;
        */
    }
    
    pub fn find_all_family_names(&self) -> Vec<String> {
        
        todo!();
        /*
            StringArray s;

            for (auto* face : faces)
                s.addIfNotAlreadyThere (face->family);

            return s;
        */
    }
    
    pub fn index_of_regular_style(styles: &Vec<String>) -> i32 {
        
        todo!();
        /*
            int i = styles.indexOf ("Regular", true);

            if (i >= 0)
                return i;

            for (i = 0; i < styles.size(); ++i)
                if (! (styles[i].containsIgnoreCase ("Bold") || styles[i].containsIgnoreCase ("Italic")))
                    return i;

            return -1;
        */
    }
    
    pub fn find_all_typeface_styles(&self, family: &String) -> Vec<String> {
        
        todo!();
        /*
            StringArray s;

            for (auto* face : faces)
                if (face->family == family)
                    s.addIfNotAlreadyThere (face->style);

            // try to get a regular style to be first in the list
            auto regular = indexOfRegularStyle (s);

            if (regular > 0)
                s.strings.swap (0, regular);

            return s;
        */
    }
    
    pub fn scan_font_paths(&mut self, paths: &Vec<String>)  {
        
        todo!();
        /*
            for (auto& path : paths)
            {
                for (const auto& iter : RangedDirectoryIterator (File::getCurrentWorkingDirectory().getChildFile (path), true))
                    if (iter.getFile().hasFileExtension ("ttf;pfb;pcf;otf"))
                        scanFont (iter.getFile());
            }
        */
    }
    
    pub fn get_monospaced_names(&self, mono_spaced: &mut Vec<String>)  {
        
        todo!();
        /*
            for (auto* face : faces)
                if (face->isMonospaced)
                    monoSpaced.addIfNotAlreadyThere (face->family);
        */
    }
    
    pub fn get_serif_names(&self, serif: &mut Vec<String>) {
        
        todo!();
        /*
            for (auto* face : faces)
                if (! (face->isSansSerif || face->isMonospaced))
                    serif.addIfNotAlreadyThere (face->family);
        */
    }
    
    pub fn get_sans_serif_names(&self, sans_serif: &mut Vec<String>)  {
        
        todo!();
        /*
            for (auto* face : faces)
                if (face->isSansSerif)
                    sansSerif.addIfNotAlreadyThere (face->family);
        */
    }
    
    pub fn scan_font(&mut self, file: &File)  {
        
        todo!();
        /*
            int faceIndex = 0;
            int numFaces = 0;

            do
            {
                FTFaceWrapper face (library, file, faceIndex);

                if (face.face != nullptr)
                {
                    if (faceIndex == 0)
                        numFaces = (int) face.face->num_faces;

                    if ((face.face->face_flags & FT_FACE_FLAG_SCALABLE) != 0)
                        faces.add (new KnownTypeface (file, faceIndex, face));
                }

                ++faceIndex;
            }
            while (faceIndex < numFaces);
        */
    }
    
    pub fn match_typeface(&self, 
        family_name: &String,
        style:       &String) -> *const KnownTypeface {
        
        todo!();
        /*
            for (auto* face : faces)
                if (face->family == familyName
                      && (face->style.equalsIgnoreCase (style) || style.isEmpty()))
                    return face;

            return nullptr;
        */
    }
    
    pub fn is_face_sans_serif(family: &String) -> bool {
        
        todo!();
        /*
            static const char* sansNames[] = { "Sans", "Verdana", "Arial", "Ubuntu" };

            for (auto* name : sansNames)
                if (family.containsIgnoreCase (name))
                    return true;

            return false;
        */
    }
}

aloe_implement_singleton!{
    FTTypefaceList
}

///--------------------
#[no_copy]
pub struct FreeTypeTypeface {
    base:         CustomTypeface,
    face_wrapper: FTFaceWrapperPtr,
}

impl FreeTypeTypeface {

    pub fn new_fron_font_ref(font: &Font) -> Self {
    
        todo!();
        /*


            : faceWrapper (FTTypefaceList::getInstance()->createFace (font.getTypefaceName(),
                                                                      font.getTypefaceStyle()))

            if (faceWrapper != nullptr)
                initialiseCharacteristics (font.getTypefaceName(),
                                           font.getTypefaceStyle());
        */
    }
    
    pub fn new(
        data:      *const c_void,
        data_size: usize) -> Self {
    
        todo!();
        /*


            : faceWrapper (FTTypefaceList::getInstance()->createFace (data, dataSize, 0))

            if (faceWrapper != nullptr)
                initialiseCharacteristics (faceWrapper->face->family_name,
                                           faceWrapper->face->style_name);
        */
    }
    
    pub fn initialise_characteristics(&mut self, 
        font_name:  &String,
        font_style: &String)  {
        
        todo!();
        /*
            setCharacteristics (fontName, fontStyle,
                                faceWrapper->face->ascender / (float) (faceWrapper->face->ascender - faceWrapper->face->descender),
                                L' ');
        */
    }
    
    pub fn load_glyph_if_possible(&mut self, character: wchar_t) -> bool {
        
        todo!();
        /*
            if (faceWrapper != nullptr)
            {
                auto face = faceWrapper->face;
                auto glyphIndex = FT_Get_Char_Index (face, (FT_ULong) character);

                if (FT_Load_Glyph (face, glyphIndex, FT_LOAD_NO_SCALE | FT_LOAD_NO_BITMAP | FT_LOAD_IGNORE_TRANSFORM | FT_LOAD_NO_HINTING) == 0
                      && face->glyph->format == ft_glyph_format_outline)
                {
                    auto scale = 1.0f / (float) (face->ascender - face->descender);
                    Path destShape;

                    if (getGlyphShape (destShape, face->glyph->outline, scale))
                    {
                        addGlyph (character, destShape, (float) face->glyph->metrics.horiAdvance * scale);

                        if ((face->face_flags & FT_FACE_FLAG_KERNING) != 0)
                            addKerning (face, (uint32) character, glyphIndex);

                        return true;
                    }
                }
            }

            return false;
        */
    }
    
    pub fn get_glyph_shape(&mut self, 
        dest_shape: &mut Path,
        outline:    &FT_Outline,
        scalex:     f32) -> bool {
        
        todo!();
        /*
            auto scaleY = -scaleX;
            auto* contours = outline.contours;
            auto* tags = outline.tags;
            auto* points = outline.points;

            for (int c = 0; c < outline.n_contours; ++c)
            {
                const int startPoint = (c == 0) ? 0 : contours [c - 1] + 1;
                const int endPoint = contours[c];

                for (int p = startPoint; p <= endPoint; ++p)
                {
                    auto x = scaleX * (float) points[p].x;
                    auto y = scaleY * (float) points[p].y;

                    if (p == startPoint)
                    {
                        if (FT_CURVE_TAG (tags[p]) == FT_Curve_Tag_Conic)
                        {
                            auto x2 = scaleX * (float) points[endPoint].x;
                            auto y2 = scaleY * (float) points[endPoint].y;

                            if (FT_CURVE_TAG (tags[endPoint]) != FT_Curve_Tag_On)
                            {
                                x2 = (x + x2) * 0.5f;
                                y2 = (y + y2) * 0.5f;
                            }

                            destShape.startNewSubPath (x2, y2);
                        }
                        else
                        {
                            destShape.startNewSubPath (x, y);
                        }
                    }

                    if (FT_CURVE_TAG (tags[p]) == FT_Curve_Tag_On)
                    {
                        if (p != startPoint)
                            destShape.lineTo (x, y);
                    }
                    else if (FT_CURVE_TAG (tags[p]) == FT_Curve_Tag_Conic)
                    {
                        const int nextIndex = (p == endPoint) ? startPoint : p + 1;
                        auto x2 = scaleX * (float) points[nextIndex].x;
                        auto y2 = scaleY * (float) points[nextIndex].y;

                        if (FT_CURVE_TAG (tags [nextIndex]) == FT_Curve_Tag_Conic)
                        {
                            x2 = (x + x2) * 0.5f;
                            y2 = (y + y2) * 0.5f;
                        }
                        else
                        {
                            ++p;
                        }

                        destShape.quadraticTo (x, y, x2, y2);
                    }
                    else if (FT_CURVE_TAG (tags[p]) == FT_Curve_Tag_Cubic)
                    {
                        const int next1 = p + 1;
                        const int next2 = (p == (endPoint - 1)) ? startPoint : (p + 2);

                        if (p >= endPoint
                             || FT_CURVE_TAG (tags[next1]) != FT_Curve_Tag_Cubic
                             || FT_CURVE_TAG (tags[next2]) != FT_Curve_Tag_On)
                            return false;

                        auto x2 = scaleX * (float) points[next1].x;
                        auto y2 = scaleY * (float) points[next1].y;
                        auto x3 = scaleX * (float) points[next2].x;
                        auto y3 = scaleY * (float) points[next2].y;

                        destShape.cubicTo (x, y, x2, y2, x3, y3);
                        p += 2;
                    }
                }

                destShape.closeSubPath();
            }

            return true;
        */
    }
    
    pub fn add_kerning(
        &mut self, 
        face:        FT_Face,
        character:   u32,
        glyph_index: u32)  {
        
        todo!();
        /*
            auto height = (float) (face->ascender - face->descender);

            uint32 rightGlyphIndex;
            auto rightCharCode = FT_Get_First_Char (face, &rightGlyphIndex);

            while (rightGlyphIndex != 0)
            {
                FT_Vector kerning;

                if (FT_Get_Kerning (face, glyphIndex, rightGlyphIndex, ft_kerning_unscaled, &kerning) == 0
                       && kerning.x != 0)
                    addKerningPair ((aloe_wchar) character, (aloe_wchar) rightCharCode, (float) kerning.x / height);

                rightCharCode = FT_Get_Next_Char (face, rightCharCode, &rightGlyphIndex);
            }
        */
    }
}
