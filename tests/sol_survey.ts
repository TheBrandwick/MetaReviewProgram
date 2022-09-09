import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { SolSurvey } from "../target/types/sol_survey";


describe("sol_survey", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolSurvey as Program<SolSurvey>;

  const provider = anchor.AnchorProvider.env();
  const platform_owner = provider.wallet.publicKey;
  const system = anchor.web3.SystemProgram;

  const tony_wallet = anchor.web3.Keypair.generate();
  const alice_wallet = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    let [program_state_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('state')],
      program.programId
    )
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      programState: program_state_pda,
      owner: platform_owner,
      systemProgram: system.programId
    }).rpc();
    console.log("Your transaction signature (initialized)", tx);
  });

  it("Tony Signs Up!", async () => {

    await fundAccount(provider, tony_wallet);

    let [program_state_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('state')],
      program.programId
    )
    let program_state_data = await program.account.programState.fetch(program_state_pda);
    // console.log({program_state_data})
    let user_count = program_state_data.userCount;
    let [user_account_pda, user_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('user'), user_count.toBuffer('be', 8)],
      program.programId
    )
    let [fund_locker_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('locker'), user_count.toBuffer('be', 8)],
      program.programId
    )
    let payload = {
      bump: user_bump,
      first_name: 'Tony',
      last_name: 'Stark',
      email: 'tony@starkindustries.com',
      profile_pic: "https://www.tonystark.com"
    }

    console.log("Tony Account Addres", user_account_pda.toString())
    // Add your test here.
    const tx = await program.methods.signUpUser(
      payload.bump, payload.first_name, payload.last_name, payload.email, payload.profile_pic
    )
      .accounts({
        programState: program_state_pda,
        userAccount: user_account_pda,
        fundLocker: fund_locker_pda,
        user: tony_wallet.publicKey,
        systemProgram: system.programId
      })
      .signers([tony_wallet])
      .rpc();
    // console.log("Your transaction signature (Signup)", tx);
  });

  it("Tony Creates Survey!", async () => {

    await fundAccount(provider, tony_wallet);

    let [program_state_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('state')],
      program.programId
    )

    let program_state_data = await program.account.programState.fetch(program_state_pda);
    // console.log({program_state_data})
    let survey_count = program_state_data.surveyCount;
    let [survey_account_pda, survey_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('survey'), survey_count.toBuffer('be', 8)],
      program.programId
    )

    let payload = {
      bump: survey_bump,
      maxParticipantsCount: new anchor.BN(10),
      rewardPerParticipant: 0.01,
      validUntil: new anchor.BN(1661684233861),
      isDraft: false,
      isActive: true,
      formUri: "https://www.aliceland.com"
    }

    // Add your test here.
    const tx = await program.methods.createSurvey(
      payload.bump,
      payload.maxParticipantsCount,
      payload.rewardPerParticipant,
      payload.validUntil,
      payload.isDraft,
      payload.isActive,
      payload.formUri
    )
      .accounts({
        programState: program_state_pda,
        survey: survey_account_pda,
        creator: tony_wallet.publicKey,
        systemProgram: system.programId
      })
      .signers([tony_wallet])
      .rpc();
    // let survey_data = await program.account.survey.fetch(survey_account_pda);
    // console.log("Create survey => ", survey_data)
    console.log("Your transaction signature (Create Survey)", tx);
  });
  it("Tony Edites Survey!", async () => {

    await fundAccount(provider, tony_wallet);

    let survey_id = new anchor.BN(0);
    let [survey_account_pda, survey_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('survey'), survey_id.toBuffer('be', 8)],
      program.programId
    )
    let survey_data = await program.account.survey.fetch(survey_account_pda);
    console.log("Edit survey => ", survey_data)
    let payload = {
      isDraft: false,
      isActive: true,
      formUri: "https://www.alicland.com"
    }

    // Add your test here.
    const tx = await program.methods.editSurvey(
      payload.isDraft,
      payload.isActive,
      payload.formUri
    )
      .accounts({
        survey: survey_account_pda,
        user: tony_wallet.publicKey,
      })
      .signers([tony_wallet])
      .rpc();
    console.log("Your transaction signature (Edit Survey)", tx);
  });

  it("Tony Enters Survey!", async () => {

    let userId = new anchor.BN(0)
    let survey_id = new anchor.BN(0)

    let [survey_account_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('survey'), survey_id.toBuffer('be', 8)],
      program.programId
    )

    let [participation_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('participation'), userId.toBuffer('be', 8), survey_id.toBuffer('be', 8)],
      program.programId
    )


    let payload = {
      userId: userId
    }

    // Add your test here.
    const tx = await program.methods.participateSurvey(
      payload.userId
    )
      .accounts({
        participation: participation_pda,
        survey: survey_account_pda,
        user: tony_wallet.publicKey,
        systemProgram: system.programId
      })
      .signers([tony_wallet])
      .rpc();
    console.log("Your transaction signature (Enter Survey)", tx);
  });
  it("Tony Submits Survey!", async () => {

    let userId = new anchor.BN(0)
    let survey_id = new anchor.BN(0)

    let [survey_account_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('survey'), survey_id.toBuffer('be', 8)],
      program.programId
    )

    let [participation_pda] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('participation'), userId.toBuffer('be', 8), survey_id.toBuffer('be', 8)],
      program.programId
    )


    // Add your test here.
    const tx = await program.methods.submitSurveyAsParticipant(userId)
      .accounts({
        participation: participation_pda,
        survey: survey_account_pda,
        user: tony_wallet.publicKey,
      })
      .signers([tony_wallet])
      .rpc();
    console.log("Your transaction signature (Submit Survey)", tx);
  });
  it("Alice Signs Up!", async () => {

    await fundAccount(provider, alice_wallet);

    let [program_state_pda_alice] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('state')],
      program.programId
    )

    let program_state_data = await program.account.programState.fetch(program_state_pda_alice);
    // console.log({program_state_data})
    let user_count = program_state_data.userCount;
    let [user_account_pda_alice, user_bump_alice] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('user'), user_count.toBuffer('be', 8)],
      program.programId
    )
    let [fund_locker_pda_alice] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode('locker'), user_count.toBuffer('be', 8)],
      program.programId
    )
    let payload = {
      bump: user_bump_alice,
      first_name: 'Alice',
      last_name: 'Land',
      email: 'alice@wonderland.com',
      profile_pic: "https://www.aliceland.com"
    }

    // Add your test here.
    const tx = await program.methods.signUpUser(
      payload.bump, payload.first_name, payload.last_name, payload.email, payload.profile_pic
    )
      .accounts({
        programState: program_state_pda_alice,
        userAccount: user_account_pda_alice,
        fundLocker: fund_locker_pda_alice,
        user: alice_wallet.publicKey,
        systemProgram: system.programId
      })
      .signers([alice_wallet])
      .rpc();
    console.log("Your transaction signature (Signup)", tx);
    // console.log("Alice Account Addres", user_account_pda.toString())
  });
});


const fundAccount = async (provider, userKey) => {
  const connection = provider.connection;
  // let b_balance = await connection.getBalance(userKey.publicKey);
  // console.log("Before Balance =", b_balance)

  let signature = await connection.requestAirdrop(userKey.publicKey, 1000953520)
  await connection.confirmTransaction(signature);

  // let balance = await connection.getBalance(userKey.publicKey);
  // console.log("After Balance =", balance)
}