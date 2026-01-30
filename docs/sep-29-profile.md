# SEP-29 On-Chain Profiles

Chioma stores user profiles on-chain using SEP-29 account data. There is no
off-chain profile storage in this flow; the only value stored on-chain is a
CID/hash reference.

## Account Data Key

- Default key: `chioma_profile`
- Override key via `STELLAR_PROFILE_DATA_KEY` (must be <= 64 bytes)

## SCVal Map Schema

The account data value is a base64-encoded SCVal `Map` with these entries:

- `version` (Symbol): `"1.0"`
- `type` (U32): account type mapping (1 landlord, 2 tenant, 3 agent)
- `updated` (U64): unix timestamp in seconds
- `data_hash` (Bytes): UTF-8 bytes of the CID/hash string

## Validation Rules

- `type` must be in `1..=3`.
- `data_hash` must be non-empty and <= 128 bytes.
- `updated` is set at write time using the current unix timestamp.

## Setting Profile Data

1. Build the SCVal map with the schema above.
2. Encode the map to base64 XDR.
3. Submit a Stellar `manage_data` operation setting the account data key to the
   base64 XDR value.

## Getting Profile Data

1. Fetch the account via Horizon and read the `data` entry for the profile key.
2. Decode the base64 XDR back into the SCVal map.

## Environment Variables

- `STELLAR_HORIZON_URL`: Horizon base URL (default: testnet)
- `STELLAR_PROFILE_DATA_KEY`: account data key (default: `chioma_profile`)
