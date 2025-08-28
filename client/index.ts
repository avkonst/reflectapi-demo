import { client, match } from './generated';

async function main() {
    const c = client('http://localhost:3000');

    const result = await c.books.list({}, {
        authorization: 'password'
    })
    let { items, pagination } = result.unwrap_ok_or_else((e) => {
        throw match(e.unwrap(), {
            Unauthorized: () => 'NotAuthorized',
            LimitExceeded: ({ requested, allowed }) => `Limit exceeded: ${requested} > ${allowed}`,
        });
    });
    console.log(`items: ${items[0]?.author}`);
    console.log(`next cursor: ${pagination.next_cursor}`);
}

main()
    .then(() => console.log('done'))
    .catch((err) => console.error(err));
