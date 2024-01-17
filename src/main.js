import { KeyPair, Session } from "shirokuma";

export const main = () => {
	const keyPair = new KeyPair();
	const publicKey = keyPair.publicKey();
	console.log(`You are: ${publicKey}`);
};
