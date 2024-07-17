crate::ix!();

pub struct FlexBoxLayoutCalculationRowInfo
{
    num_items:    i32,
    cross_size:   FlexBoxLayoutCalculationCoord,
    liney:        FlexBoxLayoutCalculationCoord,
    total_length: FlexBoxLayoutCalculationCoord,
}
