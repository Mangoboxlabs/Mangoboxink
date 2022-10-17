/* eslint-disable*/
import connectContract from "../../api/connectContract"
import {formatResult,dealResult} from "@/utils/formatUtils"
import Accounts from "../../api/Account.js";
const state = {
    contract:null
}
const gasLimit = 3000n * 100000000n;
const storageDepositLimit = null;

async function  judgeContract(web3){
    if(!state.contract){
        state.contract = await connectContract(web3, "mbcontroller")
    }
}
const mutations = {

}
const actions = {

    async launchProjectFor({rootState}){
        await judgeContract(rootState.app.web3)
        const injector = await Accounts.accountInjector();
        const AccountId = await Accounts.accountAddress();
        console.log(AccountId)
        const timeMemory = new Date().getTime()
        window.messageBox.push(timeMemory)
        let data = await state.contract.tx.launchProjectFor( {storageDepositLimit, gasLimit},
                //params
            AccountId,
            //params
            "asd",
            {
                duration: 0,
                weight: 0,
                discountRate: 0,
                ballot:AccountId
            },
            0,
            0,
            [],
            [{
                terminal:AccountId,
                token:AccountId,
                distributionLimit:0,
                distributionLimitCurrency:0,
                overflowAllowance:0,
                overflowAllowanceCurrency:0
            }],
            [AccountId],
            ""
            ).signAndSend(AccountId, { signer: injector.signer }, (result) => {
            console.error(result)
            dealResult(result,"",timeMemory)
            if (result.isInBlock || result.isFinalized) {
                result.events
                    // find/filter for failed events
                    .filter(({ event }) =>
                        rootState.app.web3.events.system.ExtrinsicFailed.is(event)
                    )
                    // we know that data for system.ExtrinsicFailed is
                    // (DispatchError, DispatchInfo)
                    .forEach(({ event: { data: [error, info] } }) => {
                        if (error.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = state.contract.registry.findMetaError(error.asModule);
                            const { docs, method, section } = decoded;

                            console.log(`${section}.${method}: ${docs.join(' ')}`);
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            console.log(error.toString());
                        }
                    });
            }
        })
        data = formatResult(data);

        return data
    },

}
export default {
    namespaced: true,
    state,
    mutations,
    actions
}
