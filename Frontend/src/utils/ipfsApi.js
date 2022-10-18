
/* eslint-disable */
import axios from "axios"
const ipfsAPI = require('ipfs-api');
const ipfs = ipfsAPI({host: 'cloudflare-ipfs.com', port: '443', protocol: 'https'});
function upload(){

}

const config = {
    APIKey: 'f6ce1c9e73bacf765e20',
    APISecret: '8741b7ffd1d198ebaf463a934bd14941eee980b3235b0761abad72542123ce73',
    JWT: 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI4YzY3NjMwOS00NDAzLTQ2ZDQtODRmNi05ODE5MDgzNjgzOWMiLCJlbWFpbCI6ImNud2ViMDNAMTYzLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiRlJBMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2V9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiI3ZTI2MWIzNWJkMmU1MzQ3NzliYiIsInNjb3BlZEtleVNlY3JldCI6IjU2ZjYzNzJlYjY4NGNiNmIyZGNmYzg4NGM1YThiZDliMjZhOTI1M2NmNmMwMDNlMTg1MzIwZDcxZTU0YWI1ZmEiLCJpYXQiOjE2MzIyOTgyMDN9.9vd1v1h_0OP7UmuPmTas4FrPFN3QjAZlxCfNHxLdpA8'
}


function Utf8ArrayToStr(array) {
    var out, i, len, c;
    var char2, char3;

    out = "";
    len = array.length;
    i = 0;
    while(i < len) {
        c = array[i++];
        switch(c >> 4)
        {
            case 0: case 1: case 2: case 3: case 4: case 5: case 6: case 7:
            // 0xxxxxxx
            out += String.fromCharCode(c);
            break;
            case 12: case 13:
            // 110x xxxx   10xx xxxx
            char2 = array[i++];
            out += String.fromCharCode(((c & 0x1F) << 6) | (char2 & 0x3F));
            break;
            case 14:
                // 1110 xxxx  10xx xxxx  10xx xxxx
                char2 = array[i++];
                char3 = array[i++];
                out += String.fromCharCode(((c & 0x0F) << 12) |
                    ((char2 & 0x3F) << 6) |
                    ((char3 & 0x3F) << 0));
                break;
        }
    }

    return out;
}

export async function getIpfs(strHash){
    if(strHash&&strHash.length>5){
        let result = await axios.get(`https://cloudflare-ipfs.com/ipfs/${strHash}#x-ipfs-companion-no-redirect`,{})

        return result
    }else{
        return false
    }

}
export async function uploadJson(jsonData){
     let result = await axios.post('https://api.pinata.cloud/pinning/pinJSONToIPFS', jsonData, {
        headers: {
            // "Authorization": config.JWT
            "pinata_api_key": config.APIKey,
            "pinata_secret_api_key": config.APISecret
        }
    })
    return result
}
export function getFromPinata(strHash){
    axios.get('https://gateway.pinata.cloud/ipfs/' + strHash).then(res => {
        console.log(res)
        return res
    })

}
