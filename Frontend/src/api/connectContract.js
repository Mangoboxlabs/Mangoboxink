
const { ContractPromise } = require('@polkadot/api-contract');


const ConnectContract = async (api,type,address) =>{
    if(!api){
      return
    }
    const abiMap ={
        MBToken:{
            address:'5FrnqmeaHvKuoiADjDCv3yG46rWvbvrWuiiq5eTTr3Kxqh2w',
            abi:require("../abi/MBToken.json")
        },
        MBOperatorStore: {
            address:'5DFNPZfZFakmBZtNLaEiGrMvCQcMtJJ4ynatujPZXFKZ53dj',
            abi:require("../abi/MBOperatorStore.json")
        },
        mbdirectory:{
            address:'5F83WaSxEXJqDmM1KgTeHLsBJrvVvmhvo5xVRQvBUVGNfWY2',
            abi:require('../abi/mbdirectory.json')
        },
        MBERC20PaymentTerminal:{
            address:'5Gx2WFYjXC8yv5z2hSpEASobNVQ45d5gccGjRd6AgPMM7qqC',
            abi:require('../abi/MBERC20PaymentTerminal.json')
        },
        MBSingleTokenPaymentTerminalStore: {
            address:'5FP3znY5SuthDe6xNE7oMyV5TPXdCsV6RBCNRRdCfRFdGuuy',
            abi:require('../abi/MBSingleTokenPaymentTerminalStore.json')
        },
        MBSplitsStore: {
            address:'5EhanZVL9bmiT4F44jFhFVrH6QPhGWiyhKV6t8xJ3nvYsDUk',
            abi: require('../abi/MBSplitsStore.json')
        },
        MBPrices: {
            address:'5HSJpZDVNVzQL5HvqGmxbeYUitQ8whKbv92bB6cr9tVVoZk2',
            abi:require('../abi/MBPrices.json')
        },
        MBTokenStore: {
            address:'5CPzXwK8U7xjoU56S3HjBCArPTLCc7KCVcyAhaDntkz5LsDx',
            abi:require('../abi/MBTokenStore.json')
        },
        MBProjects: {
            address:'5CLiYJnNrEeh6xdTtegSLvV3LapG91PvNoByUK946XqsChte',
            abi:require('../abi/MBProjects.json')
        },
        MBFundingCycleStore: {
            address:'5ExDF1h8shSgpFoTDTZFt6q8H9HCwvwtpvXLBntyCsbRaoo3',
            abi: require('../abi/MBFundingCycleStore.json')
        },
        MBController:{
            address:'5GuDNJ8FCGCxWEtzX2QPhzdFPjcW4tDAXv2no5dJBSsvJwas',
            abi:require('../abi/MBController.json')
        },
        mbcontroller:{
            address:'5GuDNJ8FCGCxWEtzX2QPhzdFPjcW4tDAXv2no5dJBSsvJwas',
            abi:require('../abi/MBController.json')
        }
    }

    if(abiMap[type].address){
        address = abiMap[type].address
    }
    return new ContractPromise(api, abiMap[type].abi, address);
  }

export default ConnectContract
