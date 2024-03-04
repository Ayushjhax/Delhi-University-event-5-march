// Use statements for external crates required by the program
use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
};

// Define an enum `MyInstruction` to represent the different instructions 
// this program can understand. Each variant can hold its own data.
#[derive(BorshSerialize, BorshDeserialize)]
pub enum MyInstruction {
    // Variant for "Say Hello" instruction with data for the message
    SayHelloInstruction(SayHelloArgs),
    // Variant for "Say Goodbye" instruction with data for two messages
    SayGoodbyeInstruction(SayGoodbyeArgs),
}

// Define a struct `SayHelloArgs` to hold data for the "Say Hello" instruction
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SayHelloArgs {
    // String to hold the hello message
    hello_message: String,
}

// Define a struct `SayGoodbyeArgs` to hold data for the "Say Goodbye" instruction
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SayGoodbyeArgs {
    // String to hold the first goodbye message
    goodbye_message: String,
    // String to hold the second goodbye message
    second_goodbye_message: String,
}

// Helper functions to process instructions
fn say_hello(args: SayHelloArgs) {
    // Log the hello message to the Solana runtime
    msg!("{}", args.hello_message);
}

fn say_goodbye(args: SayGoodbyeArgs) {
    // Log both goodbye messages to the Solana runtime
    msg!("{}", args.goodbye_message);
    msg!("{}", args.second_goodbye_message);
}

// Define the program's entrypoint using the `entrypoint!` macro
entrypoint!(hello_world);

fn hello_world(
    // Program ID of this program on the Solana blockchain
    program_id: &Pubkey,
    // Accounts provided to the program during invocation
    accounts: &[AccountInfo],
    // Raw instruction data containing the chosen instruction
    instruction_data: &[u8],
) -> ProgramResult {

    // Deserialize the instruction data into a MyInstruction enum variant
    let instruction = MyInstruction::try_from_slice(&instruction_data)?;
    
    // Match on the deserialized instruction to call the appropriate function
    match instruction {
        MyInstruction::SayHelloInstruction(args) => say_hello(args),
        MyInstruction::SayGoodbyeInstruction(args) => say_goodbye(args),
    };

    // Log the program's own Pubkey (unique program ID)
    msg!("Our program's Program ID: {}", &program_id);

    // Get an iterator over the provided accounts
    let accounts_iter = &mut accounts.iter();
    
    // Extract the first account as the payer account using next_account_info
    let payer = next_account_info(accounts_iter)?;

    // Log the address (Pubkey) of the payer account
    msg!("Payer Address: {}", payer.key);

    Ok(())
}