import { match } from 'match';
import { client } from './generated';

async function main() {
    const c = client('http://localhost:3000');
    (
        await c.pets.create({
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
    ).unwrap_ok_or_else((e) =>
        match(e.unwrap(), {
            Conflict: () => 'Conflict',
            NotAuthorized: () => 'NotAuthorized',
            InvalidIdentityCharacter: ({ char }) => `invalid character: ${char}`,
        })
    );
}

main()
    .then(() => console.log('done'))
    .catch((err) => console.error(err));
