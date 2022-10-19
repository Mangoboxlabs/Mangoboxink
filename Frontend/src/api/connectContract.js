
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
            address:'5EtyUxw17itJYGd9inLdVPvj4veN5FzS6SV4t8qY1nPe4LqE',
            abi:require('../abi/MBERC20PaymentTerminal.json')
        },
        MBSingleTokenPaymentTerminalStore: {
            address:'5FKX8VsjaGYNLynH3wdiP5i3TK3vcmYNPPtS2A6LdMjwxEWq',
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
            address:'5DyQtzR89WGjAchokxH5Ntc2mxC9ruaUGi4nKYZCSq6VUMg1',
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
            address:'5CcoJkt3SVmwmRy45Kp54U2hausgLbsKaYucqujeTGGyGop8',
            abi:require('../abi/MBController.json')
        },
        mbcontroller:{
            address:'5CcoJkt3SVmwmRy45Kp54U2hausgLbsKaYucqujeTGGyGop8',
            abi:require('../abi/MBController.json')
        }
    }

    if(!address&&abiMap[type].address){
        address = abiMap[type].address
    }
    return new ContractPromise(api, abiMap[type].abi, address);
  }

export default ConnectContract
