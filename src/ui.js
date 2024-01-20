/// Generate a colour from a public key string.
export const intoColour = (publicKeyString) => {
  let stringUniqueHash = [...publicKeyString].reduce((acc, char) => {
    return char.charCodeAt(0) + ((acc << 5) - acc);
  }, 0);
  return `hsl(${stringUniqueHash % 360}, 95%, 50%)`;
};

/// Convert a sprite into an img element and append it to the document body.
export const drawSprite = (
  spriteId,
  blobId,
  posX,
  posY,
  hexColour,
  timestamp,
  description,
) => {
  const body = document.querySelector('body');
  const img = document.createElement('img');
  img.src = `${BLOBS_PATH}${blobId}`;
  img.style.left = `${posX}px`;
  img.style.top = `${posY}px`;
  img.style.zIndex = timestamp;
  img.style.backgroundColor = hexColour;
  img.classList.add('sprite');
  img.alt = description;
  img.id = spriteId;
  body.appendChild(img);
};
