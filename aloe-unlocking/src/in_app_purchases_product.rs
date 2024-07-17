crate::ix!();

/**
  | Represents a product available in the
  | store.
  |
  */
pub struct InAppPurchasesProduct
{
    /**
      | InAppPurchasesProduct ID (also known as SKU) that uniquely
      | identifies a product in the store.
      |
      */
    identifier:   String,

    /**
      | Title of the product.
      |
      */
    title:        String,

    /**
      | Description of the product.
      |
      */
    description:  String,

    /**
      | Price of the product in local currency.
      |
      */
    price:        String,

    /**
      | Price locale.
      |
      */
    price_locale: String,
}

