import * as anchor from "@project-serum/anchor";
import { JanecekMethod } from "../target/types/janecek_method";

describe("janecek_method", () => {

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JanecekMethod as anchor.Program<JanecekMethod>;
  const new_voter = anchor.web3.Keypair.generate();

});
