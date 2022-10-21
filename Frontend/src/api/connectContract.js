import abiMap from "./apiMap";
const { ContractPromise } = require('@polkadot/api-contract');


const ConnectContract = async (api,type,address) =>{
    if(!api){
      return
    }


    if(!address&&abiMap[type].address){
        address = abiMap[type].address
    }
    return new ContractPromise(api, abiMap[type].abi, address);
  }

export default ConnectContract
