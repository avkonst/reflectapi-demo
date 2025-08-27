type UnionToIntersection<U> = (U extends any ? (k: U) => unknown : never) extends (
    k: infer I
) => void
    ? I
    : never;

// How it works:
// type Step0 =
// 	{ a: { x: string } }
// 	| { b: number }
// 	| 'c'

// First transformation:
// Turn the `string` variant into an object variant so each variant of the union is now uniform.
// type AfterStep1 =
// 	{ a: { x: string } }
// 	| { b: number }
// 	| { c: {} }

type Step1<T> = T extends object ? T : T extends string ? { [K in T]: unknown } : never;

// Second transformation:
// We want to merge the unions into a single object type.
// This is implemented by turning the union into an intersection.
// type AfterStep2 = {
// 	a: { x: string }
// 	b: number
// 	c: {}
// }

type Step2<T> = UnionToIntersection<T>;

// Final transformation:
// Turn each value type into a function that takes the value type as an argument and returns the result type.
// type AfterStep3<R> = {
// 	a: (arg: { x: string }) => R
// 	b: (arg: number) => R
// 	c: (arg: {}) => R
// }

type Step3<T, R> = { [K in keyof T]: (arg: T[K]) => R };

type Cases<T, R> = Step3<Step2<Step1<T>>, R>;

type CasesNonExhaustive<T, R> = Partial<Cases<T, R>>;

/**
 * Ergonomically and exhaustively handle all possible cases of a discriminated union in the externally tagged representation (https://serde.rs/enum-representations.html).
 * See tests for examples.
 * Talk to @andy if you have issues or question or just change it however you wish :)
 * */
export function match<T extends object | string, R>(value: T, cases: Cases<T, R>): R;
export function match<T extends object | string, R>(
    value: T,
    cases: CasesNonExhaustive<T, R>,
    otherwise: () => R
): R;
export function match<T extends object | string, R>(
    value: T,
    cases: Cases<T, R> | CasesNonExhaustive<T, R>,
    otherwise?: () => R
): R {
    const branches = cases as Record<string, (arg: any) => R>;
    switch (typeof value) {
        case 'string':
            if (value in branches) {
                return branches[value]({});
            }

            if (!otherwise) {
                throw new Error('otherwise must exist for non-exhaustive match');
            }
            return otherwise();

        case 'object': {
            if (Object.keys(value).length !== 1) {
                throw new Error(
                    'Expected object with exactly one key, see serde documentation for externally tagged enums above'
                );
            }

            const [k, v] = Object.entries(value)[0];
            if (k in branches) {
                return branches[k](v);
            }
            if (!otherwise) {
                throw new Error('otherwise must exist for non-exhaustive match');
            }
            return otherwise();
        }

        default:
            throw new Error('unreachable');
    }
}
