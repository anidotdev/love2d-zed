# Love2D Extension for Zed

The extension automatically detects LÖVE projects, then configures Lua Language Server and installs the correct LuaCATS library for your project, and removes the need for manual setup.

## Features

- Automatic LÖVE project detection
- Automatic Lua Language Server installation
- Automatic LuaLS configuration
- Automatic LÖVE version detection
  - 11.5
  - 11.4
  - 11.3
- Automatically loads the correct LuaCATS library
- Removes `Undefined global 'love'` diagnostics
- Zero configuration required

## Installation

Install the extension from the Zed Extensions marketplace.

Once installed, simply open a LÖVE project.

The extension will:

1. Detect that the project is a LÖVE game.
2. Detect the project's LÖVE version.
3. Configure Lua Language Server.
4. Load the matching LuaCATS library automatically.

No additional configuration is required.

## Supported Versions

| LÖVE Version | Supported |
|--------------|-----------|
| 11.5 | ✅ |
| 11.4 | ✅ |
| 11.3 | ✅ |

## How It Works

The extension reads your project's `conf.lua` to determine the target LÖVE version and configures Lua Language Server accordingly.

If no version can be determined, the latest supported version is used as the default.

## Roadmap

Planned improvements include:

- Asset path autocomplete
- Better project indexing
- Additional LÖVE API support
- Improved diagnostics
- More version support as new releases become available

## Contributing

If you find a bug or have an idea for improving the extension, feel free to open an issue.

## License

MIT
