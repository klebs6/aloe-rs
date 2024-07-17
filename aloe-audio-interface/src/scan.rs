crate::ix!();

pub trait ScanMinAndMaxInterleaved {

    /**
      | Used by AudioFormatReader subclasses
      | to scan for min/max ranges in interleaved
      | data.
      |
      */
    fn scan_min_and_max_interleaved(
        &self, 
        channel:              i32,
        start_sample_in_file: i64,
        num_samples:          i64
    ) -> Range<f32>;
}

pub trait CanScanForPlugins {

    /**
      | Returns true if this format needs to
      | run a scan to find its list of plugins.
      |
      */
    fn can_scan_for_plugins(&self) -> bool;
}

pub trait IsTrivialToScan {

    /**
      | Should return true if this format is
      | both safe and quick to scan - i.e. if a
      | file can be scanned within a few milliseconds
      | on a background thread, without actually
      | needing to load an executable.
      |
      */
    fn is_trivial_to_scan(&self) -> bool;
}

pub trait DoesPluginStillExist {

    /**
      | Checks whether this plugin could possibly
      | be loaded.
      | 
      | It doesn't actually need to load it,
      | just to check whether the file or component
      | still exists.
      |
      */
    fn does_plugin_still_exist(&mut self, _0: &PluginDescription) -> bool;
}

pub trait PluginNeedsRescanning {

    /**
      | Returns true if this plugin's version
      | or date has changed and it should be re-checked.
      |
      */
    fn plugin_needs_rescanning(&mut self, _0: &PluginDescription) -> bool;
}
