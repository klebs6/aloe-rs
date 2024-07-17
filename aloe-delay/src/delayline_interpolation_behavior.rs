crate::ix!();

pub trait SampleInterpolationBehavior<SampleType> where Self: Sized {

    fn interpolate_sample(
        delay_line: &mut DelayLine<SampleType,Self>, 
        channel:    i32

    ) -> SampleType;

    fn update_internal_variables(
        delay_line: &mut DelayLine<SampleType,Self>, 
    );
}

pub type LinearDelayLine<SampleType>   = DelayLine<SampleType, delay_line_interpolation_types::Linear>;
pub type ThiranDelayLine<SampleType>   = DelayLine<SampleType, delay_line_interpolation_types::Thiran>;
pub type LagrangeDelayLine<SampleType> = DelayLine<SampleType, delay_line_interpolation_types::Lagrange3rd>;
pub type VanillaDelayLine<SampleType>  = DelayLine<SampleType, delay_line_interpolation_types::NoInterpolation>;

/**
  | A collection of structs to pass as the
  | template argument when setting the
  | interpolation type for the DelayLine
  | class.
  |
  */
pub mod delay_line_interpolation_types {

    macro_rules! behavior { ($ty:ident) => { pub struct $ty {} } }

    /**
      | No interpolation between successive
      | samples in the delay line will be performed.
      | This is useful when the delay is a constant
      | integer or to create lo-fi audio effects.
      | 
      | @tags{DSP}
      |
      */
    behavior![NoInterpolation];

    /**
      | Successive samples in the delay line
      | will be linearly interpolated. This
      | type of interpolation has a low compuational
      | cost where the delay can be modulated
      | in real time, but it also introduces
      | a low-pass filtering effect into your
      | audio signal.
      | 
      | @tags{DSP}
      |
      */
    behavior![Linear];

    /**
      | Successive samples in the delay line
      | will be interpolated using a 3rd order
      | Lagrange interpolator. This method
      | incurs more computational overhead
      | than linear interpolation but reduces
      | the low-pass filtering effect whilst
      | remaining amenable to real time delay
      | modulation.
      | 
      | @tags{DSP}
      |
      */
    behavior![Lagrange3rd];

    /**
      | Successive samples in the delay line
      | will be interpolated using 1st order
      | Thiran interpolation. This method
      | is very efficient, and features a flat
      | amplitude frequency response in exchange
      | for less accuracy in the phase response.
      | This interpolation method is stateful
      | so is unsuitable for applications requiring
      | fast delay modulation.
      | 
      | @tags{DSP}
      |
      */
    behavior![Thiran];
}

impl<SampleType: SampleTypeInterface> SampleInterpolationBehavior<SampleType> for delay_line_interpolation_types::NoInterpolation {

    fn interpolate_sample(
        delay_line: &mut DelayLine<SampleType,Self>, 
        channel:    i32

    ) -> SampleType {

        todo!();
        /*
            auto index = (readPos[(size_t) channel] + delayInt) % totalSize;
            return bufferData.getSample (channel, index);
        */
    }

    fn update_internal_variables(
        _delay_line: &mut DelayLine<SampleType,Self>, 
    ) { 
        //noop
    }
}

impl<SampleType: SampleTypeInterface> SampleInterpolationBehavior<SampleType> for delay_line_interpolation_types::Linear {

    fn interpolate_sample(
        delay_line: &mut DelayLine<SampleType,Self>, 
        channel:    i32

    ) -> SampleType {

        todo!();
        /*
            auto index1 = readPos[(size_t) channel] + delayInt;
            auto index2 = index1 + 1;

            if (index2 >= totalSize)
            {
                index1 %= totalSize;
                index2 %= totalSize;
            }

            auto value1 = bufferData.getSample (channel, index1);
            auto value2 = bufferData.getSample (channel, index2);

            return value1 + delayFrac * (value2 - value1);
        */
    }

    fn update_internal_variables(
        _delay_line: &mut DelayLine<SampleType,Self>, 
    ) {
        //noop
    }
}

impl<SampleType: SampleTypeInterface> SampleInterpolationBehavior<SampleType> for delay_line_interpolation_types::Lagrange3rd {

    fn interpolate_sample(
        delay_line: &mut DelayLine<SampleType,Self>, 
        channel:    i32

    ) -> SampleType {

        todo!();

        /*
            auto index1 = readPos[(size_t) channel] + delayInt;
            auto index2 = index1 + 1;
            auto index3 = index2 + 1;
            auto index4 = index3 + 1;

            if (index4 >= totalSize)
            {
                index1 %= totalSize;
                index2 %= totalSize;
                index3 %= totalSize;
                index4 %= totalSize;
            }

            auto* samples = bufferData.getReadPointer (channel);

            auto value1 = samples[index1];
            auto value2 = samples[index2];
            auto value3 = samples[index3];
            auto value4 = samples[index4];

            auto d1 = delayFrac - 1.f;
            auto d2 = delayFrac - 2.f;
            auto d3 = delayFrac - 3.f;

            auto c1 = -d1 * d2 * d3 / 6.f;
            auto c2 = d2 * d3 * 0.5f;
            auto c3 = -d1 * d3 * 0.5f;
            auto c4 = d1 * d2 / 6.f;

            return value1 * c1 + delayFrac * (value2 * c2 + value3 * c3 + value4 * c4);
        */
    }

    fn update_internal_variables(
        delay_line: &mut DelayLine<SampleType,Self>, 
    ) {
        todo!();
        /*
            if (delayInt >= 1)
            {
                delayFrac++;
                delayInt--;
            }
        */
    }
}

impl<SampleType: SampleTypeInterface> SampleInterpolationBehavior<SampleType> for delay_line_interpolation_types::Thiran {

    fn interpolate_sample(
        delay_line: &mut DelayLine<SampleType,Self>, 
        channel:    i32

    ) -> SampleType {

        todo!();
        /*
            auto index1 = readPos[(size_t) channel] + delayInt;
            auto index2 = index1 + 1;

            if (index2 >= totalSize)
            {
                index1 %= totalSize;
                index2 %= totalSize;
            }

            auto value1 = bufferData.getSample (channel, index1);
            auto value2 = bufferData.getSample (channel, index2);

            auto output = delayFrac == 0 ? value1 : value2 + alpha * (value1 - v[(size_t) channel]);
            v[(size_t) channel] = output;

            return output;
        */
    }

    fn update_internal_variables(
        delay_line: &mut DelayLine<SampleType,Self>, 
    ) {
        todo!();

        /*
            if (delayFrac < (SampleType) 0.618 && delayInt >= 1)
            {
                delayFrac++;
                delayInt--;
            }

            alpha = (1 - delayFrac) / (1 + delayFrac);
        */
    }
}
