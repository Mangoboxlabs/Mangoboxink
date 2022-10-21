/* eslint-disable */
import connectContract from "@/api/connectContract"
import {formatResult} from "@/utils/formatUtils"
import Accounts from "@/api/Account.js";
import {dealResult,reportErr} from "@/utils/dealResult"

const value = 0;
const queryGasLimit = -1;
const gasLimit = 3000n * 100000000n;
const storageDepositLimit = null;


async function judgeContract(web3) {
    if (!state.contract) {
        state.contract = await connectContract(web3, 'MBERC20PaymentTerminal')
    }
}

const state = {
    contract: null
}
const mutations = {};
const actions = {
    async currentEthOverflowOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.currentEthOverflowOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async getPayRecords({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.getPayRecords(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async heldFeesOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.heldFeesOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async pay({rootState}, {
        _projectId,
        _amount,
        _token,
        _beneficiary,
        _minReturnedTokens,
        _preferClaimedTokens,
        _memo,
        _metadata
    }) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.pay({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount, _token, _beneficiary, _minReturnedTokens, _preferClaimedTokens, _memo, _metadata
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            dealResult(result, rootState.app.web3, state.contract, "pay")
        }).catch(err=>{
            reportErr(err)
        });

    },
    async redeemTokensOf({rootState}, {_holder, _projectId, _tokenCount, _token, _minReturnedTokens, _beneficiary, _memo}) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.redeemTokensOf({storageDepositLimit, gasLimit},
            //params
            _holder, _projectId, _tokenCount, _token, _minReturnedTokens, _beneficiary, _memo
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            dealResult(result, rootState.app.web3, state.contract, "Burn")
        }).catch(err=>{
            reportErr(err)
        });
    },
    async distributePayoutsOf({rootState}, _projectId, _amount, _currency, _token, _minReturnedTokens, _memo) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.distributePayoutsOf({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount, _currency, _token, _minReturnedTokens, _memo
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async useAllowanceOf({rootState}, _projectId, _amount, _currency, _token, _minReturnedTokens, _beneficiary, _memo) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.useAllowanceOf({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount, _currency, _token, _minReturnedTokens, _beneficiary, _memo
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async addToBalanceOf({rootState}, _projectId, _amount, _token, _memo, _metadata) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.addToBalanceOf({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount, _token, _memo, _metadata
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
}
export default {
    namespaced: true,
    mutations,
    state,
    actions
}
