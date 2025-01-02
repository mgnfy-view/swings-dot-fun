import * as spl from "@solana/spl-token";
import { assert } from "chai";

import { decimals, tokenMetadataProgram } from "./utils/constants";
import { pda, initializeProgram } from "./utils/utils";
import { setup } from "./utils/setup";

describe("swings-dot-fun", () => {
    const { provider, owner, program } = setup();

    before(async () => {
        await initializeProgram();
    });

    it("Should create token and bonding curve", async () => {
        const createTokenAndBondingCurveParams = {
            name: "Portal",
            symbol: "PORTAL",
            uri: "https://github.com/mgnfy-view/portal.git",
        };

        const platformConfig = pda.getPlatformConfig(program.programId);
        const mint = pda.getMint(createTokenAndBondingCurveParams.name, program.programId);
        const metadata = pda.getMetadata(mint);
        const bondingCurve = pda.getBondingCurve(mint, program.programId);
        const bondingCurveMintTokenAccount = pda.getBondingCurveMintTokenAccount(
            mint,
            program.programId
        );

        await program.methods
            .createTokenAndBondingCurve(createTokenAndBondingCurveParams)
            .accounts({
                platformConfig,
                creator: owner.publicKey,
                mint,
                metadata,
                bondingCurve,
                bondingCurveMintTokenAccount,
                tokenMetadataProgram,
            })
            .rpc();

        const bondingCurveAccount = await program.account.bondingCurve.fetch(bondingCurve);
        const platformConfigAccount = await program.account.platformConfig.fetch(
            pda.getPlatformConfig(program.programId)
        );

        assert.equal(bondingCurveAccount.creator.toString(), owner.publicKey.toString());
        assert.equal(bondingCurveAccount.token.toString(), mint.toString());
        assert.equal(
            bondingCurveAccount.tokenTotalSupply.toNumber(),
            platformConfigAccount.tokenTotalSupply.toNumber()
        );
        assert.equal(
            bondingCurveAccount.virtualWsolAmount.toNumber(),
            platformConfigAccount.virtualWsolAmount.toNumber()
        );
        assert.equal(
            bondingCurveAccount.targetWsolAmount.toNumber(),
            platformConfigAccount.targetWsolAmount.toNumber()
        );
        assert.equal(
            bondingCurveAccount.currentTokenReserve.toNumber(),
            platformConfigAccount.tokenTotalSupply.toNumber()
        );
        assert.equal(
            bondingCurveAccount.currentWsolReserve.toNumber(),
            platformConfigAccount.virtualWsolAmount.toNumber()
        );
        assert.isFalse(bondingCurveAccount.launched);
        assert(bondingCurveAccount.bump >= 0 && bondingCurveAccount.bump <= 255);
        assert(bondingCurveAccount.mintBump >= 0 && bondingCurveAccount.mintBump <= 255);
        assert(
            bondingCurveAccount.bondingCurveMintTokenAccountBump >= 0 &&
                bondingCurveAccount.bondingCurveMintTokenAccountBump <= 255
        );

        const mintAccount = await spl.getMint(provider.connection, mint);

        assert.isTrue(mintAccount.isInitialized);
        assert.equal(mintAccount.decimals, decimals);
        assert.equal(mintAccount.mintAuthority.toString(), bondingCurve.toString());
        assert.equal(Number(mintAccount.supply), platformConfigAccount.tokenTotalSupply.toNumber());

        const bondingCurveMintTokenAccountData = await spl.getAccount(
            provider.connection,
            bondingCurveMintTokenAccount
        );

        assert.isTrue(bondingCurveMintTokenAccountData.isInitialized);
        assert.equal(bondingCurveMintTokenAccountData.mint.toString(), mint.toString());
        assert.equal(
            Number(
                (await spl.getAccount(provider.connection, bondingCurveMintTokenAccount)).amount
            ),
            platformConfigAccount.tokenTotalSupply.toNumber()
        );
    });
});
