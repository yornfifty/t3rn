// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Option, Struct, Type, Vec, u16, u64, u8 } from '@polkadot/types';
import type { ChainId, Parameter } from '@polkadot/types/interfaces/bridges';
import type { AccountId, Balance, Hash } from '@polkadot/types/interfaces/runtime';
import type { RuntimeVersion } from '@polkadot/types/interfaces/state';

/** @name CircuitOutboundMessage */
export interface CircuitOutboundMessage extends Struct {
  readonly name: Bytes;
  readonly module_name: Bytes;
  readonly method_name: Bytes;
  readonly sender: Option<Bytes>;
  readonly target: Option<Bytes>;
  readonly arguments: Vec<Bytes>;
  readonly expected_output: Vec<GatewayExpectedOutput>;
  readonly extra_payload: Option<ExtraMessagePayload>;
}

/** @name Compose */
export interface Compose extends Struct {
  readonly name: Bytes;
  readonly code_txt: Bytes;
  readonly exec_type: Bytes;
  readonly dest: AccountId;
  readonly value: Balance;
  readonly bytes: Bytes;
  readonly input_data: Bytes;
}

/** @name ContractActionDesc */
export interface ContractActionDesc extends Struct {
  readonly action_id: Hash;
  readonly target_id: Option<TargetId>;
  readonly to: Option<AccountId>;
}

/** @name CryptoAlgo */
export interface CryptoAlgo extends Enum {
  readonly isEd25519: boolean;
  readonly isSr25519: boolean;
  readonly isEcdsa: boolean;
}

/** @name ExtraMessagePayload */
export interface ExtraMessagePayload extends Struct {
  readonly signer: Bytes;
  readonly module_name: Bytes;
  readonly method_name: Bytes;
  readonly call_bytes: Bytes;
  readonly signature: Bytes;
  readonly extra: Bytes;
  readonly tx_signed: Bytes;
  readonly custom_payload: Option<Bytes>;
}

/** @name GatewayABIConfig */
export interface GatewayABIConfig extends Struct {
  readonly block_number_type_size: u16;
  readonly hash_size: u16;
  readonly hasher: HasherAlgo;
  readonly crypto: CryptoAlgo;
  readonly address_length: u16;
  readonly value_type_size: u16;
  readonly decimals: u16;
  readonly structs: Vec<StructDecl>;
}

/** @name GatewayExpectedOutput */
export interface GatewayExpectedOutput extends Enum {
  readonly isStorage: boolean;
  readonly asStorage: {
    readonly key: Vec<Bytes>;
    readonly value: Vec<Option<Bytes>>;
  } & Struct;
  readonly isEvents: boolean;
  readonly asEvents: {
    readonly signatures: Vec<Bytes>;
  } & Struct;
  readonly isExtrinsic: boolean;
  readonly asExtrinsic: {
    readonly block_height: Option<u64>;
  } & Struct;
  readonly isOutput: boolean;
  readonly asOutput: {
    readonly output: Bytes;
  } & Struct;
}

/** @name GatewayGenesisConfig */
export interface GatewayGenesisConfig extends Struct {
  readonly modules_encoded: Option<HasherAlgo>;
  readonly signed_extension: Option<Bytes>;
  readonly runtime_version: RuntimeVersion;
  readonly extrinsics_version: u8;
  readonly genesis_hash: Bytes;
}

/** @name GatewayPointer */
export interface GatewayPointer extends Struct {
  readonly id: ChainId;
  readonly vendor: GatewayVendor;
  readonly gateway_type: GatewayType;
}

/** @name GatewayType */
export interface GatewayType extends Enum {
  readonly isInternal: boolean;
  readonly isExternal: boolean;
}

/** @name GatewayVendor */
export interface GatewayVendor extends Enum {
  readonly isSubstrate: boolean;
  readonly isEthereum: boolean;
}

/** @name HasherAlgo */
export interface HasherAlgo extends Enum {
  readonly isBlake2: boolean;
  readonly isKeccak256: boolean;
}

/** @name ProofTriePointer */
export interface ProofTriePointer extends Enum {
  readonly isState: boolean;
  readonly isTransaction: boolean;
  readonly isReceipts: boolean;
}

/** @name StructDecl */
export interface StructDecl extends Struct {
  readonly name: Type;
  readonly fields: Vec<Parameter>;
  readonly offsets: Vec<u16>;
}

/** @name TargetId */
export interface TargetId extends ChainId {}

export type PHANTOM_PRIMITIVES = 'primitives';
