crate::ix!();

pub fn create_dragging_hand_cursor()  {
    
    todo!();
        /*
            constexpr unsigned char dragHandData[] = {
            71,73,70,56,57,97,16,0,16,0,145,2,0,0,0,0,255,255,255,0,0,0,0,0,0,33,249,4,1,0,0,2,0,44,0,0,0,0,16,0,16,0,
            0,2,52,148,47,0,200,185,16,130,90,12,74,139,107,84,123,39,132,117,151,116,132,146,248,60,209,138,98,22,203,
            114,34,236,37,52,77,217, 247,154,191,119,110,240,193,128,193,95,163,56,60,234,98,135,2,0,59
        };

        return CustomMouseCursorInfo (ImageFileFormat::loadFrom (dragHandData, (size_t) numElementsInArray (dragHandData)), { 8, 7 }).create();
        */
}

pub fn has_work_area_data(prop: &GetXProperty) -> bool {
    
    todo!();
        /*
            return prop.success
            && prop.actualType == XA_CARDINAL
            && prop.actualFormat == 32
            && prop.numItems == 4
            && prop.data != nullptr;
        */
}

pub fn get_work_area(prop: &GetXProperty) -> Rectangle<i32> {
    
    todo!();
        /*
            if (hasWorkAreaData (prop))
        {
            auto* positionData = prop.data;
            std::array<long, 4> position;

            for (auto& p : position)
            {
                memcpy (&p, positionData, sizeof (long));
                positionData += sizeof (long);
            }

            return { (int) position[0], (int) position[1],
                     (int) position[2], (int) position[3] };
        }

        return {};
        */
}
