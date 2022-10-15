import connectContract from "../../api/connectContract"
import {dealResult, formatResult} from "@/utils/formatUtils"
import Accounts from "../../api/Account.js";
const state = {
    contract:null
}

const value = 0;
const queryGasLimit = -1;
const gasLimit = 3000n * 1000000n;
const storageDepositLimit = null;

async function  judgeContract(web3){
    if(!state.contract){
        state.contract = await connectContract(web3, "MBProjects")
    }
}
const mutations = {

}


const actions = {
    async approve({rootState}){
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        console.log(gasLimit)
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)


        console.log(gasLimit)
        let data = await state.contract.tx.burn( {storageDepositLimit, gasLimit},
            //params
            0
        ).signAndSend(AccountId, { signer: injector.signer }, (result) => {
            console.error(result)
            dealResult(result,"",timeMemory)
        })
        data = formatResult(data);

        return data
    },
    async ownerOf({rootState}){
        const AccountId = await Accounts.accountAddress();
        await judgeContract(rootState.app.web3)
        console.log(state.contract)
        let data = await state.contract.query.ownerOf(AccountId, {value, queryGasLimit}, 2)
        data = formatResult(data);
        console.log(data)
        return data
    },


}
export default {
    namespaced: true,
    state,
    mutations,
    actions
}
