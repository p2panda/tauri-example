import { invoke } from "@tauri-apps/api/tauri";
import { KeyPair, Session } from "shirokuma";
import {
	createSprite,
	createSpriteImage,
	getSprites,
	getSpriteImages,
} from "./queries";

import pandaUrl from "./panda.gif";

/// Local storage key for our private key.
const LOCAL_STORAGE_KEY = "privateKey";

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
const getSpriteImage = async () => {
	const latestSpriteImages = await getSpriteImages(1);
	if (latestSpriteImages.totalCount === 0) {
		console.log("No sprite images found, uploading 'panda.gif'");
		const data = await fetch(pandaUrl);
		const blob = await data.blob();
		const blobId = await window.session.createBlob(blob);
		const spriteImageId = await createSpriteImage(
			blobId,
			"A cute cartoon panda standing on it's back legs lifting it's arms up and down"
		);
		return [blobId, spriteImageId];
	} else {
		const latestSpriteImage = latestSpriteImages.documents[0];
		const { fields, meta } = latestSpriteImage;
		return [fields.blob.meta.documentId, meta.documentId];
	}
};

/// Request any new sprites from the node and append them to the document body.
const drawSprites = async () => {
	// Query any existing sprite elements by class and collect their ids. These are used in the
	// GraphQL query below to exclude them from the collection results.
	let currentSprites = document.querySelectorAll(".sprite");
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
			const { pos_x, pos_y, img } = sprite.fields;
			const { blob, description } = img.fields;
			drawSprite(
				sprite.meta.documentId,
				blob.meta.documentId,
				pos_x,
				pos_y,
				description
			);
		}
	}
};

/// Convert a sprite into an img element and append it to the document body.
const drawSprite = (spriteId, blobId, posX, posY, description) => {
	const body = document.querySelector("body");
	const img = document.createElement("img");
	img.src = `${BLOBS_PATH}${blobId}`;
	img.style.position = "fixed";
	img.style.left = `${posX}px`;
	img.style.top = `${posY}px`;
	img.classList.add("sprite");
	img.alt = description;
	img.id = spriteId;
	body.appendChild(img);
};

export const main = async () => {
	const HTTP_PORT = await invoke("http_port_command");
	window.HTTP_PORT = HTTP_PORT;

	/// Address of local node.
	window.NODE_ADDRESS = `http://localhost:${HTTP_PORT}/`;

	/// Path to the blobs HTTP endpoint.
	window.BLOBS_PATH = `${NODE_ADDRESS}blobs/`;

	/// GraphQL endpoint.
	window.GRAPHQL_ENDPOINT = NODE_ADDRESS + "graphql";

	// Get or generate a new key pair.
	const keyPair = getKeyPair();

	console.log("You are: ", keyPair.publicKey())

	// Open a long running connection to a p2panda node and configure it so all
	// calls in this session are executed using that key pair
	window.session = new Session(GRAPHQL_ENDPOINT).setKeyPair(keyPair);

	// Set an interval timer to draw any new sprites every 2 seconds.
	setInterval(async () => {
		await drawSprites();
	}, 1000);

	// Get a sprite image we will use when creating sprites.
	const [blobId, spriteImageId] = await getSpriteImage();

	const body = document.querySelector("body");

	// Set the cursor style to be a cute sprite image.
	body.style.cursor = `url("${BLOBS_PATH}${blobId}"), pointer`;

	// Set onclick handler on body which creates and draws a new sprite.
	body.onclick = async (e) => {
		const spriteId = await createSprite(e.x, e.y, spriteImageId);
		drawSprite(spriteId, blobId, e.x, e.y);
	};
};
