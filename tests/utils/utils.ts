import * as anchor from "@coral-xyz/anchor";
import { setup } from "./setup";

import { wsolMint, tokenMetadataProgram, seeds } from "./constants";

const pda = {
    getPlatformConfig(programId: anchor.web3.PublicKey) {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(seeds.platformConfig)],
            programId
        )[0];
    },
    getPlatformWsolTokenAccount(programId: anchor.web3.PublicKey) {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(seeds.platformWsolTokenAccount)],
            programId
        )[0];
    },
    getMint(name: String, programId: anchor.web3.PublicKey) {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(seeds.mint), Buffer.from(name)],
            programId
        )[0];
    },
    getMetadata(mint: anchor.web3.PublicKey) {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(seeds.metadata), tokenMetadataProgram.toBuffer(), mint.toBuffer()],
            tokenMetadataProgram
        )[0];
    },
    getBondingCurve(mint: anchor.web3.PublicKey, programId: anchor.web3.PublicKey) {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(seeds.bondingCurve), mint.toBuffer()],
            programId
        )[0];
    },
    getBondingCurveMintTokenAccount(mint: anchor.web3.PublicKey, programId: anchor.web3.PublicKey) {
        return anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(seeds.bondingCurveMintTokenAccount), Buffer.from(mint.toBuffer())],
            programId
        )[0];
    },
};

const getPlatformConfigInitParams = (owner: anchor.web3.PublicKey) => {
    return {
        owner,
        tradingFeeInBps: 100,
        tokenTotalSupply: new anchor.BN(100e9),
        virtualWsolAmount: new anchor.BN(100e9),
        targetWsolAmount: new anchor.BN(150e9),
        migrationFee: new anchor.BN(2e9),
    };
};

const initializeProgram = async () => {
    const { owner, program } = setup();
    const platformConfigInitParams = getPlatformConfigInitParams(owner.publicKey);

    await program.methods
        .initializePlatformConfig(platformConfigInitParams)
        .accounts({
            owner: owner.publicKey,
            platformConfig: pda.getPlatformConfig(program.programId),
            wsolMint,
            platformWsolTokenAccount: pda.getPlatformWsolTokenAccount(program.programId),
        })
        .rpc();
};

const createTokenAndBondingCurve = async (name: string, symbol: string, uri: string) => {
    const { owner, program } = setup();

    const mint = pda.getMint(name, program.programId);
    const bondingCurvePublicKey = pda.getBondingCurve(mint, program.programId);
    const bondingCurveMintTokenAccountPublicKey = pda.getBondingCurveMintTokenAccount(
        mint,
        program.programId
    );

    await program.methods
        .createTokenAndBondingCurve({ name, symbol, uri })
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
};

export { pda, getPlatformConfigInitParams, initializeProgram, createTokenAndBondingCurve };
