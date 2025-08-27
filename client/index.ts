import { match } from 'match';
import { client } from './generated';

async function main() {
    const c = client('http://localhost:3000');

    const result = await c.pets.create({
        name: 'Bobby',
        kind: {
            type: 'dog',
            breed: 'Golden Retriever'
        },
        age: 1,
        behaviors: [],
    }, {
        authorization: 'password'
    })
    let { id } = result.unwrap_ok_or_else((e) => {
        throw match(e.unwrap(), {
            Conflict: () => 'Conflict',
            NotAuthorized: () => 'NotAuthorized',
            InvalidIdentityCharacter: ({ char }) => `invalid character: ${char}`,
        });
    });
    console.log(`created id: ${id}`);
}

main()
    .then(() => console.log('done'))
    .catch((err) => console.error(err));
