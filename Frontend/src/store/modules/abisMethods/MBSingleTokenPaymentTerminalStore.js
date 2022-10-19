/* eslint-disable */
import connectContract from "@/api/connectContract"
import {formatResult} from "@/utils/formatUtils"
import Accounts from "@/api/Account.js";


const value = 0;
const queryGasLimit = -1;
const gasLimit = 3000n * 100000000n;
const storageDepositLimit = null;


async function judgeContract(web3) {
    if (!state.contract) {
        state.contract = await connectContract(web3, 'MBSingleTokenPaymentTerminalStore')
    }
}

const state = {
    contract: null
}
const mutations = {};
const actions = {
    async currentOverflowOf({rootState}, _terminal, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.currentOverflowOf(AccountId, {
            value,
            queryGasLimit
        }, _terminal, _projectId)
        data = formatResult(data);
        return data

    },
    async currentTotalOverflowOf({rootState}, _projectId, _decimals, _currency) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.currentTotalOverflowOf(AccountId, {
            value,
            queryGasLimit
        }, _projectId, _decimals, _currency)
        data = formatResult(data);
        return data

    },
    async recordPaymentFrom({rootState}, _payer, _amount, _projectId, _baseWeightCurrency, _beneficiary, _memo, _metadata) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.recordPaymentFrom({storageDepositLimit, gasLimit},
            //params
            _payer, _amount, _projectId, _baseWeightCurrency, _beneficiary, _memo, _metadata
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async getBalanceOf({rootState}, {_account, _projectId}) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.getBalanceOf(AccountId, {value, queryGasLimit}, _account, _projectId)
        data = formatResult(data);
        return data

    },
    async recordRedemptionFor({rootState}, _holder, _projectId, _tokenCount, _memo) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.recordRedemptionFor({storageDepositLimit, gasLimit},
            //params
            _holder, _projectId, _tokenCount, _memo
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async recordDistributionFor({rootState}, _projectId, _amount, _currency) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.recordDistributionFor({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount, _currency
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async recordUsedAllowanceOf({rootState}, _projectId, _amount, _currency) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.recordUsedAllowanceOf({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount, _currency
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async recordAddedBalanceFor({rootState}, _projectId, _amount) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.recordAddedBalanceFor({storageDepositLimit, gasLimit},
            //params
            _projectId, _amount
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
