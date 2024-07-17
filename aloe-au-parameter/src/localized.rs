crate::ix!();

pub fn create_localized_string_for_parameter_value(
    in_parameter_value: f64,
    in_parameter:       *const CAAUParameter,
    in_digits:          u32

) -> CFStringRef {

    create_localized_string_for_parameter_value_with_min_digits(in_parameter_value, in_parameter, in_digits, 1)
}

pub fn value_for_localized_parameter_string(
    string:       CFStringRef,
    in_parameter: *const CAAUParameter

) -> f64 {
    
    todo!();
        /*
            CFLocaleRef currentLocale = CFLocaleCopyCurrent();
        CFNumberFormatterRef numberFormatter = CFNumberFormatterCreate (NULL, currentLocale, kCFNumberFormatterDecimalStyle);

        double value = 0;
        Boolean worked = CFNumberFormatterGetValueFromString (numberFormatter, string, NULL, kCFNumberDoubleType, &value);

        CFRelease(currentLocale);
        CFRelease(numberFormatter);

        if (worked)
            return value;
        else {
            AudioUnitParameterInfo info = inParameter->ParamInfo();
            return info.defaultValue;
        }
        */
}

pub fn create_localized_string_for_parameter_value_with_min_digits(
    in_parameter_value: f64,
    in_parameter:       *const CAAUParameter,
    in_digits:          u32,
    min_digits:         u32

) -> CFStringRef {

    todo!();
        /*
            if (!inParameter) return nil;

        AudioUnitParameterInfo info = inParameter->ParamInfo();
        int pow10;

        switch (info.unit) {
            case kAudioUnitParameterUnit_Hertz:
                // number of significant digits based on value
                pow10 = int(log10(fmax(inParameterValue, .000001)));
                break;
            default:
                // number of significant digits based on parameter range
                pow10 = int(log10(fmax(double(info.maxValue - info.minValue), .000001)));
                break;
        }

        // pow10    range           nDigitsAfterDecimal
        //  -2      .0100-.0999     4
        //  -1      .100-.999       3
        //  0       1.00-9.99       2
        //  1       10.0-99.9       1
        //  2       100-999         0
        //  3       1000-9990       -1
        //  4       10000-99900     -2

        int nDigitsAfterDecimal = inDigits - (pow10 + 1);
        if (nDigitsAfterDecimal < 0)
            nDigitsAfterDecimal = 0;    // the least number of digits possible is zero

        if (info.flags & kAudioUnitParameterFlag_IsHighResolution)
            nDigitsAfterDecimal = 4;

        CFLocaleRef currentLocale = CFLocaleCopyCurrent();
        CFNumberFormatterRef numberFormatter = CFNumberFormatterCreate (NULL, currentLocale, kCFNumberFormatterDecimalStyle);

        CFNumberRef maxFractionDigits = CFNumberCreate (NULL, kCFNumberIntType, &nDigitsAfterDecimal);

        if (nDigitsAfterDecimal > 0)
            nDigitsAfterDecimal = minDigits;

        CFNumberRef minFractionDigits = CFNumberCreate (NULL, kCFNumberIntType, &nDigitsAfterDecimal);

        CFNumberFormatterSetProperty (numberFormatter, kCFNumberFormatterMinFractionDigits, minFractionDigits);
        CFNumberFormatterSetProperty (numberFormatter, kCFNumberFormatterMaxFractionDigits, maxFractionDigits);
        CFStringRef formattedNumberString = CFNumberFormatterCreateStringWithValue (NULL, numberFormatter, kCFNumberDoubleType, &inParameterValue);

        CFRelease(currentLocale);
        CFRelease(numberFormatter);
        CFRelease(maxFractionDigits);
        CFRelease(minFractionDigits);

        return formattedNumberString;
        */
}
