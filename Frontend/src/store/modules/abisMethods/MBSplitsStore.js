
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
                state.contract = await connectContract(web3, 'MBSplitsStore')
            }
        }
        const state = {
            contract:null
        }
		const mutations = {};
	 const actions = {
async splitsOf ({rootState}, _projectId,_group){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.splitsOf(AccountId, {value, queryGasLimit},_projectId,_group)
                    data = formatResult(data);
                    return data

			},
async set ({rootState}, _projectId,_groupedSplits){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.set( {storageDepositLimit, gasLimit},
                         //params
                  _projectId,_groupedSplits
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
