# p2panda-tauri-example

<div align="center">
  <img src="https://raw.githubusercontent.com/p2panda/.github/main/assets/tauri-example-screenshot.png" width="600" />
</div>

This is an example on how to integrate an [`aquadoggo`](https://github.com/p2panda/aquadoggo/)
node right next to a web frontend using the desktop application framework
[Tauri](https://tauri.app/). By embedding a node like that, your application will gain full
peer-to-peer and local-first capabilities.

On the front-end there is a simple JavaScript web app which demonstrates how to interact with the
embedded node using `shirokuma` to publish documents, `GraphQL` for queries, and `http`
endpoint for requesting blobs.

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

Tauri recommends where app data should be stored based on expected platform specific locations.

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

In development mode (`npm run tauri dev`) data for each app instance is not persisted.

### Schema

Schema are deployed to the node automatically on initial startup from a `schema.lock` file located
in the tauri `resources/schemas` directory. This file was created with the CLI tool
[`fishy`](https://github.com/p2panda/fishy).

When you replace these schema with the ones for your own application, don't forget to update `allow_schema_ids`
in your `config.toml` as well!  

### Logging

You can enable and configure logging using the `RUST_LOG` environment variable like so:

```bash
# Show debug logs emitted by `aquadoggo`
RUST_LOG="aquadoggo=debug" npm run tauri dev

# Show info logs emitted by all crates used in `aquadoggo`
RUST_LOG="info" npm run tauri dev
```

### Multiple instances

To test out p2p discovery and replication locally you can run multiple instances of the app
simultaneously. You do this like so:

```bash
# Start up the front-end dev server, this same endpoint is shared across app instances.
npm run dev

# Start the tauri app selecting random port http port for the node. Run this many times to launch more peers.
npm run peer

# You can choose the http port manually too
HTTP_PORT=1234 npm run peer
```

## Next steps

1. Use [`fishy`](https://github.com/p2panda/fishy) to design, manage and deploy schemas on your node
2. Read more about the p2panda TypeScript SDK [`shirokuma`](https://github.com/p2panda/shirokuma) which will help you to create data
3. Use the node's [GraphQL API](https://p2panda.org/specification/APIs/queries) to query, filter and sort data
4. Check out the configuration possibilities of [`aquadoggo`](https://github.com/p2panda/aquadoggo/)
5. Learn more about [Tauri](https://tauri.app/)

## License

`UNLICENSED`
