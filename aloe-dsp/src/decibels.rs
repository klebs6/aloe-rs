/*!
  | This module contains some helpful static
  | methods for dealing with decibel values.
  | 
  | @tags{Audio}
  |
  */
crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_Decibels.h]

pub const DECIBELS_DEFAULT_MINUS_INFINITY_DB: isize = -100;

/**
  | Converts a dBFS value to its equivalent
  | gain level.
  | 
  | A gain of 1.0 = 0 dB, and lower gains map
  | onto negative decibel values. Any decibel
  | value lower than minusInfinityDb will
  | return a gain of 0.
  |
  */
pub fn decibels_to_gain<T: num::Integer + From<isize>>(
    decibels:          T,
    minus_infinity_db: Option<T>
) -> T {

    let minus_infinity_db = minus_infinity_db.unwrap_or(T::from(DECIBELS_DEFAULT_MINUS_INFINITY_DB));

    todo!();
    /*
        return decibels > minusInfinityDb ? std::pow (Type (10.0), decibels * Type (0.05))
                                          : Type();
    */
}

/**
  | Converts a gain level into a dBFS value.
  | 
  | A gain of 1.0 = 0 dB, and lower gains map
  | onto negative decibel values. If the
  | gain is 0 (or negative), then the method
  | will return the value provided as minusInfinityDb.
  |
  */
pub fn gain_to_decibels<T: num::Integer + From<isize>>(
    gain:              T,
    minus_infinity_db: Option<T>

) -> T {

    let minus_infinity_db = minus_infinity_db.unwrap_or(T::from(DECIBELS_DEFAULT_MINUS_INFINITY_DB));

    todo!();
    /*
        return gain > Type() ? jmax (minusInfinityDb, static_cast<Type> (std::log10 (gain)) * Type (20.0))
                             : minusInfinityDb;
    */
}

/**
  | Converts a decibel reading to a string.
  | 
  | By default the returned string will
  | have the 'dB' suffix added, but this
  | can be removed by setting the shouldIncludeSuffix
  | argument to false. If a customMinusInfinityString
  | argument is provided this will be returned
  | if the value is lower than minusInfinityDb,
  | otherwise the return value will be "-INF".
  |
  */
pub fn decibels_to_string<T: num::Integer + From<isize>>(
    decibels:                     T,
    decimal_places:               Option<i32>,
    minus_infinity_db:            Option<T>,
    should_include_suffix:        Option<bool>,
    custom_minus_infinity_string: &str

) -> String {

    let decimal_places = decimal_places.unwrap_or(2);

    let minus_infinity_db = minus_infinity_db.unwrap_or(T::from(DECIBELS_DEFAULT_MINUS_INFINITY_DB));

    let should_include_suffix = should_include_suffix.unwrap_or(true);

    todo!();
    /*
        String s;
        s.preallocateBytes (20);

        if (decibels <= minusInfinityDb)
        {
            if (customMinusInfinityString.isEmpty())
                s << "-INF";
            else
                s << customMinusInfinityString;
        }
        else
        {
            if (decibels >= Type())
                s << '+';

            if (decimalPlaces <= 0)
                s << roundToInt (decibels);
            else
                s << String (decibels, decimalPlaces);
        }

        if (shouldIncludeSuffix)
            s << " dB";

        return s;
    */
}
