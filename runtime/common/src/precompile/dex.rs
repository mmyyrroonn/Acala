// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::input::{Input, InputT, Output};
use frame_support::log;
use module_evm::{
	precompiles::Precompile,
	runner::state::{PrecompileFailure, PrecompileOutput, PrecompileResult},
	Context, ExitError, ExitSucceed,
};
use module_support::{DEXManager, SwapLimit};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use primitives::{Balance, CurrencyId};
use sp_runtime::RuntimeDebug;
use sp_std::{borrow::Cow, marker::PhantomData, prelude::*};

/// The `DEX` impl precompile.
///
///
/// `input` data starts with `action`.
///
/// Actions:
/// - Get liquidity. Rest `input` bytes: `currency_id_a`, `currency_id_b`.
/// - Swap with exact supply. Rest `input` bytes: `who`, `currency_id_a`, `currency_id_b`,
///   `supply_amount`, `min_target_amount`.
pub struct DexPrecompile<R>(PhantomData<R>);

#[module_evm_utiltity_macro::generate_function_selector]
#[derive(RuntimeDebug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Action {
	GetLiquidityPool = "getLiquidityPool(address,address)",
	GetLiquidityTokenAddress = "getLiquidityTokenAddress(address,address)",
	GetSwapTargetAmount = "getSwapTargetAmount(address[],uint256)",
	GetSwapSupplyAmount = "getSwapSupplyAmount(address[],uint256)",
	SwapWithExactSupply = "swapWithExactSupply(address,address[],uint256,uint256)",
	SwapWithExactTarget = "swapWithExactTarget(address,address[],uint256,uint256)",
	AddLiquidity = "addLiquidity(address,address,address,uint256,uint256,uint256)",
	RemoveLiquidity = "removeLiquidity(address,address,address,uint256,uint256,uint256)",
}

impl<Runtime> Precompile for DexPrecompile<Runtime>
where
	Runtime: module_evm::Config + module_prices::Config,
	module_dex::Pallet<Runtime>: DEXManager<Runtime::AccountId, CurrencyId, Balance>,
{
	fn execute(input: &[u8], _target_gas: Option<u64>, _context: &Context, _is_static: bool) -> PrecompileResult {
		let input = Input::<Action, Runtime::AccountId, Runtime::AddressMapping, Runtime::Erc20InfoMapping>::new(input);

		let action = input.action()?;

		match action {
			Action::GetLiquidityPool => {
				let currency_id_a = input.currency_id_at(1)?;
				let currency_id_b = input.currency_id_at(2)?;
				log::debug!(
					target: "evm",
					"dex: get_liquidity_pool currency_id_a: {:?}, currency_id_b: {:?}",
					currency_id_a, currency_id_b
				);

				let (balance_a, balance_b) = <module_dex::Pallet<Runtime> as DEXManager<
					Runtime::AccountId,
					CurrencyId,
					Balance,
				>>::get_liquidity_pool(currency_id_a, currency_id_b);

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: Output::default().encode_u128_tuple(balance_a, balance_b),
					logs: Default::default(),
				})
			}
			Action::GetLiquidityTokenAddress => {
				let currency_id_a = input.currency_id_at(1)?;
				let currency_id_b = input.currency_id_at(2)?;
				log::debug!(
					target: "evm",
					"dex: get_liquidity_token address currency_id_a: {:?}, currency_id_b: {:?}",
					currency_id_a, currency_id_b
				);

				let value = <module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::get_liquidity_token_address(currency_id_a, currency_id_b)
					.ok_or_else(|| PrecompileFailure::Error { exit_status: ExitError::Other("Dex get_liquidity_token_address failed".into())})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: Output::default().encode_address(&value),
					logs: Default::default(),
				})
			}
			Action::GetSwapTargetAmount => {
				// solidity abi enocde array will add an offset at input[1]
				let supply_amount = input.balance_at(2)?;
				let path_len = input.u32_at(3)?;
				let mut path = vec![];
				for i in 0..path_len {
					path.push(input.currency_id_at((4 + i) as usize)?);
				}
				log::debug!(
					target: "evm",
					"dex: get_swap_target_amount path: {:?}, supply_amount: {:?}",
					path, supply_amount
				);

				let value = <module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::get_swap_amount(&path, SwapLimit::ExactSupply(supply_amount, Balance::MIN))
					.map(|(_, target)| target)
					.ok_or_else(|| PrecompileFailure::Error { exit_status: ExitError::Other("Dex get_swap_target_amount failed".into())})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: Output::default().encode_u128(value),
					logs: Default::default(),
				})
			}
			Action::GetSwapSupplyAmount => {
				// solidity abi enocde array will add an offset at input[1]
				let target_amount = input.balance_at(2)?;
				let path_len = input.u32_at(3)?;
				let mut path = vec![];
				for i in 0..path_len {
					path.push(input.currency_id_at((4 + i) as usize)?);
				}
				log::debug!(
					target: "evm",
					"dex: get_swap_supply_amount path: {:?}, target_amount: {:?}",
					path, target_amount
				);

				let value = <module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::get_swap_amount(&path, SwapLimit::ExactTarget(Balance::MAX, target_amount))
					.map(|(supply, _)| supply)
					.ok_or_else(|| PrecompileFailure::Error { exit_status: ExitError::Other("Dex get_swap_supply_amount failed".into())})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: Output::default().encode_u128(value),
					logs: Default::default(),
				})
			}
			Action::SwapWithExactSupply => {
				let who = input.account_id_at(1)?;
				// solidity abi enocde array will add an offset at input[2]
				let supply_amount = input.balance_at(3)?;
				let min_target_amount = input.balance_at(4)?;
				let path_len = input.u32_at(5)?;
				let mut path = vec![];
				for i in 0..path_len {
					path.push(input.currency_id_at((6 + i) as usize)?);
				}
				log::debug!(
					target: "evm",
					"dex: swap_with_exact_supply who: {:?}, path: {:?}, supply_amount: {:?}, min_target_amount: {:?}",
					who, path, supply_amount, min_target_amount
				);

				let (_, value) =
					<module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::swap_with_specific_path(&who, &path, SwapLimit::ExactSupply(supply_amount, min_target_amount))
					.map_err(|e| PrecompileFailure::Error { exit_status:ExitError::Other(Cow::Borrowed(e.into()))})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: Output::default().encode_u128(value),
					logs: Default::default(),
				})
			}
			Action::SwapWithExactTarget => {
				let who = input.account_id_at(1)?;
				// solidity abi enocde array will add an offset at input[2]
				let target_amount = input.balance_at(3)?;
				let max_supply_amount = input.balance_at(4)?;
				let path_len = input.u32_at(5)?;
				let mut path = vec![];
				for i in 0..path_len {
					path.push(input.currency_id_at((6 + i) as usize)?);
				}
				log::debug!(
					target: "evm",
					"dex: swap_with_exact_target who: {:?}, path: {:?}, target_amount: {:?}, max_supply_amount: {:?}",
					who, path, target_amount, max_supply_amount
				);

				let (value, _) =
					<module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::swap_with_specific_path(&who, &path, SwapLimit::ExactTarget(max_supply_amount, target_amount))
					.map_err(|e| PrecompileFailure::Error { exit_status:ExitError::Other(Cow::Borrowed(e.into()))})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: Output::default().encode_u128(value),
					logs: Default::default(),
				})
			}
			Action::AddLiquidity => {
				let who = input.account_id_at(1)?;
				let currency_id_a = input.currency_id_at(2)?;
				let currency_id_b = input.currency_id_at(3)?;
				let max_amount_a = input.balance_at(4)?;
				let max_amount_b = input.balance_at(5)?;
				let min_share_increment = input.balance_at(6)?;

				log::debug!(
					target: "evm",
					"dex: add_liquidity who: {:?}, currency_id_a: {:?}, currency_id_b: {:?}, max_amount_a: {:?}, max_amount_b: {:?}, min_share_increment: {:?}",
					who, currency_id_a, currency_id_b, max_amount_a, max_amount_b, min_share_increment,
				);

				<module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::add_liquidity(
					&who,
					currency_id_a,
					currency_id_b,
					max_amount_a,
					max_amount_b,
					min_share_increment,
					false,
				)
				.map_err(|e| PrecompileFailure::Error {
					exit_status: ExitError::Other(Cow::Borrowed(e.into())),
				})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: vec![],
					logs: Default::default(),
				})
			}
			Action::RemoveLiquidity => {
				let who = input.account_id_at(1)?;
				let currency_id_a = input.currency_id_at(2)?;
				let currency_id_b = input.currency_id_at(3)?;
				let remove_share = input.balance_at(4)?;
				let min_withdrawn_a = input.balance_at(5)?;
				let min_withdrawn_b = input.balance_at(6)?;

				log::debug!(
					target: "evm",
					"dex: remove_liquidity who: {:?}, currency_id_a: {:?}, currency_id_b: {:?}, remove_share: {:?}, min_withdrawn_a: {:?}, min_withdrawn_b: {:?}",
					who, currency_id_a, currency_id_b, remove_share, min_withdrawn_a, min_withdrawn_b,
				);

				<module_dex::Pallet<Runtime> as DEXManager<Runtime::AccountId, CurrencyId, Balance>>::remove_liquidity(
					&who,
					currency_id_a,
					currency_id_b,
					remove_share,
					min_withdrawn_a,
					min_withdrawn_b,
					false,
				)
				.map_err(|e| PrecompileFailure::Error {
					exit_status: ExitError::Other(Cow::Borrowed(e.into())),
				})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: 0,
					output: vec![],
					logs: Default::default(),
				})
			}
		}
	}
}
