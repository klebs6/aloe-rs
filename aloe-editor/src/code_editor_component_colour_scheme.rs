crate::ix!();

/**
  | Defines a colour for a token type
  |
  */
pub struct CodeEditorComponentColourSchemeTokenType
{
    name:   String,
    colour: Colour,
}

/**
  | Defines a syntax highlighting colour
  | scheme
  |
  */
pub struct CodeEditorComponentColourScheme {
    types: Vec<CodeEditorComponentColourSchemeTokenType>,
}

impl CodeEditorComponentColourScheme {

    pub fn set(&mut self, 
        name:   &String,
        colour: Colour)  {
        
        todo!();
        /*
            for (auto& tt : types)
        {
            if (tt.name == name)
            {
                tt.colour = colour;
                return;
            }
        }

        CodeEditorComponentColourSchemeTokenType tt;
        tt.name = name;
        tt.colour = colour;
        types.add (tt);
        */
    }
}

