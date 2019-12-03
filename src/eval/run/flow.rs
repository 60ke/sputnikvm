//! Flow control instructions.

use ::Memory;
use bigint::{U256, M256};
use eval::{State,Account,Storage_Data};
use patch::Patch;
extern crate sgx_tstd as std;
use std::println;
use std::collections::HashMap;
use std::string::ToString;
pub fn sload<M: Memory + Default, P: Patch>(state: &mut State<M, P>) {
    pop!(state, index: U256);
    let value = state.account_state.storage_read(state.context.address, index).unwrap();
    let mut storage:Storage_Data = HashMap::new();
    storage.insert(index.to_string(),value.as_usize().to_string());
    let account = Account{
        address: "0x".to_string() + &state.context.address.hex(),
        balance_action: "".to_string(),
        balance: "0".to_string(),
        code: "".to_string(),
        storage
    };
    state.readset.accounts.push(account);
    push!(state, value);
    println!("sload state {:?} index {:?}  value:{:?} state.readset {:?}sputnikvm/src/eval/run/flow.rs:12",state.context,index,value,state.readset);

}

pub fn sstore<M: Memory + Default, P: Patch>(state: &mut State<M, P>) {
    pop!(state, index: U256, value: M256);
    state.account_state.storage_write(state.context.address, index, value).unwrap();
    let mut storage:Storage_Data = HashMap::new();
    storage.insert(index.to_string(),value.as_usize().to_string());
    // todo wql
    let account = Account{
        address: "0x".to_string() + &state.context.address.hex(),
        balance_action: "".to_string(),
        balance: "0".to_string(),
        code: "".to_string(),
        storage
    };
    state.writeset.accounts.push(account);
    println!("sstore state {:?} index {:?}  value:{:?}\n state.writeset {:?} sputnikvm/src/eval/run/flow.rs:12",state.context,index,value,state.writeset);
}

pub fn mload<M: Memory + Default, P: Patch>(state: &mut State<M, P>) {
    pop!(state, index: U256);
    let value = state.memory.read(index);
    push!(state, value);
    println!("call flow mload state.memory index:{:?} value:{:?}",index,value);
}

pub fn mstore<M: Memory + Default, P: Patch>(state: &mut State<M, P>) {
    pop!(state, index: U256, value: M256);
    state.memory.write(index, value).unwrap();
}

pub fn mstore8<M: Memory + Default, P: Patch>(state: &mut State<M, P>) {
    pop!(state, index: U256, value: M256);
    state.memory.write_raw(index, (value.0.low_u32() & 0xFF) as u8).unwrap();
}
