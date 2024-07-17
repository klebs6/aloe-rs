crate::ix!();

pub trait IsKioskMode {

    /**
      | True if the window is in kiosk-mode.
      |
      */
    fn is_kiosk_mode(&self) -> bool;
}
