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
        state.contract = await connectContract(web3, 'MBController')
    }
}

const state = {
    contract: null
}
const mutations = {};
const actions = {
    async launchProjectFor({rootState}, {
        _owner,
        _projectMetadata,
        _data,
        _metadata,
        _mustStartAtOrAfter,
        _groupedSplits,
        _fundAccessConstraints,
        _terminals,
        _memo
    }) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        /*eslint-disable*/
        _data.weight?_data.weight = BigInt(parseInt(_data.weight) * 10**18):""
        await state.contract.tx.launchProjectFor({storageDepositLimit, gasLimit},
            //params
            _owner, _projectMetadata, _data, _metadata, _mustStartAtOrAfter, _groupedSplits, _fundAccessConstraints, _terminals, _memo
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            console.log(result)
            dealResult(result, rootState.app.web3, state.contract, "Create")

        }).catch(err=>{
            reportErr(err)
        });

    },
    async getOwnerProjects({rootState}, owner) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.getOwnerProjects(AccountId, {value, queryGasLimit}, owner)
        data = formatResult(data);
        return data

    },
    async issueTokenFor({rootState}, {_projectId, _name, _symbol}) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.issueTokenFor({storageDepositLimit, gasLimit},
            //params
            _projectId, _name, _symbol
        ).signAndSend(AccountId, {signer: injector.signer}, result => {
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

    },
    async mintTokensOf({rootState}, _projectId, _tokenCount, _beneficiary, _memo, _preferClaimedTokens) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.mintTokensOf({storageDepositLimit, gasLimit},
            //params
            _projectId, _tokenCount, _beneficiary, _memo, _preferClaimedTokens
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
