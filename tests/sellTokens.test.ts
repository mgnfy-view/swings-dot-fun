import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";

import { errors, wsolMint } from "./utils/constants";
import { pda, initializeProgram, createTokenAndBondingCurve, buyTokens } from "./utils/utils";
import { setup } from "./utils/setup";

describe("swings-dot-fun", () => {
    const { provider, owner, program } = setup();

    const createTokenAndBondingCurveParams = {
        name: "Portal",
        symbol: "PORTAL",
        uri: "https://github.com/mgnfy-view/portal.git",
    };

    before(async () => {
        await initializeProgram();
        await createTokenAndBondingCurve(
            createTokenAndBondingCurveParams.name,
            createTokenAndBondingCurveParams.symbol,
            createTokenAndBondingCurveParams.uri
        );
        const wsolAmount = new anchor.BN(10e9);
        await buyTokens(createTokenAndBondingCurveParams.name, wsolAmount);
    });

    it("Selling tokens fails if token amount is 0", async () => {
        const tokenAmount = new anchor.BN(0);

        const platformConfig = pda.getPlatformConfig(program.programId);
        const platformWsolTokenAccount = pda.getPlatformWsolTokenAccount(program.programId);
        const mint = pda.getMint(createTokenAndBondingCurveParams.name, program.programId);
        const bondingCurve = pda.getBondingCurve(mint, program.programId);
        const bondingCurveMintTokenAccount = pda.getBondingCurveMintTokenAccount(
            mint,
            program.programId
        );
        const sellerMintTokenAccount = await spl.getAssociatedTokenAddress(mint, owner.publicKey);
        const sellerWsolTokenAccount = await spl.getAssociatedTokenAddress(
            wsolMint,
            owner.publicKey
        );

        try {
            await program.methods
                .sellTokens(createTokenAndBondingCurveParams.name, tokenAmount)
                .accounts({
                    seller: owner.publicKey,
                    platformConfig,
                    wsolMint,
                    platformWsolTokenAccount,
                    mint,
                    bondingCurve,
                    bondingCurveMintTokenAccount,
                    sellerMintTokenAccount,
                    sellerWsolTokenAccount,
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as anchor.AnchorError).error.errorMessage;

            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Selling tokens succeeds", async () => {
        const tokenAmount = new anchor.BN(1e9);

        const platformConfig = pda.getPlatformConfig(program.programId);
        const platformWsolTokenAccount = pda.getPlatformWsolTokenAccount(program.programId);
        const mint = pda.getMint(createTokenAndBondingCurveParams.name, program.programId);
        const bondingCurve = pda.getBondingCurve(mint, program.programId);
        const bondingCurveMintTokenAccount = pda.getBondingCurveMintTokenAccount(
            mint,
            program.programId
        );
        const sellerMintTokenAccount = await spl.getAssociatedTokenAddress(mint, owner.publicKey);
        const sellerWsolTokenAccount = await spl.getAssociatedTokenAddress(
            wsolMint,
            owner.publicKey
        );

        await program.methods
            .sellTokens(createTokenAndBondingCurveParams.name, tokenAmount)
            .accounts({
                seller: owner.publicKey,
                platformConfig,
                wsolMint,
                platformWsolTokenAccount,
                mint,
                bondingCurve,
                bondingCurveMintTokenAccount,
                sellerMintTokenAccount,
                sellerWsolTokenAccount,
            })
            .rpc();

        // const platformConfigAccount = await program.account.platformConfig.fetch(platformConfig);
        // const platformWsolTokenAccountBalance = Number(
        //     (await spl.getAccount(provider.connection, platformWsolTokenAccount)).amount
        // );

        // assert.equal(
        //     platformConfigAccount.accumulatedWsolFees.toNumber(),
        //     expectedAccumulatedWsolFeeAmount
        // );
        // assert.equal(platformWsolTokenAccountBalance, wsolAmount.toNumber() + ex);

        // const bondingCurveAccount = await program.account.bondingCurve.fetch(bondingCurve);

        // assert.equal(bondingCurveAccount.currentTokenReserve.toNumber(), tokenAmountOut);
        // assert.equal(
        //     bondingCurveAccount.currentWsolReserve.toNumber(),
        //     bondingCurveAccount.virtualWsolAmount.toNumber() + wsolAmount.toNumber()
        // );
        // assert.isTrue(bondingCurveAccount.launched);

        // const buyerMintTokenAccountBalance = Number(
        //     (await spl.getAccount(provider.connection, bondingCurveMintTokenAccount)).amount
        // );

        // assert.equal(buyerMintTokenAccountBalance, tokenAmountOut);
    });
});
