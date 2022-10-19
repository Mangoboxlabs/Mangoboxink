
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
                state.contract = await connectContract(web3, 'MBOperatorStore')
            }
        }
        const state = {
            contract:null
        }
		const mutations = {};
	 const actions = {
async hasPermission ({rootState}, _operator,_account,_permissionIndex){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.hasPermission(AccountId, {value, queryGasLimit},_operator,_account,_permissionIndex)
                    data = formatResult(data);
                    return data

			},
async setOperator ({rootState}, _operator,_permissionIndexes){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.setOperator( {storageDepositLimit, gasLimit},
                         //params
                  _operator,_permissionIndexes
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
