import { OperationFields } from "shirokuma";

const SPRITES_SCHEMA_ID =
  "sprites_0020d542c271bf3b5fb8d419584219c8120946cd783a8e48398f831f958ba5ede995";

const SPRITE_IMAGES_SCHEMA_ID =
  "sprite_images_002032604325c478c09ef9c60af330928f9e38a801d5941c3e0b87c5e13fe3ca629e";

const request = async (query) => {
  return fetch(window.GRAPHQL_ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      query: query,
      variables: {},
    }),
  })
    .then((res) => res.json())
    .then((result) => {
      return result;
    });
};

export const createSpriteImage = async (blobId, description) => {
  const timestamp = Math.floor(new Date().getTime() / 1000.0);
  let fields = new OperationFields({
    timestamp,
    description,
  });

  fields.insert("blob", "relation", blobId);

  const spriteImageId = await window.session.create(fields, {
    schemaId: SPRITE_IMAGES_SCHEMA_ID,
  });

  return spriteImageId;
};

export const createSprite = async (
  pos_x,
  pos_y,
  hexColour,
  timestamp,
  imgId
) => {
  let fields = new OperationFields({
    pos_x: Math.floor(pos_x),
    pos_y: Math.floor(pos_y),
    timestamp,
    colour: hexColour,
  });

  fields.insert("img", "relation", imgId);

  const spriteId = await window.session.create(fields, {
    schemaId: SPRITES_SCHEMA_ID,
  });

  return spriteId;
};

export const getSpriteImages = async (first, after) => {
  const options = {
    schema: SPRITE_IMAGES_SCHEMA_ID,
    first,
    after,
    orderBy: `timestamp`,
    orderDirection: `DESC`,
    fields: `{
	  cursor
	  fields {
		description
		timestamp
		blob {
		  meta {
	        documentId
		  }
		}
	  }
	  meta {
		documentId
		viewId
	  }
    }`,
  };
  return await paginatedQuery(options);
};

export const getSprites = async (first, notInSpriteIds, after) => {
  notInSpriteIds = Array.from(notInSpriteIds)
    .map((sprite) => `"${sprite}"`)
    .join();

  const options = {
    schema: SPRITES_SCHEMA_ID,
    first,
    after,
    orderBy: `timestamp`,
    orderDirection: `ASC`,
    meta: `{ documentId: { notIn: [${notInSpriteIds}] } }`,
    fields: `{
      cursor
		fields {
		  colour
	      pos_x
		  pos_y
		  timestamp
		  img {
			fields {
			  description
			  timestamp
				blob {
				  meta {
				    documentId
				  }
				}
			  }
			}
		}
		meta {
		  documentId
		  viewId
		}
	}`,
  };
  return await paginatedQuery(options);
};

export const paginatedQuery = async (options) => {
  const {
    schema,
    first,
    after,
    orderBy,
    orderDirection,
    filter,
    fields,
    meta,
  } = options;

  const queryName = `all_${schema}`;
  const query = `
    query {
      ${queryName}(
        ${first ? `first: ${first},` : ""} 
        ${after ? `after: "${after}",` : ""} 
        ${orderBy ? `orderBy: ${orderBy},` : ""} 
        ${orderDirection ? `orderDirection: ${orderDirection},` : ""} 
        ${filter ? `filter: ${filter},` : ""} 
        ${meta ? `meta: ${meta},` : ""} 
      ) {
        totalCount
        hasNextPage
        endCursor
        documents ${fields}
      }
    }
  `;

  const result = await request(query);
  if (result.errors) {
    console.error("GraphQL errors: ", result.errors);
  }
  return result.data[queryName];
};
