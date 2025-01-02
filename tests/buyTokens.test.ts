import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { assert } from "chai";

import { errors, wsolMint } from "./utils/constants";
import { pda, initializeProgram, createTokenAndBondingCurve } from "./utils/utils";
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
    });

    it("Buying tokens fails if wsol amount is 0", async () => {
        const wsolAmount = new anchor.BN(0);

        const platformConfig = pda.getPlatformConfig(program.programId);
        const platformWsolTokenAccount = pda.getPlatformWsolTokenAccount(program.programId);
        const mint = pda.getMint(createTokenAndBondingCurveParams.name, program.programId);
        const bondingCurve = pda.getBondingCurve(mint, program.programId);
        const bondingCurveMintTokenAccount = pda.getBondingCurveMintTokenAccount(
            mint,
            program.programId
        );
        const buyerMintTokenAccount = await spl.getAssociatedTokenAddress(mint, owner.publicKey);

        try {
            await program.methods
                .buyTokens(createTokenAndBondingCurveParams.name, wsolAmount)
                .accounts({
                    buyer: owner.publicKey,
                    platformConfig,
                    wsolMint,
                    platformWsolTokenAccount,
                    mint,
                    bondingCurve,
                    bondingCurveMintTokenAccount,
                    buyerMintTokenAccount,
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as anchor.AnchorError).error.errorMessage;

            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Buying tokens succeeds", async () => {
        const wsolAmount = new anchor.BN(50e9);
        const feeAmount = 50e7;
        const tokenAmountOut = 66666666667;

        const platformConfig = pda.getPlatformConfig(program.programId);
        const platformWsolTokenAccount = pda.getPlatformWsolTokenAccount(program.programId);
        const mint = pda.getMint(createTokenAndBondingCurveParams.name, program.programId);
        const bondingCurve = pda.getBondingCurve(mint, program.programId);
        const bondingCurveMintTokenAccount = pda.getBondingCurveMintTokenAccount(
            mint,
            program.programId
        );
        const buyerMintTokenAccount = await spl.getAssociatedTokenAddress(mint, owner.publicKey);

        await program.methods
            .buyTokens(createTokenAndBondingCurveParams.name, wsolAmount)
            .accounts({
                buyer: owner.publicKey,
                platformConfig,
                wsolMint,
                platformWsolTokenAccount,
                mint,
                bondingCurve,
                bondingCurveMintTokenAccount,
                buyerMintTokenAccount,
            })
            .rpc();

        const platformConfigAccount = await program.account.platformConfig.fetch(platformConfig);
        const platformWsolTokenAccountBalance = Number(
            (await spl.getAccount(provider.connection, platformWsolTokenAccount)).amount
        );

        assert.equal(platformConfigAccount.accumulatedWsolFees.toNumber(), feeAmount);
        assert.equal(platformWsolTokenAccountBalance, wsolAmount.toNumber() + feeAmount);

        const bondingCurveAccount = await program.account.bondingCurve.fetch(bondingCurve);

        assert.equal(bondingCurveAccount.currentTokenReserve.toNumber(), tokenAmountOut);
        assert.equal(
            bondingCurveAccount.currentWsolReserve.toNumber(),
            bondingCurveAccount.virtualWsolAmount.toNumber() + wsolAmount.toNumber()
        );

        const buyerMintTokenAccountBalance = Number(
            (await spl.getAccount(provider.connection, bondingCurveMintTokenAccount)).amount
        );

        assert.equal(buyerMintTokenAccountBalance, tokenAmountOut);
    });
});
