crate::ix!();

///-------------------
pub struct GridSizeCalculation {
    relative_width_unit:  f32, // default = 0.0f
    relative_height_unit: f32, // default = 0.0f
    remaining_width:      f32, // default = 0.0f
    remaining_height:     f32, // default = 0.0f
}

impl GridSizeCalculation {

    pub fn get_total_absolute_size(
        tracks:   &[GridTrackInfo],
        gap_size: GridPx) -> f32 {
        
        todo!();
        /*
            float totalCellSize = 0.0f;

            for (const auto& trackInfo : tracks)
                if (! trackInfo.isFractional() || trackInfo.isAuto())
                    totalCellSize += trackInfo.getSize();

            float totalGap = tracks.size() > 1 ? static_cast<float> ((tracks.size() - 1) * gapSize.pixels)
                                               : 0.0f;

            return totalCellSize + totalGap;
        */
    }
    
    pub fn get_relative_unit_size(
        size:           f32,
        total_absolute: f32,
        tracks:         &[GridTrackInfo]) -> f32 {
        
        todo!();
        /*
            const float totalRelative = jlimit (0.0f, size, size - totalAbsolute);
            float factorsSum = 0.0f;

            for (const auto& trackInfo : tracks)
                if (trackInfo.isFractional())
                    factorsSum += trackInfo.getSize();

            jassert (factorsSum != 0.0f);
            return totalRelative / factorsSum;
        */
    }
    
    pub fn get_total_absolute_height(
        row_tracks: &[GridTrackInfo],
        row_gap:    GridPx) -> f32 {
        
        todo!();
        /*
            return getTotalAbsoluteSize (rowTracks, rowGap);
        */
    }
    
    pub fn get_total_absolute_width(
        column_tracks: &[GridTrackInfo],
        column_gap:    GridPx) -> f32 {
        
        todo!();
        /*
            return getTotalAbsoluteSize (columnTracks, columnGap);
        */
    }
    
    pub fn get_relative_width_unit(
        grid_width:    f32,
        column_gap:    GridPx,
        column_tracks: &[GridTrackInfo]) -> f32 {
        
        todo!();
        /*
            return getRelativeUnitSize (gridWidth, getTotalAbsoluteWidth (columnTracks, columnGap), columnTracks);
        */
    }
    
    pub fn get_relative_height_unit(
        grid_height: f32,
        row_gap:     GridPx,
        row_tracks:  &[GridTrackInfo]) -> f32 {
        
        todo!();
        /*
            return getRelativeUnitSize (gridHeight, getTotalAbsoluteHeight (rowTracks, rowGap), rowTracks);
        */
    }
    
    pub fn has_any_fractions(tracks: &[GridTrackInfo]) -> bool {
        
        todo!();
        /*
            for (auto& t : tracks)
                if (t.isFractional())
                    return true;

            return false;
        */
    }
    
    pub fn compute_sizes(&mut self, 
        grid_width:        f32,
        grid_height:       f32,
        column_gap_to_use: GridPx,
        row_gap_to_use:    GridPx,
        column_tracks:     &[GridTrackInfo],
        row_tracks:        &[GridTrackInfo])  {
        
        todo!();
        /*
            if (hasAnyFractions (columnTracks))
                relativeWidthUnit = getRelativeWidthUnit (gridWidth, columnGapToUse, columnTracks);
            else
                remainingWidth = gridWidth - getTotalAbsoluteSize (columnTracks, columnGapToUse);

            if (hasAnyFractions (rowTracks))
                relativeHeightUnit = getRelativeHeightUnit (gridHeight, rowGapToUse, rowTracks);
            else
                remainingHeight = gridHeight - getTotalAbsoluteSize (rowTracks, rowGapToUse);
        */
    }
}
