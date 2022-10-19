/* eslint-disable */
import connectContract from "@/api/connectContract"
import {formatResult} from "@/utils/formatUtils"
import {dealResult,reportErr} from "@/utils/dealResult"
import Accounts from "@/api/Account.js";


const value = 0;
const queryGasLimit = -1;
const gasLimit = 3000n * 100000000n;
const storageDepositLimit = null;


async function judgeContract(web3) {
    if (!state.contract) {
        state.contract = await connectContract(web3, 'MBTokenStore')
    }
}

const state = {
    contract: null
}
const mutations = {};
const actions = {
    async totalSupplyOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.totalSupplyOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async balanceOf({rootState}, {_holder, _projectId}) {
        console.log( _holder, _projectId)
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.balanceOf(AccountId, {value, queryGasLimit}, _holder, _projectId)
        data = formatResult(data);
        return data

    },
    async issueFor({rootState}, _projectId, _name, _symbol) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.issueFor({storageDepositLimit, gasLimit},
            //params
            _projectId, _name, _symbol
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            dealResult(result, rootState.app.web3, state.contract)
        }).catch(err=>{
            reportErr(err)
        });

    },
    async mintFor({rootState}, {_holder, _projectId, _amount, _preferClaimedTokens}) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        console.log(AccountId)
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)

        await state.contract.tx.mintFor({storageDepositLimit, gasLimit},
            //params
            _holder, _projectId, _amount, _preferClaimedTokens
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            dealResult(result, rootState.app.web3, state.contract)
        }).catch(err=>{
            reportErr(err)
        });

    },
    async claimFor({rootState}, _projectId, _holder, _amount) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.claimFor({storageDepositLimit, gasLimit},
            //params
            _projectId, _holder, _amount
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            dealResult(result, rootState.app.web3, state.contract)
        }).catch(err=>{
            reportErr(err)
        });

    },
    async burnFrom({rootState}, _holder, _projectId, _amount, _preferClaimedTokens) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.burnFrom({storageDepositLimit, gasLimit},
            //params
            _holder, _projectId, _amount, _preferClaimedTokens
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
