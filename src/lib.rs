use zed_extension_api as zed;

struct Love2DExtension;

impl zed::Extension for Love2DExtension {}

zed::register_extension!(Love2DExtension);
