/* tslint:disable */
/* eslint-disable */
/**
 * IB REST API
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.29.0
 * Contact: api@interactivebrokers.com
 *
 * NOTE: Manual stub. The OpenAPI oneOf for `ClientPublicKeySetKeysInner` has
 * alternatives the upstream generator can't render into a valid TS union,
 * so we fall back to a permissive `any` passthrough here. Downstream callers
 * that touch `.kty` should cast the value to their preferred JWK shape.
 */

export type ClientPublicKeySetKeysInner = any;

export function ClientPublicKeySetKeysInnerFromJSON(json: any): ClientPublicKeySetKeysInner {
    return json;
}

export function ClientPublicKeySetKeysInnerFromJSONTyped(
    json: any,
    _ignoreDiscriminator: boolean,
): ClientPublicKeySetKeysInner {
    return json;
}

export function ClientPublicKeySetKeysInnerToJSON(value: any): any {
    return value;
}

export function ClientPublicKeySetKeysInnerToJSONTyped(
    value?: ClientPublicKeySetKeysInner | null,
    _ignoreDiscriminator: boolean = false,
): any {
    return value ?? undefined;
}
