import type { Provider } from "@project-serum/anchor";
import {
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import type { PublicKey, Signer } from "@solana/web3.js";
import { Transaction } from "@solana/web3.js";

interface AccountParams {
  mint: PublicKey;
  owner: PublicKey;
  payer?: PublicKey;
}

export const getOrCreateAssociatedTokenAccounts = async (
  provider: Provider,
  {
    accountParams,
    signers,
  }: {
    accountParams: AccountParams[];
    signers?: Signer[];
  }
): Promise<PublicKey[]> => {
  // - make the full list of addresses,
  // - make CreateTokenAccount instructions for the ones that don't exist yet,
  // - create whatever's necessary in one shot,
  // - return the list of addresses
  const addresses: PublicKey[] = [];
  const tx = new Transaction();

  for (const { mint, owner, payer } of accountParams) {
    const address = await getAssociatedTokenAddress(mint, owner, true);
    addresses.push(address);
    try {
      await getAccount(provider.connection, address);
    } catch (e) {
      if (e instanceof Error) {
        tx.add(
          createAssociatedTokenAccountInstruction(
            payer || owner,
            address,
            owner,
            mint
          )
        );
      } else {
        throw e;
      }
    }
  }
  if (tx.instructions.length) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    await provider.sendAndConfirm!(tx, signers);
  }

  return addresses;
};
