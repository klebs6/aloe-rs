crate::ix!();

///---------------
pub struct GridPlacementHelpersLineRange { 
    start: i32,
    end:   i32,
}

pub struct GridPlacementHelpersLineArea  { 
    column: GridPlacementHelpersLineRange,
    row:    GridPlacementHelpersLineRange,
}

pub struct GridPlacementHelpersLineInfo  { 
    line_names: Vec<String>,
}

pub struct GridPlacementHelpersNamedArea
{
    name:  String,
    lines: GridPlacementHelpersLineArea,
}

pub mod grid_placement_helpers {

    use super::*;

    pub const invalid: isize = -999999;
    pub const emptyAreaCharacter: &'static str = ".";

    pub fn get_array_of_lines_from_tracks(tracks: &[GridTrackInfo]) -> Vec<GridPlacementHelpersLineInfo> {
        
        todo!();
        /*
            // fill line info array
            Vec<GridPlacementHelpersLineInfo> lines;

            for (int i = 1; i <= tracks.size(); ++i)
            {
                const auto& currentTrack = tracks.getReference (i - 1);

                if (i == 1) // start line
                {
                    GridPlacementHelpersLineInfo li;
                    li.lineNames.add (currentTrack.getStartLineName());
                    lines.add (li);
                }

                if (i > 1 && i <= tracks.size()) // two lines in between tracks
                {
                    const auto& prevTrack = tracks.getReference (i - 2);

                    GridPlacementHelpersLineInfo li;
                    li.lineNames.add (prevTrack.getEndLineName());
                    li.lineNames.add (currentTrack.getStartLineName());

                    lines.add (li);
                }

                if (i == tracks.size()) // end line
                {
                    GridPlacementHelpersLineInfo li;
                    li.lineNames.add (currentTrack.getEndLineName());
                    lines.add (li);
                }
            }

            jassert (lines.size() == tracks.size() + 1);

            return lines;
        */
    }
    
    pub fn deduce_absolute_line_number_from_line_name(
        prop:   GridItemProperty,
        tracks: &[GridTrackInfo]) -> i32 {
        
        todo!();
        /*
            jassert (prop.hasAbsolute());

            const auto lines = getArrayOfLinesFromTracks (tracks);
            int count = 0;

            for (int i = 0; i < lines.size(); i++)
            {
                for (const auto& name : lines.getReference (i).lineNames)
                {
                    if (prop.getName() == name)
                    {
                        ++count;
                        break;
                    }
                }

                if (count == prop.getNumber())
                    return i + 1;
            }

            jassertfalse;
            return count;
        */
    }
    
    pub fn deduce_absolute_line_number(
        prop:   GridItemProperty,
        tracks: &[GridTrackInfo]) -> i32 {
        
        todo!();
        /*
            jassert (prop.hasAbsolute());

            if (prop.hasName())
                return deduceAbsoluteLineNumberFromLineName (prop, tracks);

            return prop.getNumber() > 0 ? prop.getNumber() : tracks.size() + 2 + prop.getNumber();
        */
    }
    
    pub fn deduce_absolute_line_number_from_named_span(
        start_line_number:  i32,
        property_with_span: GridItemProperty,
        tracks:             &[GridTrackInfo]) -> i32 {
        
        todo!();
        /*
            jassert (propertyWithSpan.hasSpan());

            const auto lines = getArrayOfLinesFromTracks (tracks);
            int count = 0;

            for (int i = startLineNumber; i < lines.size(); i++)
            {
                for (const auto& name : lines.getReference (i).lineNames)
                {
                    if (propertyWithSpan.getName() == name)
                    {
                        ++count;
                        break;
                    }
                }

                if (count == propertyWithSpan.getNumber())
                    return i + 1;
            }

            jassertfalse;
            return count;
        */
    }
    
    pub fn deduce_absolute_line_number_based_on_span(
        start_line_number:  i32,
        property_with_span: GridItemProperty,
        tracks:             &[GridTrackInfo]) -> i32 {
        
        todo!();
        /*
            jassert (propertyWithSpan.hasSpan());

            if (propertyWithSpan.hasName())
                return deduceAbsoluteLineNumberFromNamedSpan (startLineNumber, propertyWithSpan, tracks);

            return startLineNumber + propertyWithSpan.getNumber();
        */
    }
    
    pub fn deduce_line_range(
        prop:   GridItemStartAndEndProperty,
        tracks: &[GridTrackInfo]) -> GridPlacementHelpersLineRange {
        
        todo!();
        /*
            GridPlacementHelpersLineRange s;

            jassert (! (prop.start.hasAuto() && prop.end.hasAuto()));

            if (prop.start.hasAbsolute() && prop.end.hasAuto())
            {
                prop.end = GridItem::Span (1);
            }
            else if (prop.start.hasAuto() && prop.end.hasAbsolute())
            {
                prop.start = GridItem::Span (1);
            }

            if (prop.start.hasAbsolute() && prop.end.hasAbsolute())
            {
                s.start = deduceAbsoluteLineNumber (prop.start, tracks);
                s.end   = deduceAbsoluteLineNumber (prop.end, tracks);
            }
            else if (prop.start.hasAbsolute() && prop.end.hasSpan())
            {
                s.start = deduceAbsoluteLineNumber (prop.start, tracks);
                s.end   = deduceAbsoluteLineNumberBasedOnSpan (s.start, prop.end, tracks);
            }
            else if (prop.start.hasSpan() && prop.end.hasAbsolute())
            {
                s.start = deduceAbsoluteLineNumber (prop.end, tracks);
                s.end   = deduceAbsoluteLineNumberBasedOnSpan (s.start, prop.start, tracks);
            }
            else
            {
                // Can't have an item with spans on both start and end.
                jassertfalse;
                s.start = s.end = {};
            }

            // swap if start overtakes end
            if (s.start > s.end)
                std::swap (s.start, s.end);
            else if (s.start == s.end)
                s.end = s.start + 1;

            return s;
        */
    }
    
    pub fn deduce_line_area(
        item:        &GridItem,
        grid:        &Grid,
        named_areas: &HashMap<String,GridPlacementHelpersLineArea>) -> GridPlacementHelpersLineArea {
        
        todo!();
        /*
            if (item.area.isNotEmpty() && ! grid.templateAreas.isEmpty())
            {
                // Must be a named area!
                jassert (namedAreas.count (item.area) != 0);

                return namedAreas.at (item.area);
            }

            return { deduceLineRange (item.column, grid.templateColumns),
                     deduceLineRange (item.row,    grid.templateRows) };
        */
    }
    
    pub fn parse_areas_property(areas_strings: &Vec<String>) -> Vec<Vec<String>> {
        
        todo!();
        /*
            Vec<Vec<String>> strings;

            for (const auto& areaString : areasStrings)
                strings.add (Vec<String>::fromTokens (areaString, false));

            if (strings.size() > 0)
            {
                for (auto s : strings)
                {
                    jassert (s.size() == strings[0].size()); // all rows must have the same number of columns
                }
            }

            return strings;
        */
    }
    
    pub fn find_area(strings_arrays: &mut Vec<Vec<String>>) -> GridPlacementHelpersNamedArea {
        
        todo!();
        /*
            GridPlacementHelpersNamedArea area;

            for (auto& stringArray : stringsArrays)
            {
                for (auto& string : stringArray)
                {
                    // find anchor
                    if (area.name.isEmpty())
                    {
                        if (string != emptyAreaCharacter)
                        {
                            area.name = string;
                            area.lines.row.start = stringsArrays.indexOf (stringArray) + 1; // non-zero indexed;
                            area.lines.column.start = stringArray.indexOf (string) + 1; // non-zero indexed;

                            area.lines.row.end = stringsArrays.indexOf (stringArray) + 2;
                            area.lines.column.end = stringArray.indexOf (string) + 2;

                            // mark as visited
                            string = emptyAreaCharacter;
                        }
                    }
                    else
                    {
                        if (string == area.name)
                        {
                            area.lines.row.end = stringsArrays.indexOf (stringArray) + 2;
                            area.lines.column.end = stringArray.indexOf (string) + 2;

                            // mark as visited
                            string = emptyAreaCharacter;
                        }
                    }
                }
            }

            return area;
        */
    }
    
    pub fn deduce_named_areas(areas_strings: &Vec<String>) -> HashMap<String,GridPlacementHelpersLineArea> {
        
        todo!();
        /*
            auto stringsArrays = parseAreasProperty (areasStrings);

            std::map<String, GridPlacementHelpersLineArea> areas;

            for (auto area = findArea (stringsArrays); area.name.isNotEmpty(); area = findArea (stringsArrays))
            {
                if (areas.count (area.name) == 0)
                    areas[area.name] = area.lines;
                else
                    // Make sure your template-areas property only has one area with the same name and is well-formed
                    jassertfalse;
            }

            return areas;
        */
    }
    
    pub fn get_coord(
        track_number:  i32,
        relative_unit: f32,
        gap:           GridPx,
        tracks:        &[GridTrackInfo]) -> f32 {
        
        todo!();
        /*
            float c = 0;

            for (const auto* it = tracks.begin(); it != tracks.begin() + trackNumber - 1; ++it)
                c += it->getAbsoluteSize (relativeUnit) + static_cast<float> (gap.pixels);

            return c;
        */
    }
    
    pub fn get_cell_bounds(
        column_number: i32,
        row_number:    i32,
        column_tracks: &[GridTrackInfo],
        row_tracks:    &[GridTrackInfo],
        calculation:   GridSizeCalculation,
        column_gap:    GridPx,
        row_gap:       GridPx) -> Rectangle<f32> {
        
        todo!();
        /*
            jassert (columnNumber >= 1 && columnNumber <= columnTracks.size());
            jassert (rowNumber >= 1 && rowNumber <= rowTracks.size());

            const auto x = getCoord (columnNumber, calculation.relativeWidthUnit, columnGap, columnTracks);
            const auto y = getCoord (rowNumber, calculation.relativeHeightUnit, rowGap, rowTracks);

            const auto& columnTrackInfo = columnTracks.getReference (columnNumber - 1);
            const float width = columnTrackInfo.getAbsoluteSize (calculation.relativeWidthUnit);

            const auto& rowTrackInfo = rowTracks.getReference (rowNumber - 1);
            const float height = rowTrackInfo.getAbsoluteSize (calculation.relativeHeightUnit);

            return { x, y, width, height };
        */
    }
    
    pub fn align_cell(
        area:              Rectangle<f32>,
        column_number:     i32,
        row_number:        i32,
        number_of_columns: i32,
        number_of_rows:    i32,
        calculation:       GridSizeCalculation,
        align_content:     GridAlignContent,
        justify_content:   GridJustifyContent) -> Rectangle<f32> {
        
        todo!();
        /*
            if (alignContent == GridAlignContent::end)
                area.setY (area.getY() + calculation.remainingHeight);

            if (justifyContent == GridJustifyContent::end)
                area.setX (area.getX() + calculation.remainingWidth);

            if (alignContent == GridAlignContent::center)
                area.setY (area.getY() + calculation.remainingHeight / 2);

            if (justifyContent == GridJustifyContent::center)
                area.setX (area.getX() + calculation.remainingWidth / 2);

            if (alignContent == GridAlignContent::spaceBetween)
            {
                const auto shift = ((float) (rowNumber - 1) * (calculation.remainingHeight / float(numberOfRows - 1)));
                area.setY (area.getY() + shift);
            }

            if (justifyContent == GridJustifyContent::spaceBetween)
            {
                const auto shift = ((float) (columnNumber - 1) * (calculation.remainingWidth / float(numberOfColumns - 1)));
                area.setX (area.getX() + shift);
            }

            if (alignContent == GridAlignContent::spaceEvenly)
            {
                const auto shift = ((float) rowNumber * (calculation.remainingHeight / float(numberOfRows + 1)));
                area.setY (area.getY() + shift);
            }

            if (justifyContent == GridJustifyContent::spaceEvenly)
            {
                const auto shift = ((float) columnNumber * (calculation.remainingWidth / float(numberOfColumns + 1)));
                area.setX (area.getX() + shift);
            }

            if (alignContent == GridAlignContent::spaceAround)
            {
                const auto inbetweenShift = calculation.remainingHeight / float(numberOfRows);
                const auto sidesShift = inbetweenShift / 2;
                auto shift = (float) (rowNumber - 1) * inbetweenShift + sidesShift;

                area.setY (area.getY() + shift);
            }

            if (justifyContent == GridJustifyContent::spaceAround)
            {
                const auto inbetweenShift = calculation.remainingWidth / float(numberOfColumns);
                const auto sidesShift = inbetweenShift / 2;
                auto shift = (float) (columnNumber - 1) * inbetweenShift + sidesShift;

                area.setX (area.getX() + shift);
            }

            return area;
        */
    }
    
    pub fn get_area_bounds(
        column_line_number_start: i32,
        column_line_number_end:   i32,
        row_line_number_start:    i32,
        row_line_number_end:      i32,
        column_tracks:            &[GridTrackInfo],
        row_tracks:               &[GridTrackInfo],
        calculation:              GridSizeCalculation,
        align_content:            GridAlignContent,
        justify_content:          GridJustifyContent,
        column_gap:               GridPx,
        row_gap:                  GridPx) -> Rectangle<f32> {
        
        todo!();
        /*
            auto startCell = getCellBounds (columnLineNumberStart, rowLineNumberStart,
                                            columnTracks, rowTracks,
                                            calculation,
                                            columnGap, rowGap);

            auto endCell = getCellBounds (columnLineNumberEnd - 1, rowLineNumberEnd - 1,
                                          columnTracks, rowTracks,
                                          calculation,
                                          columnGap, rowGap);

            startCell = alignCell (startCell,
                                   columnLineNumberStart, rowLineNumberStart,
                                   columnTracks.size(), rowTracks.size(),
                                   calculation,
                                   alignContent,
                                   justifyContent);

            endCell = alignCell (endCell,
                                 columnLineNumberEnd - 1, rowLineNumberEnd - 1,
                                 columnTracks.size(), rowTracks.size(),
                                 calculation,
                                 alignContent,
                                 justifyContent);

            auto horizontalRange = startCell.getHorizontalRange().getUnionWith (endCell.getHorizontalRange());
            auto verticalRange = startCell.getVerticalRange().getUnionWith (endCell.getVerticalRange());
            return { horizontalRange.getStart(),  verticalRange.getStart(),
                     horizontalRange.getLength(), verticalRange.getLength() };
        */
    }
}
