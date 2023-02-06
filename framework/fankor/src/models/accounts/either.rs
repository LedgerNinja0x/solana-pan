use crate::errors::{FankorErrorCode, FankorResult};
use crate::models::{Account, DefaultAccount, FankorContext, UninitializedAccount, ZcAccount};
use crate::prelude::PdaChecker;
use crate::traits::{AccountInfoVerification, CpiInstruction, Instruction};
use solana_program::account_info::AccountInfo;
use solana_program::instruction::AccountMeta;
use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::Write;

/// Alias for the common case of having either an actual account or its uninitialized counterpart.
pub type MaybeUninitializedAccount<'info, T> =
    Either<Account<'info, T>, UninitializedAccount<'info>>;

/// Alias for the common case of having either a zero-copy account or its uninitialized counterpart.
pub type MaybeUninitializedZcAccount<'info, T> =
    Either<ZcAccount<'info, T>, UninitializedAccount<'info>>;

/// Alias for the common case of having either an actual account or the default account.
pub type MaybeDefaultAccount<'info, T> = Either<T, DefaultAccount<'info>>;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Tries to deserialize `L` first and then `R` if `L` fails.
///
/// This is useful to have a fallback for some type, for example, it can be used for maybe
/// uninitialized accounts: `Either<Account<'info, T>, UninitializedAccount<'info, T>>`.
/// For this case you can use the `MaybeUninitializedAccount` type alias.
///
/// Note that `L` and `R` must be disjoint types, otherwise the deserialization will
/// always return `L`.
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<'info, L: Instruction<'info>, R: Instruction<'info>> Either<L, R> {
    // GETTERS -----------------------------------------------------------------

    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }

    pub fn left(&self) -> Option<&L> {
        match self {
            Either::Left(v) => Some(v),
            Either::Right(_) => None,
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut L> {
        match self {
            Either::Left(v) => Some(v),
            Either::Right(_) => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        match self {
            Either::Left(_) => None,
            Either::Right(v) => Some(v),
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut R> {
        match self {
            Either::Left(_) => None,
            Either::Right(v) => Some(v),
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn unwrap_left(self) -> Option<L> {
        match self {
            Either::Left(v) => Some(v),
            Either::Right(_) => None,
        }
    }

    pub fn unwrap_right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(v) => Some(v),
        }
    }
}

impl<'info, L: Instruction<'info>, R: Instruction<'info>> Instruction<'info> for Either<L, R> {
    type CPI = CpiEither<L::CPI, R::CPI>;
    type LPI = LpiEither<L::LPI, R::LPI>;

    fn verify_account_infos<'a>(
        &self,
        config: &mut AccountInfoVerification<'a, 'info>,
    ) -> FankorResult<()> {
        match self {
            Either::Left(v) => v.verify_account_infos(config),
            Either::Right(v) => v.verify_account_infos(config),
        }
    }

    #[inline(never)]
    fn try_from(
        context: &'info FankorContext<'info>,
        buf: &mut &[u8],
        accounts: &mut &'info [AccountInfo<'info>],
    ) -> FankorResult<Self> {
        if buf.is_empty() {
            return Err(FankorErrorCode::NotEnoughDataToDeserializeInstruction.into());
        }

        let result = match buf[0] {
            0 => {
                let mut new_buf = &buf[1..];
                let mut new_accounts = *accounts;
                let result = Either::Left(L::try_from(context, &mut new_buf, &mut new_accounts)?);

                *accounts = new_accounts;
                *buf = new_buf;

                result
            }
            1 => {
                let mut new_buf = &buf[1..];
                let mut new_accounts = *accounts;
                let result = Either::Right(R::try_from(context, &mut new_buf, &mut new_accounts)?);

                *accounts = new_accounts;
                *buf = new_buf;

                result
            }
            _ => {
                return Err(FankorErrorCode::InstructionDidNotDeserialize {
                    account: type_name::<Self>().to_string(),
                }
                .into())
            }
        };

        Ok(result)
    }
}

impl<'info, L: PdaChecker<'info>, R: PdaChecker<'info>> PdaChecker<'info> for Either<L, R> {
    fn pda_info(&self) -> Option<&'info AccountInfo<'info>> {
        match self {
            Either::Left(v) => v.pda_info(),
            Either::Right(v) => v.pda_info(),
        }
    }
}

impl<'info, L: Debug + Instruction<'info>, R: Debug + Instruction<'info>> Debug for Either<L, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Either::Left(v) => f
                .debug_struct("Either")
                .field("Left", &v)
                .field("Right", &Option::<R>::None)
                .finish(),
            Either::Right(v) => f
                .debug_struct("Either")
                .field("Left", &Option::<L>::None)
                .field("Right", &v)
                .finish(),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub enum CpiEither<L, R> {
    Left(L),
    Right(R),
}

impl<'info, L: CpiInstruction<'info>, R: CpiInstruction<'info>> CpiInstruction<'info>
    for CpiEither<L, R>
{
    fn serialize_into_instruction_parts<W: Write>(
        &self,
        writer: &mut W,
        metas: &mut Vec<AccountMeta>,
        infos: &mut Vec<AccountInfo<'info>>,
    ) -> FankorResult<()> {
        match self {
            CpiEither::Left(v) => v.serialize_into_instruction_parts(writer, metas, infos),
            CpiEither::Right(v) => v.serialize_into_instruction_parts(writer, metas, infos),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub enum LpiEither<L, R> {
    Left(L),
    Right(R),
}

impl<L: crate::traits::LpiInstruction, R: crate::traits::LpiInstruction>
    crate::traits::LpiInstruction for LpiEither<L, R>
{
    fn serialize_into_instruction_parts<W: Write>(
        &self,
        writer: &mut W,
        metas: &mut Vec<AccountMeta>,
    ) -> FankorResult<()> {
        match self {
            LpiEither::Left(v) => v.serialize_into_instruction_parts(writer, metas),
            LpiEither::Right(v) => v.serialize_into_instruction_parts(writer, metas),
        }
    }
}
