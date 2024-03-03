import {
    PublicKey, SystemProgram,LAMPORTS_PER_SOL
} from "@solana/web3.js";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { Ctx } from "./ctx";

export namespace RPC {
    export async function initialize(ctx: Ctx) {
        await ctx.program.methods.initialize(
            ctx.addFeeUnConPubkey
        ).accounts({
            authority: ctx.authority.publicKey,
            configAccount: ctx.configAccount,
            systemProgram: SystemProgram.programId,
        }).signers([ctx.authority]).rpc();
    }
    export async function changeCreateFee(ctx: Ctx, addfeeUnCon: PublicKey, newFee: number,
        creationFreeOptionSol: number) {
        await ctx.program.methods.changeCreateFee(
            addfeeUnCon,
            new anchor.BN(newFee),
            creationFreeOptionSol,
        ).accounts({
            authority: ctx.authority.publicKey,
            configAccount: ctx.configAccount,
            systemProgram: SystemProgram.programId,
        }).signers([ctx.authority]).rpc();
    }
    //--------------------------------------------------------------------DEVER-------------------------------------------------------------------------------//
    export async function createLaunchpad(ctx: Ctx) {
        const launchpadParamsStep1 = {
            listingOption: 1,
            affiliate: 0,
            totalTokenTo: new anchor.BN(300000),
            decimals: 0,
            contractToken: ctx.launchpadMint
          }    
          const launchpadParamsStep2 = {
            preRate: new anchor.BN(1),
            whitelist: new anchor.BN(0),
            liquidityLockDay: new anchor.BN(0),
            softCap: new anchor.BN(10 * LAMPORTS_PER_SOL),
            hardCap: new anchor.BN(100 * LAMPORTS_PER_SOL),//100 SOL
            minBuy: new anchor.BN(0.1 * LAMPORTS_PER_SOL),//0.1 SOL
            maxBuy: new anchor.BN(1 * LAMPORTS_PER_SOL), //1 SOL
            typeRefund: new anchor.BN(0),
            liquidityRate: new anchor.BN(15),
            listingRate: new anchor.BN(10),
            startTime: new anchor.BN(12321321),
            endTime: new anchor.BN(1709475327000),
          }
          const launchpadParamsStep3 = {
            logoUrl: "https://www.pinksale.finance/",
            website: "https://www.pinksale.finance/",
            facebook: "https://www.pinksale.finance/",
            twitter: "https://www.pinksale.finance/",
            github: "https://www.pinksale.finance/",
            telegram: "https://www.pinksale.finance/",
            instagram: "https://www.pinksale.finance/",
            reddit: "https://www.pinksale.finance/",
            discord: "https://www.pinksale.finance/",
            youtube: "https://www.pinksale.finance/",
            description: "https://www.pinksale.finance/",
          };

          
        await ctx.program.methods.createLaunchpad(
            launchpadParamsStep1,  launchpadParamsStep2, launchpadParamsStep3
        ).accounts({
            dever: ctx.dever.publicKey,
            launchpadMint: ctx.launchpadMint,
            launchpadAccount: ctx.launchpadAccount,
            configAccount: ctx.configAccount,
            mintConfigAccount: ctx.mintConfigAccount,
            systemProgram: SystemProgram.programId,
            vaultLaunchpad: ctx.vaultLaunchpad,
            deverTokenAccount: ctx.deverTokenAccount.address,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID
        }).signers([ctx.dever]).rpc();
    }

    export async function addWhitelist(ctx: Ctx) {
        const whitelist = [ctx.authority.publicKey, ctx.contributor.publicKey, ctx.dever.publicKey]
        await ctx.program.methods.addToWhitelist(
            whitelist
        ).accounts({
            dever: ctx.dever.publicKey,
            launchpadAccount: ctx.launchpadAccount,
            whitelistAccount: ctx.whitelistAccount,
            systemProgram: SystemProgram.programId,
        }).signers([ctx.dever]).rpc();
    }

    //--------------------------------------------------------------------USER-------------------------------------------------------------------------------------//
    export async function contribute(ctx: Ctx, amount: number) {
        await ctx.program.methods.contribute(
           new anchor.BN(amount)
        ).accounts({
            contributor: ctx.contributor.publicKey,
            launchpadAccount: ctx.launchpadAccount,
            mintConfigAccount: ctx.mintConfigAccount,
            userConfigAccount: ctx.userConfigAccount,
            whitelistAccount: ctx.whitelistAccount,
            systemProgram: SystemProgram.programId,
        }).signers([ctx.contributor]).rpc();
    }

    export async function unContribute(ctx: Ctx) {
        await ctx.program.methods.unContribute(
        ).accounts({
            contributor: ctx.contributor.publicKey,
            launchpadAccount: ctx.launchpadAccount,
            userConfigAccount: ctx.userConfigAccount,
            addFeeUnCon: ctx.addFeeUnConPubkey,
            dever: ctx.dever.publicKey,
            mintConfigAccount: ctx.mintConfigAccount,
            systemProgram: SystemProgram.programId,
        }).signers([ctx.contributor]).rpc({
            skipPreflight:true
          });
    }
}
