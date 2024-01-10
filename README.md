# tauri-example

This is a very basic example on how to integrate an [`aquadoggo`](https://github.com/p2panda/aquadoggo/) node right next to a React frontend using the desktop application framework [Tauri](https://tauri.app/). By embedding a node like that, your application will gain full peer-to-peer and local-first capabilities.

## Requirements

* Rust
* NodeJS

## Usage

```bash
# Install NodeJS dependencies
npm install

# Start Tauri development window
npm run tauri dev
```

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

## Next steps

1. Use [`fishy`](https://github.com/p2panda/fishy) to design, manage and deploy schemas on your node
2. Read more about the p2panda TypeScript SDK [`shirokuma`](https://github.com/p2panda/shirokuma) which will help you to create data
3. Use the node's [GraphQL API](https://p2panda.org/specification/APIs/queries) to query, filter and sort data
4. Check out the configuration possibilities of [`aquadoggo`](https://github.com/p2panda/aquadoggo/)
5. Learn more about [Tauri](https://tauri.app/)

## License

`UNLICENSED`
