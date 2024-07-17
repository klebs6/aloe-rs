crate::ix!();

/**
  | Represents a track.
  |
  */
pub struct GridTrackInfo {

    /**
      | Either a fraction or an absolute size
      | in pixels
      |
      */
    size:            f32,
    is_fraction:     bool,
    has_keyword:     bool,
    start_line_name: String,
    end_line_name:   String,
}

impl Default for GridTrackInfo {
    
    /**
      | Creates a track with auto dimension.
      |
      */
    fn default() -> Self {

        Self {
            size:            0.0,
            is_fraction:     false,
            has_keyword:     false,
            start_line_name: String::new(),
            end_line_name:   String::new(),
        }
    }
}

impl From<GridPx> for GridTrackInfo {

    fn from(size_in_pixels: GridPx) -> Self {
    
        todo!();
        /*
            : size (static_cast<float> (sizeInPixels.pixels)), isFraction (false)
        */
    }
}

impl From<GridFr> for GridTrackInfo {

    fn from(fraction_of_free_space: GridFr) -> Self {
    
        todo!();
        /*
            : size ((float)fractionOfFreeSpace.fraction), isFraction (true)
        */
    }
}
    
impl GridTrackInfo {

    pub fn is_auto(&self) -> bool {
        
        todo!();
        /*
            return hasKeyword;
        */
    }
    
    pub fn is_fractional(&self) -> bool {
        
        todo!();
        /*
            return isFraction;
        */
    }
    
    pub fn is_pixels(&self) -> bool {
        
        todo!();
        /*
            return ! isFraction;
        */
    }
    
    pub fn get_start_line_name(&self) -> &String {
        
        todo!();
        /*
            return startLineName;
        */
    }
    
    pub fn get_end_line_name(&self) -> &String {
        
        todo!();
        /*
            return endLineName;
        */
    }

    /**
      | Get the track's size - which might mean
      | an absolute pixels value or a fractional
      | ratio.
      |
      */
    pub fn get_size(&self) -> f32 {
        
        todo!();
        /*
            return size;
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*
        : has_keyword(true),

        
        */
    }
    
    pub fn new_with_end_line_name_plus_size_in_pixels(
        size_in_pixels:       GridPx,
        end_line_name_to_use: &String) -> Self {
    
        todo!();
        /*
        : track_info(sizeInPixels),

            endLineName = endLineNameToUse;
        */
    }
    
    pub fn new_with_end_line_name_plus_free_space_fraction(
        fraction_of_free_space: GridFr,
        end_line_name_to_use:   &String) -> Self {
    
        todo!();
        /*
        : track_info(fractionOfFreeSpace),

            endLineName = endLineNameToUse;
        */
    }
    
    pub fn new_with_start_line_name_plus_size_in_pixels(
        start_line_name_to_use: &String,
        size_in_pixels:         GridPx) -> Self {
    
        todo!();
        /*
        : track_info(sizeInPixels),

            startLineName = startLineNameToUse;
        */
    }
    
    pub fn new_with_start_line_name_plus_fraction_of_free_space(
        start_line_name_to_use: &String,
        fraction_of_free_space: GridFr) -> Self {
    
        todo!();
        /*
        : track_info(fractionOfFreeSpace),

            startLineName = startLineNameToUse;
        */
    }
    
    pub fn new_with_start_line_name_plus_size_in_pixels_plus_end_line_name(
        start_line_name_to_use: &String,
        size_in_pixels:         GridPx,
        end_line_name_to_use:   &String) -> Self {
    
        todo!();
        /*
        : track_info(startLineNameToUse, sizeInPixels),

            endLineName = endLineNameToUse;
        */
    }
    
    pub fn new_with_start_and_end_name_plus_free_space_fraction(
        start_line_name_to_use: &String,
        fraction_of_free_space: GridFr,
        end_line_name_to_use:   &String) -> Self {
    
        todo!();
        /*
        : track_info(startLineNameToUse, fractionOfFreeSpace),

            endLineName = endLineNameToUse;
        */
    }
    
    pub fn get_absolute_size(&self, relative_fractional_unit: f32) -> f32 {
        
        todo!();
        /*
            if (isFractional())
            return size * relativeFractionalUnit;
        else
            return size;
        */
    }
}
