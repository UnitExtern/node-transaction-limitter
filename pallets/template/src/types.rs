use frame_support::{traits::Get, BoundedVec, CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;



/// Enumeration of Transaction Types.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
pub enum TransactionType {
	DoSomething,
    None
}