import type { AnchorProvider, Provider } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import {
  AccountLayout,
  createAssociatedTokenAccountInstruction,
  createInitializeAccountInstruction,
  createMint,
  getAccount,
  getAssociatedTokenAddress,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { sendIns } from "@friktion-labs/friktion-utils";

export const wait = (delayMS: number) =>
  new Promise((resolve) => setTimeout(resolve, delayMS));

export const createUnderlyingAndQuoteMints = async (
  provider: Provider,
  wallet: Keypair,
  mintAuthority: Keypair
) => {
  const underlyingToken = await createMint(
    provider.connection,
    wallet,
    mintAuthority.publicKey,
    null,
    6
  );

  const quoteToken = await createMint(
    provider.connection,
    wallet,
    mintAuthority.publicKey,
    null,
    2
  );
  return {
    quoteToken,
    underlyingToken,
  };
};

export const initNewAssociatedTokenAccountIfNeeded = async (
  provider: AnchorProvider,
  /** The owner for the new mint account */
  owner: PublicKey,
  /** The SPL Token Mint address */
  mint: PublicKey
) => {
  const tokenAccount = await getAssociatedTokenAddress(mint, owner, true);

  try {
    await getAccount(provider.connection, tokenAccount);
  } catch (err) {
    const ix = createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      tokenAccount,
      owner,
      mint
    );
    await sendIns(provider, ix);
  }
  return {
    tokenAccount,
  };
};

export const initNewTokenAccount = async (
  connection: Connection,
  /** The owner for the new mint account */
  owner: PublicKey,
  /** The SPL Token Mint address */
  mint: PublicKey,
  wallet: Keypair,
  targetKeypair?: Keypair
) => {
  let tokenAccount = new Keypair();
  if (targetKeypair) tokenAccount = targetKeypair;
  const transaction = new Transaction();

  const assetPoolRentBalance =
    await connection.getMinimumBalanceForRentExemption(AccountLayout.span);
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: tokenAccount.publicKey,
      lamports: assetPoolRentBalance,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  transaction.add(
    createInitializeAccountInstruction(tokenAccount.publicKey, mint, owner)
  );
  await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet, tokenAccount],
    {
      commitment: "confirmed",
    }
  );
  return {
    tokenAccount,
  };
};

/**
 *
 * TODO: This should be transformed to use associated token program accounts. That will make it easier
 *
 * @param connection
 * @param minter
 * @param mintAuthority
 * @param underlyingToken
 * @param underlyingAmount
 * @param optionMint
 * @param writerTokenMint
 * @param quoteToken
 * @param quoteAmount
 * @returns
 */
export const createMinter = async (
  connection: Connection,
  minter: Keypair,
  mintAuthority: Keypair,
  underlyingMint: PublicKey,
  underlyingAmount: number,
  optionMint: PublicKey,
  writerTokenMint: PublicKey,
  quoteMint: PublicKey,
  quoteAmount = 0
) => {
  const transaction = new Transaction();

  const underlyingAccount = new Keypair();
  const assetPoolRentBalance =
    await connection.getMinimumBalanceForRentExemption(AccountLayout.span);
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: minter.publicKey,
      newAccountPubkey: underlyingAccount.publicKey,
      lamports: assetPoolRentBalance,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  transaction.add(
    createInitializeAccountInstruction(
      underlyingAccount.publicKey,
      underlyingMint,
      minter.publicKey
    )
  );

  const quoteAccount = new Keypair();
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: minter.publicKey,
      newAccountPubkey: quoteAccount.publicKey,
      lamports: assetPoolRentBalance,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  transaction.add(
    createInitializeAccountInstruction(
      quoteAccount.publicKey,
      quoteMint,
      minter.publicKey
    )
  );

  // create an associated token account to hold the options
  const optionAccount = new Keypair();
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: minter.publicKey,
      newAccountPubkey: optionAccount.publicKey,
      lamports: assetPoolRentBalance,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  transaction.add(
    createInitializeAccountInstruction(
      optionAccount.publicKey,
      optionMint,
      minter.publicKey
    )
  );

  // create an associated token account to hold the writer tokens
  const writerTokenAccount = new Keypair();
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: minter.publicKey,
      newAccountPubkey: writerTokenAccount.publicKey,
      lamports: assetPoolRentBalance,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  transaction.add(
    createInitializeAccountInstruction(
      writerTokenAccount.publicKey,
      writerTokenMint,
      minter.publicKey
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [
      minter,
      underlyingAccount,
      quoteAccount,
      optionAccount,
      writerTokenAccount,
    ],
    {
      commitment: "confirmed",
    }
  );

  // mint underlying tokens to the minter's account
  await mintTo(
    connection,
    minter,
    underlyingMint,
    underlyingAccount.publicKey,
    mintAuthority,
    underlyingAmount,
    []
  );

  if (quoteAmount > 0) {
    await mintTo(
      connection,
      minter,
      quoteMint,
      quoteAccount.publicKey,
      mintAuthority,
      quoteAmount,
      []
    );
  }
  return { optionAccount, quoteAccount, underlyingAccount, writerTokenAccount };
};

export const requestAndConfirmAirdrop = async (
  connection: anchor.web3.Connection,
  address: anchor.web3.PublicKey,
  amount = LAMPORTS_PER_SOL
) => {
  const fromAirdropSignature = await connection.requestAirdrop(address, amount);
  await connection.confirmTransaction(fromAirdropSignature);
};

export const createSplMint = async (
  connection: anchor.web3.Connection,
  mintAuthority: anchor.web3.PublicKey,
  numDecimals: number
) => {
  const payer = anchor.web3.Keypair.generate();
  await requestAndConfirmAirdrop(connection, payer.publicKey, LAMPORTS_PER_SOL);
  return await createMint(
    connection,
    payer,
    mintAuthority,
    mintAuthority,
    numDecimals
  );
};
