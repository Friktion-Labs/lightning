import { ConnectedVoltSDK, FriktionSDK, MM_TOKEN_MINT_AUTHORITY, SERUM_PROGRAM_IDS, VoltSDK } from "@friktion-labs/friktion-sdk";

import { sendIns, sendInsList } from "@friktion-labs/friktion-utils";
import * as anchor from "@project-serum/anchor";
import {
  BN
} from "bn.js";
import {
  getAccount,
  mintTo,
  TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { TextEncoder } from "util";
import { createSplMint } from "./utils/helpers";
import { createTokenAccounts } from "./utils/tokenAccountUtils";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { getOrCreateAssociatedTokenAccounts } from './utils/associated_token';

export const FRIKTION_PROGRAM_ID = new PublicKey(
    "VoLT1mJz1sbnxwq5Fv2SXjdVDgPXrb9tJyC8WpMDkSp"
  );
  
export const DAO_EXAMPLES_PROGRAM_ID = new PublicKey(
  "DAo2pDtpiBFDu4TTiv2WggP6PfQ6FnKqwSRYxpMjyuV2"
);

const WRAPPED_SOL_ADDRESS = new PublicKey("So11111111111111111111111111111111111111112")

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.CpiExamples as anchor.Program;
const anchorProvider = program.provider as anchor.AnchorProvider;
const walletPayer = (anchorProvider.wallet as anchor.Wallet).payer;
const connection = anchorProvider.connection;

describe("cpi example tests", () => {

  let friktionSdk: FriktionSDK;

  let underlyingMint: PublicKey, quoteMint: PublicKey;
  let marketMakerAccessTokenMint: PublicKey;
  let vaultMint: PublicKey;
  let voltKey: PublicKey;

  const underlyingAmountToStart = new BN(1000000);

  const setupWallet = async (
    walletToSetup: PublicKey,
    unconnectedVoltSdk: VoltSDK,
    shouldAirdrop: boolean = true
  ): Promise<{
    vaultTokenAccount: PublicKey;
    underlyingTokenAccount: PublicKey;
    quoteTokenAccount: PublicKey;
  }> => {
    if (shouldAirdrop) {
      await anchorProvider.connection.confirmTransaction(
        await anchorProvider.connection.requestAirdrop(
          walletToSetup,
          10_000_000_000
        ),
        "confirmed"
      );
    }

    console.log("creating token accounts");

    const {
      vaultTokenAccount,
      underlyingTokenAccount,
      quoteTokenAccount,
    } = await createTokenAccounts(
      anchorProvider,
      unconnectedVoltSdk.voltVault.vaultMint,
      underlyingMint,
      quoteMint,
      walletToSetup,
      walletPayer
    );

    console.log("mint underlying");

    if (underlyingMint.toString() !== WRAPPED_SOL_ADDRESS.toString()) {
      await mintTo(
        connection,
        walletPayer,
        underlyingMint,
        underlyingTokenAccount,
        walletPayer,
        underlyingAmountToStart.toNumber() * 2
      );
    }

    console.log(
      "tokens in underlying acct: ",
      (await getAccount(connection, underlyingTokenAccount)).amount.toString()
    );

    console.log("derive pending deposit/withdrawal keys");

    return {
      vaultTokenAccount,
      underlyingTokenAccount,
      quoteTokenAccount,
    };
  };

  before("setup mints", async () => {
    underlyingMint = await createSplMint(
      anchorProvider.connection,
      walletPayer.publicKey,
      3
    );
    quoteMint = await createSplMint(
      anchorProvider.connection,
      walletPayer.publicKey,
      3
    );
    marketMakerAccessTokenMint = await createSplMint(
      program.provider.connection,
      MM_TOKEN_MINT_AUTHORITY,
      0
    );
    

    friktionSdk = new FriktionSDK({
      provider: anchorProvider,
      network: "mainnet-beta",
      testingOpts: {
        netOpts: {
          MM_TOKEN_MINT: marketMakerAccessTokenMint,
        },
      },
    });  

  })

  before("initialize vault", async () => {
    const { 
      voltKey: voltKeyLocal,
      instruction
     } =
      await VoltSDK.initializeVoltWithoutOptionMarketSeed(
        {
          sdk: friktionSdk,
          user: walletPayer.publicKey,
          permissionedMarketPremiumMint: quoteMint,
          serumProgramId: SERUM_PROGRAM_IDS.Mainnet,
          transferTimeWindow: new BN(1),
          upperBoundOtmStrikeFactor: new BN(1),

          underlyingAssetMint: underlyingMint, 
          quoteAssetMint: quoteMint, 
          underlyingAmountPerContract: new BN(100),
          expirationInterval: new BN(24 * 60 * 60),
          capacity: new BN(1000000),
          individualCapacity: new BN(1000000),
          permissionlessAuctions: false,
        }
      );

      voltKey = voltKeyLocal;

    await sendInsList(anchorProvider, [instruction], undefined, undefined, 400_000);

    const unconnectedVoltSdk = await friktionSdk.loadVoltByKey(voltKey)

    await setupWallet(
      anchorProvider.wallet.publicKey,
      unconnectedVoltSdk,
    );
    
    const cVoltSdk = new ConnectedVoltSDK(
      connection,
      anchorProvider.wallet.publicKey,
      unconnectedVoltSdk,
    );

    await sendInsList(anchorProvider, [await cVoltSdk.startRound()]);

    vaultMint = unconnectedVoltSdk.voltVault.vaultMint;
  });
  
  it("deposits", async () => {

    const unconnectedVoltSdk = await friktionSdk.loadVoltAndExtraDataByKey(voltKey)

    
    const textEncoder = new TextEncoder();
    const [daoProgramAuthorityKey] =
      await PublicKey.findProgramAddress(
        [textEncoder.encode("daoProgramAuthority")],
        DAO_EXAMPLES_PROGRAM_ID
      );

    // token accounts owned by daoProgramAuthorityKey. the given values are just a placeholder
    const [daoAuthorityUnderlyingTokenAccount, daoAuthorityVaultTokenAccount] = await getOrCreateAssociatedTokenAccounts(
      anchorProvider,
      {
          accountParams: [
              {   
            mint: underlyingMint,
            owner: daoProgramAuthorityKey,
            payer: anchorProvider.wallet.publicKey,
          }, {
            mint: vaultMint,
            owner: daoProgramAuthorityKey,
            payer: anchorProvider.wallet.publicKey,
          }
           ]
     }
    );

    await mintTo(
      connection,
      walletPayer,
      underlyingMint,
      daoAuthorityUnderlyingTokenAccount,
      walletPayer,
      underlyingAmountToStart.toNumber() * 2
    );

    const {
      extraVoltKey,
      roundInfoKey,
      roundUnderlyingTokensKey,
      roundVoltTokensKey,
      pendingDepositInfoKey,
      epochInfoKey
    } = await VoltSDK.findUsefulAddresses(
      unconnectedVoltSdk.voltKey,
      unconnectedVoltSdk.voltVault,
      daoProgramAuthorityKey,
      unconnectedVoltSdk.sdk.programs.Volt.programId
    );

    const depositAmount = new anchor.BN(1);
    const depositExampleAccountsStruct = {
      authority: anchorProvider.wallet.publicKey,

      voltVault: unconnectedVoltSdk.voltKey,
      extraVoltData: extraVoltKey,

      daoAuthority: daoProgramAuthorityKey,

      vaultMint: unconnectedVoltSdk.voltVault.vaultMint,
      vaultAuthority: unconnectedVoltSdk.voltVault.vaultAuthority,
      depositPool: unconnectedVoltSdk.voltVault.depositPool,
      writerTokenPool: unconnectedVoltSdk.voltVault.writerTokenPool,

      underlyingTokenSource: daoAuthorityUnderlyingTokenAccount,
      vaultTokenDestination: daoAuthorityVaultTokenAccount,

      roundInfo: roundInfoKey,
      roundVoltTokens: roundVoltTokensKey,
      roundUnderlyingTokens: roundUnderlyingTokensKey,

      
      pendingDepositInfo: pendingDepositInfoKey,

      epochInfo: epochInfoKey,
      whitelist: unconnectedVoltSdk.extraVoltData.whitelist,
      entropyProgram: unconnectedVoltSdk.extraVoltData.entropyProgramId,
      entropyGroup: unconnectedVoltSdk.extraVoltData.entropyGroup,
      entropyAccount: unconnectedVoltSdk.extraVoltData.entropyAccount,
      entropyCache: unconnectedVoltSdk.extraVoltData.entropyCache,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      voltProgramId: FRIKTION_PROGRAM_ID,
    };

    await sendInsList(anchorProvider, [program.instruction.depositExample(
      depositAmount,
      {
        accounts: depositExampleAccountsStruct,
      }
    )], undefined, undefined, 400_000);

  });
});