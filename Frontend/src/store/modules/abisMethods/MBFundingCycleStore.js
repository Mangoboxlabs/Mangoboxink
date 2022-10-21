<<<<<<< HEAD
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
        state.contract = await connectContract(web3, 'MBFundingCycleStore')
    }
}

const state = {
    contract: null
}
const mutations = {};
const actions = {
    async get({rootState}, _projectId, _configuration) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.get(AccountId, {value, queryGasLimit}, _projectId, _configuration)
        data = formatResult(data);
        return data

    },
    async latestConfiguredOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.latestConfiguredOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async queuedOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.queuedOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async currentOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.currentOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async currentBallotStateOf({rootState}, _projectId) {
        await judgeContract(rootState.app.web3)
        const AccountId = await Accounts.accountAddress();
        let data = await state.contract.query.currentBallotStateOf(AccountId, {value, queryGasLimit}, _projectId)
        data = formatResult(data);
        return data

    },
    async configureFor({rootState}, _projectId, _weight, _metadata, _mustStartAtOrAfter) {
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        await state.contract.tx.configureFor({storageDepositLimit, gasLimit},
            //params
            _projectId, _weight, _metadata, _mustStartAtOrAfter
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
=======

    /* eslint-disable */
		import connectContract from "@/api/connectContract"
        import { formatResult} from "@/utils/formatUtils"
        import Accounts from "@/api/Account.js";


        const value = 0;
        const queryGasLimit = -1;
        const gasLimit = 3000n * 100000000n;
        const storageDepositLimit = null;


		async function  judgeContract(web3){
            if(!state.contract){
                state.contract = await connectContract(web3, 'MBFundingCycleStore')
            }
        }
        const state = {
            contract:null
        }
		const mutations = {};
	 const actions = {
async get ({rootState}, _projectId,_configuration){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.get(AccountId, {value, queryGasLimit},_projectId,_configuration)
                    data = formatResult(data);
                    return data

			},
async latestConfiguredOf ({rootState}, _projectId){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.latestConfiguredOf(AccountId, {value, queryGasLimit},_projectId)
                    data = formatResult(data);
                    return data

			},
async queuedOf ({rootState}, _projectId){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.queuedOf(AccountId, {value, queryGasLimit},_projectId)
                    data = formatResult(data);
                    return data

			},
async currentOf ({rootState}, _projectId){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.currentOf(AccountId, {value, queryGasLimit},_projectId)
                    data = formatResult(data);
                    return data

			},
async currentBallotStateOf ({rootState}, _projectId){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.currentBallotStateOf(AccountId, {value, queryGasLimit},_projectId)
                    data = formatResult(data);
                    return data

			},
async configureFor ({rootState}, _projectId,_weight,_metadata,_mustStartAtOrAfter){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.configureFor( {storageDepositLimit, gasLimit},
                         //params
                  _projectId,_weight,_metadata,_mustStartAtOrAfter
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
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
>>>>>>> fdb68b1d4a522c718f88d5330404ac3598dc416e
