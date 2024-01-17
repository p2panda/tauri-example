# p2panda-tauri-example

This is a very basic example on how to integrate an
[`aquadoggo`](https://github.com/p2panda/aquadoggo/) node right next to a React frontend using
the desktop application framework [Tauri](https://tauri.app/). By embedding a node like that, your
application will gain full peer-to-peer and local-first capabilities.

## Requirements

- Rust
- NodeJS

## Usage

```bash
# Install NodeJS dependencies
npm install

# Start Tauri development window
npm run tauri dev
```

## Storage

Tauri chooses where app data is stored based on expected platform specific locations.

Linux: Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.  
macOS: Resolves to `$HOME/Library/Application Support`.  
Windows: Resolves to `{FOLDERID_LocalAppData}`.  

Data for both the WebView and the rust code is persisted to the sub-folder `p2panda-tauri-example`.

## Identity

On first run a new ed25519 key pair is generated and saved to a file named `private-key.txt` in
the app data directory. From this point on it is re-used each time you start the application, this
is the identity of the embedded `aquadoggo` node (which is different from a client's identity).

## Configuration

The embedded `aquadoggo` node can be configured via a `config.toml` file. On initial startup
a default config is generated and stored in the app data directory.

See config file comments for detailed instructions on the extensive configuration options. Visit  
the [`aquadoggo` cli](https://github.com/p2panda/aquadoggo/tree/main/aquadoggo_cli) for further
information and examples.

## Compile release binaries

With the help of a github workflow task binaries for Windows, MacOS and Linux are compiled and
pushed to a new release whenever a you push a new tag. For example:

```bash
# Create a new tag
git tag v0.1.0

# Push the current code and tags
git push --tags
```

This will trigger the `ci` to compile binaries, create a new release (`v0.1.0`) and upload the
binaries to the release assets.

## Development

### Application data

In development mode (`npm run tauri dev`) application data is persisted to `src-tauri/tmp`. If
you want to wipe all your app data and start fresh, you can delete this directory and all it's
content.

## Next steps

1. Use [`fishy`](https://github.com/p2panda/fishy) to design, manage and deploy schemas on your node
2. Read more about the p2panda TypeScript SDK [`shirokuma`](https://github.com/p2panda/shirokuma) which will help you to create data
3. Use the node's [GraphQL API](https://p2panda.org/specification/APIs/queries) to query, filter and sort data
4. Check out the configuration possibilities of [`aquadoggo`](https://github.com/p2panda/aquadoggo/)
5. Learn more about [Tauri](https://tauri.app/)

## License

`UNLICENSED`
