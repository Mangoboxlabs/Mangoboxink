
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
                state.contract = await connectContract(web3, 'MBDirectory')
            }
        }
        const state = {
            contract:null
        }
		const mutations = {};
	 const actions = {
async terminalsOf ({rootState}, _projectId){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.terminalsOf(AccountId, {value, queryGasLimit},_projectId)
                    data = formatResult(data);
                    return data

			},
async primaryTerminalOf ({rootState}, _token,_projectId){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.primaryTerminalOf(AccountId, {value, queryGasLimit},_token,_projectId)
                    data = formatResult(data);
                    return data

			},
async isTerminalOf ({rootState}, _projectId,_terminal){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.isTerminalOf(AccountId, {value, queryGasLimit},_projectId,_terminal)
                    data = formatResult(data);
                    return data

			},
async setControllerOf ({rootState}, _projectId,_controller){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.setControllerOf( {storageDepositLimit, gasLimit},
                         //params
                  _projectId,_controller
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });

			},
async setTerminalsOf ({rootState}, _projectId,_terminals){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.setTerminalsOf( {storageDepositLimit, gasLimit},
                         //params
                  _projectId,_terminals
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });

			},
async setPrimaryTerminalOf ({rootState}, _projectId,_token,_terminal){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.setPrimaryTerminalOf( {storageDepositLimit, gasLimit},
                         //params
                  _projectId,_token,_terminal
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
