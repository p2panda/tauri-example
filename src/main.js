import { invoke } from '@tauri-apps/api/tauri';
import { KeyPair, Session } from 'shirokuma';
import {
  createSprite,
  createSpriteImage,
  getSprites,
  getSpriteImages,
} from './queries';
import { intoColour, drawSprite } from './ui';
import pandaGif from './assets/panda.gif'

/// Local storage key for our private key.
const LOCAL_STORAGE_KEY = 'privateKey';

/// Initiate some global constants.
const init = async () => {
  /// Get http port of the embedded aquadoggo node by invoking a tauri command.
  const HTTP_PORT = await invoke('http_port_command');
  window.HTTP_PORT = HTTP_PORT;

  /// Address of local node.
  window.NODE_ADDRESS = `http://localhost:${HTTP_PORT}/`;

  /// Path to the blobs HTTP endpoint.
  window.BLOBS_PATH = `${NODE_ADDRESS}blobs/`;

  /// GraphQL endpoint.
  window.GRAPHQL_ENDPOINT = NODE_ADDRESS + 'graphql';
};

/// Generate a new KeyPair or retrieve existing one from local storage.
const getKeyPair = () => {
  const privateKey = window.localStorage.getItem(LOCAL_STORAGE_KEY);
  if (privateKey) {
    return new KeyPair(privateKey);
  }

  const keyPair = new KeyPair();
  window.localStorage.setItem(LOCAL_STORAGE_KEY, keyPair.privateKey());
  return keyPair;
};

/// Get the latest sprite image, or upload our own if none are found.
const getLatestSpriteImage = async () => {
  // Check the cache first!
  if (window.LATEST_SPRITE) {
    return window.LATEST_SPRITE;
  }

  // Query the node for a collection of max 1 sprite image.
  let latestSpriteImages = await getSpriteImages(1);

  // If the totalCount is zero then nobody published any sprite images yet and we should do it.
  if (latestSpriteImages.totalCount === 0) {
    console.log("No sprite images found, uploading 'panda.gif'");

    // Fetch a cute panda gif.
    const data = await fetch(pandaGif);
    const blob = await data.blob();

    // Publish it to the node as a blob_v1 document.
    const blobId = await window.session.createBlob(blob);

    // Now create the sprite image document using the blob id we got in the previous step. We add a
    // description which can be used in the image element's alt text later.
    await createSpriteImage(
      blobId,
      "A cute cartoon panda standing on it's back legs lifting it's arms up and down",
    );
  }

  // It might take a few milliseconds for the document to be ready.
  while (latestSpriteImages.totalCount === 0) {
    latestSpriteImages = await getSpriteImages(1);
  }

  // Set the cache and return the sprite image document.
  const latestSpriteImage = latestSpriteImages.documents[0];
  window.LATEST_SPRITE = latestSpriteImage;
  return latestSpriteImage;
};

/// Request any new sprites from the node and append them to the document body.
const drawSprites = async () => {
  // Query any existing sprite elements by class and collect their ids. These are used in the
  // GraphQL query below to exclude them from the collection results.
  let currentSprites = document.querySelectorAll('.sprite');
  currentSprites = Array.from(currentSprites).map((sprite) => sprite.id);

  // Initial pagination values.
  let hasNextPage = true;
  let endCursor = null;
  let documents = [];

  while (hasNextPage) {
    // Query the node for any new sprites, passing in the current sprites so they can be
    // filtered out of the results.
    const response = await getSprites(10, currentSprites, endCursor);
    ({ hasNextPage, endCursor, documents } = response);

    // For each returned sprite, append them to the current document's body.
    for (let sprite of documents) {
      const { pos_x, pos_y, img, colour, timestamp } = sprite.fields;
      const { blob, description } = img.fields;
      drawSprite(
        sprite.meta.documentId,
        blob.meta.documentId,
        pos_x,
        pos_y,
        colour,
        timestamp,
        description,
      );
    }
  }
};

/// Create, publish and draw a sprite every time the mouse is clicked, yeh!
const onClickCreateSprite = async (e) => {
  const spriteImage = await getLatestSpriteImage();
  // Derive a unique deterministic colour from our public key.
  const colour = intoColour(getKeyPair().publicKey());
  // Get a unix timestamp for now.
  const timestamp = Math.floor(new Date().getTime() / 1000.0);
  // Create the sprite.
  const spriteId = await createSprite(
    e.x,
    e.y,
    colour,
    timestamp,
    spriteImage.meta.documentId,
  );

  // Draw the sprite straight away.
  drawSprite(
    spriteId,
    spriteImage.fields.blob.meta.documentId,
    e.x,
    e.y,
    colour,
    timestamp,
  );
};

export const main = async () => {
  // Initiate some global constants.
  await init();

  // Get or generate a new key pair.
  const keyPair = getKeyPair();
  console.log('You are: ', keyPair.publicKey());

  // Open a long running connection to a p2panda node and configure it so all
  // calls in this session are executed using that key pair
  window.session = new Session(GRAPHQL_ENDPOINT).setKeyPair(keyPair);

  // Get a sprite image we will use when creating sprites.
  const spriteImage = await getLatestSpriteImage();

  // Set the cursor style to be a cute sprite image.
  const body = document.querySelector('body');
  // Published blobs are served from a HTTP endpoint so we can request it from the local node by
  // it's document id.
  body.style.cursor = `url("${BLOBS_PATH}${spriteImage.fields.blob.meta.documentId}"), pointer`;

  // Set onclick handler on body which creates and draws a new sprite.
  body.onclick = onClickCreateSprite;

  // Set an interval timer to draw any new sprites every 1 seconds.
  setInterval(async () => {
    await drawSprites();
  }, 1000);
};
