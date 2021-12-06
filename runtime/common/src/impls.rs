// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Auxillary struct/enums for polkadot runtime.

use sp_runtime::traits::{Convert, Saturating};
use sp_runtime::{FixedPointNumber, FixedI128, Perquintill};
use frame_support::traits::{OnUnbalanced, Imbalance, Currency, Get};
use crate::{MaximumBlockWeight, NegativeImbalance};

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
where
	R: balances::Trait + authorship::Trait,
	<R as system::Trait>::AccountId: From<primitives::AccountId>,
	<R as system::Trait>::AccountId: Into<primitives::AccountId>,
	<R as system::Trait>::Event: From<balances::RawEvent<
		<R as system::Trait>::AccountId,
		<R as balances::Trait>::Balance,
		balances::DefaultInstance>
	>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		let numeric_amount = amount.peek();
		let author = <authorship::Module<R>>::author();
		<balances::Module<R>>::resolve_creating(&<authorship::Module<R>>::author(), amount);
		<system::Module<R>>::deposit_event(balances::RawEvent::Deposit(author, numeric_amount));
	}
}

/// Converter for currencies to votes.
pub struct CurrencyToVoteHandler<R>(sp_std::marker::PhantomData<R>);

impl<R> CurrencyToVoteHandler<R>
where
	R: balances::Trait,
	R::Balance: Into<u128>,
{
	fn factor() -> u128 {
		let issuance: u128 = <balances::Module<R>>::total_issuance().into();
		(issuance / u64::max_value() as u128).max(1)
	}
}

impl<R> Convert<u128, u64> for CurrencyToVoteHandler<R>
where
	R: balances::Trait,
	R::Balance: Into<u128>,
{
	fn convert(x: u128) -> u64 { (x / Self::factor()) as u64 }
}

impl<R> Convert<u128, u128> for CurrencyToVoteHandler<R>
where
	R: balances::Trait,
	R::Balance: Into<u128>,
{
	fn convert(x: u128) -> u128 { x * Self::factor() }
}

/// Update the given multiplier based on the following formula
///
///   diff = (previous_block_weight - target_weight)/max_weight
///   v = 0.00004
///   next_weight = weight * (1 + (v * diff) + (v * diff)^2 / 2)
///
/// Where `target_weight` must be given as the `Get` implementation of the `T` generic type.
/// https://research.web3.foundation/en/latest/polkadot/Token%20Economics/#relay-chain-transaction-fees
pub struct TargetedFeeAdjustment<T, R>(sp_std::marker::PhantomData<(T, R)>);

impl<T: Get<Perquintill>, R: system::Trait> Convert<FixedI128, FixedI128> for TargetedFeeAdjustment<T, R> {
	fn convert(multiplier: FixedI128) -> FixedI128 {
		let max_weight = MaximumBlockWeight::get();
		let block_weight = <system::Module<R>>::block_weight().total().min(max_weight);
		let target_weight = (T::get() * max_weight) as u128;
		let block_weight = block_weight as u128;

		// determines if the first_term is positive
		let positive = block_weight >= target_weight;
		let diff_abs = block_weight.max(target_weight) - block_weight.min(target_weight);
		// safe, diff_abs cannot exceed u64 and it can always be computed safely even with the lossy
		// `FixedI128::saturating_from_rational`.
		let diff = FixedI128::saturating_from_rational(diff_abs, max_weight.max(1));
		let diff_squared = diff.saturating_mul(diff);

		// 0.00004 = 4/100_000 = 40_000/10^9
		let v = FixedI128::saturating_from_rational(4, 100_000);
		// 0.00004^2 = 16/10^10 Taking the future /2 into account... 8/10^10
		let v_squared_2 = FixedI128::saturating_from_rational(8, 10_000_000_000u64);

		let first_term = v.saturating_mul(diff);
		let second_term = v_squared_2.saturating_mul(diff_squared);

		if positive {
			// Note: this is merely bounded by how big the multiplier and the inner value can go,
			// not by any economical reasoning.
			let excess = first_term.saturating_add(second_term);
			multiplier.saturating_add(excess)
		} else {
			// Defensive-only: first_term > second_term. Safe subtraction.
			let negative = first_term.saturating_sub(second_term);
			multiplier.saturating_sub(negative)
				// despite the fact that apply_to saturates weight (final fee cannot go below 0)
				// it is crucially important to stop here and don't further reduce the weight fee
				// multiplier. While at -1, it means that the network is so un-congested that all
				// transactions have no weight fee. We stop here and only increase if the network
				// became more busy.
				.max(FixedI128::saturating_from_integer(-1))
		}
	}
}
