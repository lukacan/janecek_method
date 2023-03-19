import * as anchor from "@project-serum/anchor";
import { JanecekMethod } from "../target/types/janecek_method";
import { PublicKey, Keypair } from '@solana/web3.js'
import * as assert from "assert";

describe("janecek_method", () => {
  let admin_secretKey = Uint8Array.from([206,146,51,36,247,155,237,189,223,
    244,222,196,51,107,85,193,76,22,132,33,97,251,158,85,93,55,197,87,
    122,101,145,176,224,68,42,180,144,146,5,12,50,29,187,121,160,8,189,
    245,198,249,155,243,106,173,77,196,113,211,235,27,128,134,150,195]);

  let admin_keypair = Keypair.fromSecretKey(admin_secretKey);
  


  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JanecekMethod as anchor.Program<JanecekMethod>;
  const new_voter = anchor.web3.Keypair.generate();


  it("Admin add funds",async () =>{
    const signature = await program.provider.connection.requestAirdrop(admin_keypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);
  })


  it('Create Party', async () => {
    
    // create random user and airdrop
    const ferko_mrkvicka = anchor.web3.Keypair.generate();
    const signature = await program.provider.connection.requestAirdrop(ferko_mrkvicka.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);
    
    
    // derive pda from party name
    const [pda1,bump1] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('political party1'),
      ],
      program.programId
    )


    const [pda2,bump2] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('political party2'),
      ],
      program.programId
    )

    // check if party already exists, if dont, create
    const info1 = await program.provider.connection.getAccountInfo(pda1);
    if(info1 != null)
    {
      console.log("Already exists")
    }else
    {
      try {
        await program.rpc.createParty('political party1', {
          accounts: {
              author: ferko_mrkvicka.publicKey,
              party: pda1,
              systemProgram: anchor.web3.SystemProgram.programId,
          },signers:[
            ferko_mrkvicka
          ]
      }); 
      } catch (error) {
      }
    }

    const info2 = await program.provider.connection.getAccountInfo(pda2);
    if(info2 != null)
    {
      console.log("Already exists")
    }else
    {
      try {
        await program.rpc.createParty('political party2', {
          accounts: {
              author: ferko_mrkvicka.publicKey,
              party: pda2,
              systemProgram: anchor.web3.SystemProgram.programId,
          },signers:[
            ferko_mrkvicka
          ]
      }); 
      } catch (error) {
      }
    }

  });
  it("Add Voter", async () =>{

    const info = await program.provider.connection.getAccountInfo(new_voter.publicKey);
    if(info != null)
    {
      console.log("PublicKey already registered as user")
    }else
    {
      await program.rpc.createVoter({
        accounts: {
            admin: admin_keypair.publicKey,
            voter: new_voter.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },signers:[
          admin_keypair,new_voter
        ]
      });
    }

  });

  it("Add Voter with no admin rights", async () =>{
    const new_voter_ = anchor.web3.Keypair.generate();
    const new_admin = anchor.web3.Keypair.generate();

    const signature = await program.provider.connection.requestAirdrop(new_admin.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);


    const info = await program.provider.connection.getAccountInfo(new_voter_.publicKey);
    if(info != null)
    {
      console.log("PublicKey already registered as voter")
    }else
    {
      try {
        await program.rpc.createVoter({
          accounts: {
              admin: new_admin.publicKey,
              voter: new_voter_.publicKey,
              systemProgram: anchor.web3.SystemProgram.programId,
          },signers:[
            new_admin,new_voter_
          ]
        });
      } catch (error) {
        assert.ok(error instanceof anchor.AnchorError);
        assert.equal((error as anchor.AnchorError)
        .error.errorCode.code, 'PermissionDenied');
      }

    }

  });


  it("Vote positive", async () =>{
    //const new_voter = anchor.web3.Keypair.generate();

    const info = await program.provider.connection.getAccountInfo(new_voter.publicKey);
    if(info != null)
    {
      console.log("PublicKey already registered as user")
    }else
    {
      await program.rpc.createVoter({
        accounts: {
            admin: admin_keypair.publicKey,
            voter: new_voter.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },signers:[
          admin_keypair,new_voter
        ]
      });
    }
    const [pda,bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('political party1'),
      ],
      program.programId
    )


    await program.rpc.votePositive({
      accounts:{
        voter: new_voter.publicKey,
        party: pda,
        systemProgram:anchor.web3.SystemProgram.programId,
      },signers:[
        new_voter
      ]
    });

  });




  it("Spend all votes", async () =>{
    const new_voter_ = anchor.web3.Keypair.generate();

    
    const info = await program.provider.connection.getAccountInfo(new_voter_.publicKey);
    if(info != null)
    {
      console.log("PublicKey already registered as user")
    }else
    {
      await program.rpc.createVoter({
        accounts: {
            admin: admin_keypair.publicKey,
            voter: new_voter_.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },signers:[
          admin_keypair,new_voter_
        ]
      });
    }


    const [pda1,bump1] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('political party1'),
      ],
      program.programId
    )

    const [pda2,bump2] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('political party2'),
      ],
      program.programId
    )


    await program.rpc.votePositive({
      accounts:{
        voter: new_voter_.publicKey,
        party: pda1,
        systemProgram:anchor.web3.SystemProgram.programId,
      },signers:[
        new_voter_
      ]
    });


    await program.rpc.votePositive({
      accounts:{
        voter: new_voter_.publicKey,
        party: pda2,
        systemProgram:anchor.web3.SystemProgram.programId,
      },signers:[
        new_voter_
      ]
    });


    await program.rpc.voteNegative({
      accounts:{
        voter: new_voter_.publicKey,
        party: pda2,
        systemProgram:anchor.web3.SystemProgram.programId,
      },signers:[
        new_voter_
      ]
    });


    try {
      await program.rpc.voteNegative({
        accounts:{
          voter: new_voter_.publicKey,
          party: pda2,
          systemProgram:anchor.web3.SystemProgram.programId,
        },signers:[
          new_voter_
        ]
      });
    } catch (error) {
      assert.ok(error instanceof anchor.AnchorError);
      assert.equal((error as anchor.AnchorError)
      .error.errorCode.code, 'NotAllowedOperation');
    }

  });


});
