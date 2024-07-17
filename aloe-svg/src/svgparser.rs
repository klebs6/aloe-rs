crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_SVGParser.cpp]

pub trait CreateFromSvg {

    fn create_fromsvg(&mut self, svg_document: &XmlElement) -> Box<Drawable>;
}

pub trait CreateFromSvgFile {

    fn create_from_svg_file(&mut self, svg_file: &File) -> Box<Drawable>;
}

pub trait ParseSvgPath {

    fn parse_svg_path(&mut self, svg_path: &String) -> Path;
}

///-------------------
impl<'a> CreateFromSvg for Drawable<'a> {
    
     fn create_fromsvg(&mut self, svg_document: &XmlElement) -> Box<Drawable> {
        
        todo!();
        /*
            if (! svgDocument.hasTagNameIgnoringNamespace ("svg"))
            return {};

        SVGState state (&svgDocument);
        return std::unique_ptr<Drawable> (state.parseSVGElement (SVGState::SvgStateXmlPath (&svgDocument, {})));
        */
    }
}

impl<'a> CreateFromSvgFile for Drawable<'a> {
    
    fn create_from_svg_file(&mut self, svg_file: &File) -> Box<Drawable> {
        
        todo!();
        /*
            if (auto xml = parseXMLIfTagMatches (svgFile, "svg"))
            return createFromSVG (*xml);

        return {};
        */
    }
}

impl<'a> ParseSvgPath for Drawable<'a> {
    
    fn parse_svg_path(&mut self, svg_path: &String) -> Path {
        
        todo!();
        /*
            SVGState state (nullptr);
        Path p;
        state.parsePathString (p, svgPath);
        return p;
        */
    }
}
