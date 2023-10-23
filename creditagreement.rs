use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::declare_id!("YourProgramIDHere");

// Kredi anlaşması veri yapısı
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CreditContract {
    pub borrower: Pubkey,
    pub amount: u64,
    pub interest_rate: u8, // Değişken faiz oranı
}

solana_program::entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let lender_account = next_account_info(accounts_iter)?;
    let borrower_account = next_account_info(accounts_iter)?;

    // Kredi sözleşmesi hesabı
    let credit_contract_account = next_account_info(accounts_iter)?;

    if credit_contract_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // İşlem verisi: [0]: Kredi talep tutarı, [1]: Faiz oranı
    if instruction_data.len() != 2 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Kredi talep tutarı ve faiz oranını al
    let credit_amount = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
    let interest_rate = instruction_data[8];

    // Kredi sözleşmesini güncelle
    let mut credit_contract = CreditContract::load(credit_contract_account)?;

    // Kredi talep tutarını ve faiz oranını güncelle
    credit_contract.amount = credit_amount;
    credit_contract.interest_rate = interest_rate;

    // Kredi sözleşmesini kaydet
    credit_contract.save(credit_contract_account)?;

    // Kredi verme işlemi burada yapılabilir

    Ok(())
}

impl CreditContract {
    pub fn load(account: &AccountInfo) -> Result<CreditContract, ProgramError> {
        // Veriyi oku ve ayrıştır
        let data = account.try_borrow_data()?;
        let (borrower, amount, interest_rate) =
            array_refs![data, 32, 8, 1];

        Ok(CreditContract {
            borrower: Pubkey::new_from_array(*borrower),
            amount: u64::from_le_bytes(*amount),
            interest_rate: interest_rate[0],
        })
    }

    pub fn save(&self, account: &AccountInfo) -> Result<(), ProgramError> {
        // Veriyi yaz
        let data = account.try_borrow_mut_data()?;
        let data_len = data.len();

        let (borrower_dst, amount_dst, interest_rate_dst) =
            mut_array_refs![data, 32, 8, 1];

        borrower_dst.copy_from_slice(self.borrower.to_bytes().as_slice());
        amount_dst.copy_from_slice(&self.amount.to_le_bytes());
        interest_rate_dst[0] = self.interest_rate;

        // Bellek kontrolü
        if data_len > 41 {
            for byte in &mut data[41..] {
                *byte = 0;
            }
        }

        Ok(())
    }
}
