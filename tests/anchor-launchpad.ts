import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorLaunchpad } from "../target/types/anchor_launchpad";
import { Connection, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Ctx, createCtx } from "./helpers/ctx";
import { RPC } from "./helpers/rpc";
import { CheckCtx } from "./helpers/check";

describe("anchor-launchpad", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorLaunchpad as Program<AnchorLaunchpad>;
  const connection = new Connection("http://localhost:8899", 'recent');

  let ctx: Ctx;

  //-----------------------------------------------------------------------ADMIN-----------------------------------------------------------------------------------
  it("Is initialized!", async () => {
    // Add your test here.
    ctx = await createCtx(connection, program);
    await RPC.initialize(ctx);
    await CheckCtx.checkConfigAccountInfo(ctx, ctx.addFeeUnConPubkey, new anchor.BN(0.2 * LAMPORTS_PER_SOL), 99);
  });

  it("Change create fee", async () => {
    // Add your test here.
    const newAddFeeUnConPubkey = ctx.addFeeUnConPubkey;
    const newFee = 0.3 * LAMPORTS_PER_SOL;
    const newCreationFreeOptionSol = 90;
    await RPC.changeCreateFee(ctx,  newAddFeeUnConPubkey, newFee, newCreationFreeOptionSol);
    await CheckCtx.checkConfigAccountInfo(ctx, newAddFeeUnConPubkey, new anchor.BN(newFee), newCreationFreeOptionSol);

  });
  //----------------------------------------------------------------------DEVER-------------------------------------------------------------------------------------

  it("Create Launchpad!", async () => {
    // Add your test here.
    await RPC.createLaunchpad(ctx);

  });

  it("Add to whitelist", async () => {
    // Add your test here.
    await RPC.addWhitelist(ctx);

  });
  //----------------------------------------------------------------------USER-------------------------------------------------------------------------------------

  it("Contribute!", async () => {
    // Add your test here.
    let amount = 5000;
    await RPC.contribute(ctx, amount);
  });

  it("Un Contribute!", async () => {
    // Add your test here.

    let mintConfigAccount  = await ctx.program.account.mintConfigAccount.fetch(ctx.mintConfigAccount);
    console.log(mintConfigAccount.mint.toString());
    console.log(ctx.launchpadMint.toString())
    await RPC.unContribute(ctx);
  });

});
