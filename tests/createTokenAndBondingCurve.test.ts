import * as spl from "@solana/spl-token";
import { assert } from "chai";

import { decimals, tokenMetadataProgram } from "./utils/constants";
import { pda, initializeProgram } from "./utils/utils";
import { setup } from "./utils/setup";

describe("swings-dot-fun", () => {
    const { provider, owner, program } = setup();

    before(async () => {
        await initializeProgram();

        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );
    });

    it("Should create token and bonding curve", async () => {
        const createTokenAndBondingCurveParams = {
            name: "Portal",
            symbol: "PORTAL",
            uri: "https://github.com/mgnfy-view/portal.git",
        };
        const mint = pda.getMint(createTokenAndBondingCurveParams.name, program.programId);
        const bondingCurvePublicKey = pda.getBondingCurve(mint, program.programId);
        const bondingCurveMintTokenAccountPublicKey = pda.getBondingCurveMintTokenAccount(
            mint,
            program.programId
        );

        await program.methods
            .createTokenAndBondingCurve(createTokenAndBondingCurveParams)
            .accounts({
                platformConfig: pda.getPlatformConfig(program.programId),
                creator: owner.publicKey,
                mint,
                metadata: pda.getMetadata(mint),
                bondingCurve: bondingCurvePublicKey,
                bondingCurveMintTokenAccount: bondingCurveMintTokenAccountPublicKey,
                tokenMetadataProgram,
            })
            .rpc();

        const bondingCurve = await program.account.bondingCurve.fetch(bondingCurvePublicKey);
        const platformConfig = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(bondingCurve.creator.toString(), owner.publicKey.toString());
        assert.equal(bondingCurve.token.toString(), mint.toString());
        assert.equal(
            bondingCurve.tokenTotalSupply.toNumber(),
            platformConfig.tokenTotalSupply.toNumber()
        );
        assert.equal(
            bondingCurve.virtualWsolAmount.toNumber(),
            platformConfig.virtualWsolAmount.toNumber()
        );
        assert.equal(
            bondingCurve.targetWsolAmount.toNumber(),
            platformConfig.targetWsolAmount.toNumber()
        );
        assert.equal(
            bondingCurve.currentTokenReserve.toNumber(),
            platformConfig.tokenTotalSupply.toNumber()
        );
        assert.equal(
            bondingCurve.currentWsolReserve.toNumber(),
            platformConfig.virtualWsolAmount.toNumber()
        );
        assert.isFalse(bondingCurve.launched);
        assert(bondingCurve.bump >= 0 && bondingCurve.bump <= 255);
        assert(bondingCurve.mintBump >= 0 && bondingCurve.mintBump <= 255);
        assert(
            bondingCurve.bondingCurveMintTokenAccountBump >= 0 &&
                bondingCurve.bondingCurveMintTokenAccountBump <= 255
        );

        const mintAccount = await spl.getMint(provider.connection, mint);

        assert.isTrue(mintAccount.isInitialized);
        assert.equal(mintAccount.decimals, decimals);
        assert.equal(mintAccount.mintAuthority.toString(), bondingCurvePublicKey.toString());

        const bondingCurveMintTokenAccount = await spl.getAccount(
            provider.connection,
            bondingCurveMintTokenAccountPublicKey
        );

        assert.isTrue(bondingCurveMintTokenAccount.isInitialized);
        assert.equal(
            Number(
                (await spl.getAccount(provider.connection, bondingCurveMintTokenAccountPublicKey))
                    .amount
            ),
            platformConfig.tokenTotalSupply.toNumber()
        );
        assert.equal(bondingCurveMintTokenAccount.mint.toString(), mint.toString());
    });
});
