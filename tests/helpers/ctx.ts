import {
    PublicKey,
    Connection,
    Signer,

} from '@solana/web3.js';
import {
    createMint,
    getOrCreateAssociatedTokenAccount,
    Account as TokenAccount,
    mintTo,
    getAssociatedTokenAddress,
} from '@solana/spl-token';
import { Program } from "@coral-xyz/anchor";
import { AnchorLaunchpad } from "../../target/types/anchor_launchpad";
import { createUserWithLamports } from './helper';


export interface Ctx {
    connection: Connection,
    program: Program<AnchorLaunchpad>,
    authority: Signer,
    dever: Signer,
    contributor: Signer,
    addFeeUnConPubkey: PublicKey,
    configAccount: PublicKey,
    launchpadMint: PublicKey,
    launchpadAccount: PublicKey
    whitelistAccount: PublicKey,
    mintConfigAccount: PublicKey,
    userConfigAccount: PublicKey,
    vaultLaunchpad: PublicKey,
    deverTokenAccount: TokenAccount
}

export async function createCtx(connection: Connection, program: Program<AnchorLaunchpad>): Promise<Ctx> {

    const authority = await createUserWithLamports(connection, 1);
    const dever = await createUserWithLamports(connection, 1);
    const contributor = await createUserWithLamports(connection, 1);

    const addFeeUnCon = await createUserWithLamports(connection, 1);
    const addFeeUnConPubkey = addFeeUnCon.publicKey;
    const launchpadMint = await createMint(
        connection,
        dever, // payer
        dever.publicKey, // mintAuthority
        dever.publicKey, // freezeAuthority
        0 // decimals
    );

    const configAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("config"), authority.publicKey.toBuffer()],
        program.programId
    )[0];

    const launchpadAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("launchpad"), launchpadMint.toBuffer(), dever.publicKey.toBuffer()],
        program.programId
    )[0];

    const mintConfigAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("mint_config"), launchpadMint.toBuffer(), dever.publicKey.toBuffer()],
        program.programId
    )[0];

    const whitelistAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("whitelist"), launchpadAccount.toBuffer(), dever.publicKey.toBuffer()],
        program.programId
    )[0];

    const userConfigAccount = PublicKey.findProgramAddressSync(
        [Buffer.from("user_config"), contributor.publicKey.toBuffer(), launchpadAccount.toBuffer()],
        program.programId
    )[0];

    const vaultLaunchpad = await getAssociatedTokenAddress(launchpadMint, launchpadAccount, true);

    const deverTokenAccount = await getOrCreateAssociatedTokenAccount(connection, dever, launchpadMint, dever.publicKey);

    //mint for dever 3_000_000 tokens
    await mintTo(
        connection,
        dever,
        launchpadMint,
        deverTokenAccount.address,
        dever,
        3000000,
    );

    return {
        connection,
        program,
        authority,
        dever,
        contributor,
        addFeeUnConPubkey,
        configAccount,
        launchpadMint,
        launchpadAccount,
        userConfigAccount,
        whitelistAccount,
        mintConfigAccount,
        vaultLaunchpad,
        deverTokenAccount
    }
}


