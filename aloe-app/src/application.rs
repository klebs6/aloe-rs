crate::ix!();

pub trait GetApplicationName {

    /**
      | Returns the application's name.
      |
      */
    fn get_application_name(&mut self) -> String;
}

pub trait GetApplicationVersion {

    /**
      | Returns the application's version
      | number.
      |
      */
    fn get_application_version(&mut self) -> String;
}
