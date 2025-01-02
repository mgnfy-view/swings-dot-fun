import * as anchor from "@coral-xyz/anchor";
import { AnchorError } from "@coral-xyz/anchor";
import { assert } from "chai";

import { errors } from "./utils/constants";
import { pda, initializeProgram } from "./utils/utils";
import { setup } from "./utils/setup";

describe("Platform config", () => {
    const { owner, program } = setup();

    before(async () => {
        await initializeProgram();
    });

    it("Setting trading fee in bps fails if fee is greater than 10_000", async () => {
        const newTradingFeeInBps = 10_001;

        try {
            await program.methods
                .setTradingFeeInBps(newTradingFeeInBps)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.invalidFeeValueInBps);
        }
    });

    it("Setting trading fee in bps succeeds", async () => {
        const newTradingFeeInBps = 5_000;

        await program.methods
            .setTradingFeeInBps(newTradingFeeInBps)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.tradingFeeInBps, newTradingFeeInBps);
    });

    it("Setting token total supply fails if supply is 0", async () => {
        const newTokenTotalSupply = new anchor.BN(0);

        try {
            await program.methods
                .setTokenTotalSupply(newTokenTotalSupply)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Setting token total supply succeeds", async () => {
        const newTokenTotalSupply = new anchor.BN(200e9);

        await program.methods
            .setTokenTotalSupply(newTokenTotalSupply)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.tokenTotalSupply.toNumber(), newTokenTotalSupply.toNumber());
    });

    it("Setting virtual wsol amount fails if amount is 0", async () => {
        const newVirtualWsolAmount = new anchor.BN(0);

        try {
            await program.methods
                .setVirtualWsolAmount(newVirtualWsolAmount)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Setting virtual wsol amount fails if the difference between target wsol amount and virtual wsol amount is not greater than migration fee", async () => {
        const newVirtualWsolAmount = new anchor.BN(149e9);

        try {
            await program.methods
                .setVirtualWsolAmount(newVirtualWsolAmount)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.invalidPlatformConfigParams);
        }
    });

    it("Setting virtual wsol amount succeeds", async () => {
        const newVirtualWsolAmount = new anchor.BN(110e9);

        await program.methods
            .setVirtualWsolAmount(newVirtualWsolAmount)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.virtualWsolAmount.toNumber(), newVirtualWsolAmount.toNumber());
    });

    it("Setting target wsol amount fails if amount is 0", async () => {
        const newTargetWsolAmount = new anchor.BN(0);

        try {
            await program.methods
                .setTargetWsolAmount(newTargetWsolAmount)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Setting target wsol amount fails if the difference between target wsol amount and virtual wsol amount is not greater than migration fee", async () => {
        const newTargetWsolAmount = new anchor.BN(111e9);

        try {
            await program.methods
                .setTargetWsolAmount(newTargetWsolAmount)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.invalidPlatformConfigParams);
        }
    });

    it("Setting target wsol amount succeeds", async () => {
        const newTargetWsolAmount = new anchor.BN(200e9);

        await program.methods
            .setTargetWsolAmount(newTargetWsolAmount)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.targetWsolAmount.toNumber(), newTargetWsolAmount.toNumber());
    });

    it("Setting migration fee fails if amount is 0", async () => {
        const newMigrationFee = new anchor.BN(0);

        try {
            await program.methods
                .setMigrationFee(newMigrationFee)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.valueZero);
        }
    });

    it("Setting migration fee fails if the difference between target wsol amount and virtual wsol amount is not greater than migration fee", async () => {
        const newMigrationFee = new anchor.BN(100e9);

        try {
            await program.methods
                .setMigrationFee(newMigrationFee)
                .accounts({
                    owner: owner.publicKey,
                    platformConfig: pda.getPlatformConfig(program.programId),
                })
                .rpc();
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.invalidPlatformConfigParams);
        }
    });

    it("Setting migration fee succeeds", async () => {
        const newMigrationFee = new anchor.BN(5e8);

        await program.methods
            .setMigrationFee(newMigrationFee)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.migrationFee.toNumber(), newMigrationFee.toNumber());
    });

    it("Setting owner succeeds", async () => {
        const newOwner = anchor.web3.Keypair.generate();

        await program.methods
            .setOwner(newOwner.publicKey)
            .accounts({
                owner: owner.publicKey,
                platformConfig: pda.getPlatformConfig(program.programId),
            })
            .rpc();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(platformConfig.owner.toString(), newOwner.publicKey.toString());
    });
});
