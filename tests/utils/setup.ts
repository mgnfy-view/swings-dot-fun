import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SwingsDotFun } from "../../target/types/swings_dot_fun";

export function setup() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const owner = (provider.wallet as anchor.Wallet).payer;

    const program = anchor.workspace.SwingsDotFun as Program<SwingsDotFun>;

    return {
        provider,
        owner,
        program,
    };
}
