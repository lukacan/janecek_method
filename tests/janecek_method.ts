import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { JanecekMethod } from "../target/types/janecek_method";
import { PublicKey } from '@solana/web3.js'

describe("janecek_method", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JanecekMethod as Program<JanecekMethod>;

  it('can create party', async () => {
    // Before sending the transaction to the blockchain.
    const party = anchor.web3.Keypair.generate();
    const [userStatsPDA,number] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('name2'),
      ],
      program.programId
    )    
    console.log(userStatsPDA)
    await program.rpc.createParty('SMER', {
        accounts: {
            author: program.provider.wallet.publicKey,
            party: userStatsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
        }
    });

    //const partyAccount = await program.account.party.fetch(party.publicKey);
    //console.log(partyAccount);
  });
});
