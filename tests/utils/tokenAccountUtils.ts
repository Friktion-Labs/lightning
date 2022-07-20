import * as anchor from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor";
import { getAccount, getMint } from "@solana/spl-token";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import assert from "assert";
import {
  initNewAssociatedTokenAccountIfNeeded,
  initNewTokenAccount,
} from "./helpers";

export const Side = {
  Bid: { bid: {} },
  Ask: { ask: {} },
};

export const OrderType = {
  Limit: { limit: {} },
  ImmediateOrCancel: { immediateOrCancel: {} },
  PostOnly: { postOnly: {} },
};

export const SelfTradeBehavior = {
  DecrementTake: { decremenTtake: {} },
  CancelProvide: { cancelProvide: {} },
  AbortTransaction: { abortTransaction: {} },
};

export const textEncoder = new TextEncoder();

export const getRandomInt = function (max: number) {
  return Math.floor(Math.random() * max);
};

// X = total supply of vault tokens before deposit
// D = deposited value in base asset (e.g BTC)
// Y = total value in vault before deposit in base asset (e.g BTC)
export const getLpTokens = async (x: number, d: number, y: number) => {
  x = x == 0 ? 1 : x;
  const base_value = (y + d) * x;
  return d / base_value;
};

export const getTokenSupply = async (
  connection: Connection,
  token: PublicKey
) => {
  return new anchor.BN((await getMint(connection, token)).supply.toString());
};

export const createNormalOrAssociated = async (
  provider: AnchorProvider,
  user: any,
  mint: PublicKey,

  payer: Keypair,
  makeAss: boolean = true
) => {
  if (makeAss) {
    return await initNewAssociatedTokenAccountIfNeeded(provider, user, mint);
  } else {
    const { tokenAccount } = await initNewTokenAccount(
      provider.connection,
      user,
      mint,
      payer
    );
    return {
      tokenAccount: tokenAccount.publicKey,
    };
  }
};

export const createTokenAccounts = async (
  provider: AnchorProvider,
  vaultMint: PublicKey,
  underlyingToken: PublicKey,
  quoteToken: PublicKey,
  user: any,
  payer: Keypair
) => {
  const connection = provider.connection;
  if (user.secretKey !== undefined) {
    user = user.publicKey;
  }

  console.log("creating vault token account");

  const { tokenAccount: vaultTokenAccount } = await createNormalOrAssociated(
    provider,
    user,
    vaultMint,
    payer
  );

  console.log("creating underlying token account");

  const { tokenAccount: underlyingTokenAccount } =
    await createNormalOrAssociated(provider, user, underlyingToken, payer);

  console.log("creating quote token account");

  let { tokenAccount: quoteTokenAccount } = await createNormalOrAssociated(
    provider,
    user,
    quoteToken,
    payer
  );

  console.log(
    "vault token balance = " +
      (await getAccount(connection, vaultTokenAccount)).amount
  );
  console.log(
    "underlying token balance = " +
      (await getAccount(connection, underlyingTokenAccount)).amount
  );

  console.log(
    "quote token balance = " +
      (await getAccount(connection, quoteTokenAccount)).amount
  );

  return {
    vaultTokenAccount,
    underlyingTokenAccount,
    quoteTokenAccount,
  };
};
