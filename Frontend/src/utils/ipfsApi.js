
/* eslint-disable */
import axios from "axios"
const ipfsAPI = require('ipfs-api');
const ipfs = ipfsAPI({host: 'cloudflare-ipfs.com', port: '443', protocol: 'https'});
const FormData = require('form-data');
function upload(){

}

const config = {
    APIKey: '907ad3abd90dd849cb50',
    APISecret: '727b0fe39e90cd54d246358711e1b01a1d7af5c466bb8888b761546077328738',
    JWT: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI3YjdkZDYzOC1mZDVkLTQ2NGMtYjY5Yi1kY2ViMTZhODBjOGEiLCJlbWFpbCI6Im1hbmdvYm94bGFic0BnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwicGluX3BvbGljeSI6eyJyZWdpb25zIjpbeyJpZCI6IkZSQTEiLCJkZXNpcmVkUmVwbGljYXRpb25Db3VudCI6MX0seyJpZCI6Ik5ZQzEiLCJkZXNpcmVkUmVwbGljYXRpb25Db3VudCI6MX1dLCJ2ZXJzaW9uIjoxfSwibWZhX2VuYWJsZWQiOmZhbHNlLCJzdGF0dXMiOiJBQ1RJVkUifSwiYXV0aGVudGljYXRpb25UeXBlIjoic2NvcGVkS2V5Iiwic2NvcGVkS2V5S2V5IjoiOTA3YWQzYWJkOTBkZDg0OWNiNTAiLCJzY29wZWRLZXlTZWNyZXQiOiI3MjdiMGZlMzllOTBjZDU0ZDI0NjM1ODcxMWUxYjAxYTFkN2FmNWM0NjZiYjg4ODhiNzYxNTQ2MDc3MzI4NzM4IiwiaWF0IjoxNjY2MzIwOTQxfQ.fUXVgaQvansmo3wlpB2QSlmLFdorMKszlg2U5qS5QKE'
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
    if(strHash&&strHash.length>5&&typeof strHash == "string"){
        let result = await axios.get(`https://cloudflare-ipfs.com/ipfs/${strHash}#x-ipfs-companion-no-redirect`,{})

        return result
    }else{
        return false
    }

}

export async function uploadFile(file){
    let data = new FormData();
    data.append('file', file);
    data.append('pinataOptions', '{"cidVersion": 1}');
    data.append('pinataMetadata', `{"name": ${file.name}, "keyvalues": {"company": "Pinata"}}`);
    var posdData = {
        method: 'post',
        url: 'https://api.pinata.cloud/pinning/pinFileToIPFS',
        headers: {
            'Authorization': config.JWT,
        },
        data : data
    };
    let result = await axios(posdData)
    console.log(result)
    return result
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
