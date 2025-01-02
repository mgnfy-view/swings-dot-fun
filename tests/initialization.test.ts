import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { AnchorError } from "@coral-xyz/anchor";
import { assert } from "chai";

import { wsolMint, errors } from "./utils/constants";
import { pda, getPlatformConfigInitParams } from "./utils/utils";
import { setup } from "./utils/setup";

describe("swings-dot-fun", () => {
    const { provider, owner, program } = setup();

    it("Initialization fails if token total supply is 0", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);
        platformConfigInitParams.tokenTotalSupply = new anchor.BN(0);

        try {
            await program.methods
                .initializePlatformConfig(platformConfigInitParams)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                    wsolMint,
                    platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Initialization fails if virtual wsol amount is 0", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);
        platformConfigInitParams.virtualWsolAmount = new anchor.BN(0);

        try {
            await program.methods
                .initializePlatformConfig(platformConfigInitParams)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                    wsolMint,
                    platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Initialization fails if target wsol amount is 0", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);
        platformConfigInitParams.targetWsolAmount = new anchor.BN(0);

        try {
            await program.methods
                .initializePlatformConfig(platformConfigInitParams)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                    wsolMint,
                    platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Initialization fails if trading fee in bps is greater than 10_000", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);
        platformConfigInitParams.tradingFeeInBps = 10_001;

        try {
            await program.methods
                .initializePlatformConfig(platformConfigInitParams)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                    wsolMint,
                    platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.invalidFeeValueInBps);
        }
    });

    it("Initialization fails if migration fee is greater than the difference between target wsol amount and virtual wsol amount", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);
        platformConfigInitParams.migrationFee = new anchor.BN(100e9);

        try {
            await program.methods
                .initializePlatformConfig(platformConfigInitParams)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                    wsolMint,
                    platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.invalidPlatformConfigParams);
        }
    });

    it("Initialization succeeds", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);

        await program.methods
            .initializePlatformConfig(platformConfigInitParams)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
                wsolMint,
                platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.owner.toString(), owner.publicKey.toString());
        assert.equal(platformConfig.tradingFeeInBps, platformConfigInitParams.tradingFeeInBps);
        assert.equal(platformConfig.accumulatedWsolFees.toNumber(), 0);
        assert.equal(
            platformConfig.tokenTotalSupply.toNumber(),
            platformConfigInitParams.tokenTotalSupply.toNumber()
        );
        assert.equal(
            platformConfig.virtualWsolAmount.toNumber(),
            platformConfigInitParams.virtualWsolAmount.toNumber()
        );
        assert.equal(
            platformConfig.targetWsolAmount.toNumber(),
            platformConfigInitParams.targetWsolAmount.toNumber()
        );
        assert.equal(
            platformConfig.migrationFee.toNumber(),
            platformConfigInitParams.migrationFee.toNumber()
        );
        assert(platformConfig.bump >= 0 && platformConfig.bump <= 255);
        assert(
            platformConfig.platformWsolTokenAccountBump >= 0 &&
                platformConfig.platformWsolTokenAccountBump <= 255
        );

        const platformWsolTokenAccount = await spl.getAccount(
            provider.connection,
            pda.getPlatformWsolTokenAccount(program.programId)
        );
        assert.isTrue(platformWsolTokenAccount.isInitialized);
    });

    it("Cannot be initialized again", async () => {
        let platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);

        try {
            await program.methods
                .initializePlatformConfig(platformConfigInitParams)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                    wsolMint,
                    platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
                })
                .rpc();
        } catch {}
    });
});
