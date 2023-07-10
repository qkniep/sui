// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { beforeAll, describe, expect, it } from "vitest";

import {
  Ed25519Keypair,
  getExecutionStatusType,
  getTransactionDigest,
  JsonRpcProvider,
  localnetConnection,
  RawSigner,
  TransactionBlock,
} from "../../src";

const SHARDS = 32;
const USERS_PER_SHARD = 4;
const TOTAL_USERS = SHARDS * USERS_PER_SHARD;

type User = {
  keypair: Ed25519Keypair;
  address: string;
  signer: RawSigner;
};

describe("Transaction Builders", () => {
  let provider: JsonRpcProvider;
  let users: User[] = [];

  beforeAll(async () => {
    provider = new JsonRpcProvider(localnetConnection);
    for (let i = 0; i < TOTAL_USERS; i++) {
      const keypair = new Ed25519Keypair();
      const user = {
        keypair,
        address: keypair.getPublicKey().toSuiAddress(),
        signer: new RawSigner(keypair, provider),
      };
      await provider.requestSuiFromFaucet(user.address);
      users.push(user);
    }
  });

  it("SplitCoins + TransferObjects", async () => {
    for (let i = 0; i < 1000; i++) {
      let results = [];

      for (let shard = 0; shard < 10; shard++) {
        //const shard = Math.floor(Math.random() * SHARDS);
        const userIdx = USERS_PER_SHARD * shard +
          Math.floor(Math.random() * USERS_PER_SHARD);
        let otherUserIdx = userIdx;
        while (otherUserIdx === userIdx) {
          otherUserIdx = USERS_PER_SHARD * shard +
            Math.floor(Math.random() * USERS_PER_SHARD);
        }
        const user = users[userIdx];
        const otherUser = users[otherUserIdx];
        const tx = new TransactionBlock();
        const [coin] = tx.splitCoins(tx.gas, [tx.pure(1000)]);
        tx.transferObjects([coin], tx.pure(otherUser.address));
        results.push(validateTransaction(user.signer, tx));
      }

      await Promise.all(results);
    }
  });

  /*it("TransferObjects gas object", async () => {
    for (let i = 0; i < 100; i++) {
      const shard = Math.floor(Math.random() * 10);
      const userIdx = 10 * shard + Math.floor(Math.random() * 10);
      let otherUserIdx = userIdx;
      while (otherUserIdx === userIdx) {
        otherUserIdx = 10 * shard + Math.floor(Math.random() * 10);
      }
      const user = users[userIdx];
      const otherUser = users[otherUserIdx];
      const tx = new TransactionBlock();
      tx.transferObjects([tx.gas], tx.pure(otherUser.address));
      await validateTransaction(user.signer, tx);
    }
  });*/
});

async function validateTransaction(signer: RawSigner, tx: TransactionBlock) {
  const localDigest = await signer.getTransactionBlockDigest(tx);
  const result = await signer.signAndExecuteTransactionBlock({
    transactionBlock: tx,
    options: {
      showEffects: true,
    },
  });
  expect(localDigest).toEqual(getTransactionDigest(result));
  expect(getExecutionStatusType(result)).toEqual("success");
}
