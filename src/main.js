import { KeyPair, Session } from "shirokuma";

/// Local storage key for our private key.
const LOCAL_STORAGE_KEY = "privateKey";

/// Address of local node.
const NODE_ADDRESS = `http://localhost:2020/`;

/// GraphQL endpoint.
const GRAPHQL_ENDPOINT = NODE_ADDRESS + "graphql";

/// Generate a new KeyPair or retrieve existing one from local storage.
export const getKeyPair = () => {
	const privateKey = window.localStorage.getItem(LOCAL_STORAGE_KEY);
	if (privateKey) {
		return new KeyPair(privateKey);
	}

	const keyPair = new KeyPair();
	window.localStorage.setItem(LOCAL_STORAGE_KEY, keyPair.privateKey());
	return keyPair;
};

export const main = async () => {
	const keyPair = getKeyPair();

	const publicKey = keyPair.publicKey();
	console.log(`You are: ${publicKey}`);

	// Open a long running connection to a p2panda node and configure it so all
	// calls in this session are executed using that key pair
	window.session = new Session(GRAPHQL_ENDPOINT).setKeyPair(keyPair);
};
