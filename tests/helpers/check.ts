import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import {
    getAccount as getTokenAccount,
    Account as TokenAccount,
} from '@solana/spl-token';
import { Ctx } from "./ctx";

type Balance = number | anchor.BN | bigint;
export namespace CheckCtx {

    export async function lamportsBalance(ctx: Ctx, key: PublicKey, balanceBefore: Balance, addedBalance: Balance) {
        await Check.lamportsBalance(ctx.connection, key, Number(balanceBefore) + Number(addedBalance));
    }

    export async function checkConfigAccountInfo(ctx: Ctx, expectedAddFeeUnCon: PublicKey, expectedCreateFee: anchor.BN , expectedCreationFreeOptionSol: number) {
        const config = await ctx.program.account.configAccount.fetch(ctx.configAccount);
        expect(config.addFeeUnCon.toString()).to.equal(expectedAddFeeUnCon.toString());
        expect(config.creationFee.toString()).to.equal(expectedCreateFee.toString());
        expect(config.creationFeeOptionSol.toString()).to.equal(expectedCreationFreeOptionSol.toString());
    }
  
}
export namespace Check {
    export async function lamportsBalance(connection: Connection, account: PublicKey, expectedBalance: number, msg?: String) {
        let info = await connection.getAccountInfo(account);
        let message = "Lamports balance";
        if (msg) { message += ` of ${msg}` }
        expect(`${info.lamports}`, message).to.be.eq(`${expectedBalance}`);
    }

    export async function tokenBalance(connection: Connection, key: PublicKey, expectedBalance: number, msg?: string) {
        let acc: TokenAccount | null = await getTokenAccount(connection, key).catch(() => null);

        if (acc) {
            expect(`${acc.amount}`, `Token balance ${msg}`).to.be.eq(`${expectedBalance}`);
        } else {
            // Account does not exist, so it has zero tokens ;)
            expect(`${0}`, `Token balance (acc not found) ${msg}`).to.be.eq(`${expectedBalance}`);
        }
    }
}
