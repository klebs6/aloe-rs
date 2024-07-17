crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawablePath.h]

/**
  | A drawable object which renders a filled
  | or outlined shape.
  | 
  | For details on how to change the fill
  | and stroke, see the DrawableShape class.
  | 
  | @see Drawable, DrawableShape
  | 
  | @tags{GUI}
  |
  */
#[leak_detector]
#[derive(Default)]
pub struct DrawablePath<'a> {

    base: DrawableShape<'a>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/drawables/aloe_DrawablePath.cpp]
impl<'a> DrawablePath<'a> {

    pub fn new(other: &DrawablePath) -> Self {
    
        todo!();
        /*
        : drawable_shape(other),

            setPath (other.path);
        */
    }
    
    pub fn create_copy(&self) -> Box<Drawable> {
        
        todo!();
        /*
            return std::make_unique<DrawablePath> (*this);
        */
    }
    
    /**
      | Changes the path that will be drawn.
      | @see setFill, setStrokeType
      |
      */
    pub fn set_path_from_path_ref(&mut self, new_path: &Path)  {
        
        todo!();
        /*
            path = newPath;
        pathChanged();
        */
    }
    
    /**
      | Changes the path that will be drawn.
      | @see setFill, setStrokeType
      |
      */
    pub fn set_path(&mut self, new_path: PathBuf)  {
        
        todo!();
        /*
            path = std::move (newPath);
        pathChanged();
        */
    }
    
    /**
      | Returns the current path.
      |
      */
    pub fn get_path(&self) -> &Path {
        
        todo!();
        /*
            return path;
        */
    }
    
    /**
      | Returns the current path for the outline.
      |
      */
    pub fn get_stroke_path(&self) -> &Path {
        
        todo!();
        /*
            return strokePath;
        */
    }
}
