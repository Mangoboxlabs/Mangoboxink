
    /* eslint-disable */
		import connectContract from "@/api/connectContract"
        import { formatResult} from "@/utils/formatUtils"
        import Accounts from "@/api/Account.js";
      
        
        const value = 0;
        const queryGasLimit = -1;
        const gasLimit = 3000n * 1000000n;
        const storageDepositLimit = null;

	
		async function  judgeContract(web3){
            if(!state.contract){
                state.contract = await connectContract(web3, 'MBProjects')
            }
        }
        const state = {
            contract:null
        }
		const mutations = {};
	 const actions = { 
async create_for ({rootState}, _owner,_metadata){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.create_for( {storageDepositLimit, gasLimit},
                         //params
                  _owner,_metadata
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });
				
			},
async balance_of ({rootState}, owner){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.balance_of(AccountId, {value, queryGasLimit},owner)
                    data = formatResult(data);
                    return data
				
			},
async owner_of ({rootState}, id){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.owner_of(AccountId, {value, queryGasLimit},id)
                    data = formatResult(data);
                    return data
				
			},
async get_approved ({rootState}, id){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.get_approved(AccountId, {value, queryGasLimit},id)
                    data = formatResult(data);
                    return data
				
			},
async is_approved_for_all ({rootState}, owner,operator){
				    await judgeContract(rootState.app.web3)
				    const AccountId = await Accounts.accountAddress();
                     let data = await state.contract.query.is_approved_for_all(AccountId, {value, queryGasLimit},owner,operator)
                    data = formatResult(data);
                    return data
				
			},
async set_approval_for_all ({rootState}, to,approved){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.set_approval_for_all( {storageDepositLimit, gasLimit},
                         //params
                  to,approved
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });
				
			},
async approve ({rootState}, to,id){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.approve( {storageDepositLimit, gasLimit},
                         //params
                  to,id
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });
				
			},
async transfer ({rootState}, destination,id){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.transfer( {storageDepositLimit, gasLimit},
                         //params
                  destination,id
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });
				
			},
async transfer_from ({rootState}, from,to,id){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.transfer_from( {storageDepositLimit, gasLimit},
                         //params
                  from,to,id
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });
				
			},
async mint ({rootState}, owner,id){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.mint( {storageDepositLimit, gasLimit},
                         //params
                  owner,id
                ).signAndSend(AccountId, { signer: injector.signer }, result => {
                    if (result.status.isInBlock) {
                        console.log('in a block');
                    } else if (result.status.isFinalized) {
                        console.log('finalized');
                    }
                });
				
			},
async burn ({rootState}, id){
                        await judgeContract(rootState.app.web3)
                        const injector = await Accounts.accountInjector();
                        const AccountId = await Accounts.accountAddress();
                        const timeMemory = new Date().getTime()
                        window.messageBox.push(timeMemory)
                        await state.contract.tx.burn( {storageDepositLimit, gasLimit},
                         //params
                  id
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
	