const crypto = require('crypto');
const axios = require('axios');
require('dotenv').config();

const TOKEN = process.env.TOKEN;
const SECRET = process.env.SECRET;
const PLUG_DEVICE_ID = process.env.PLUG_DEVICE_ID;
const BASE_API_URL = 'https://api.switch-bot.com';

// 実行
// getDeviceData();
togglePlugPower();

function generateHeaders() {
    const t = Date.now();
    const nonce = crypto.randomUUID();
    const data = TOKEN + t + nonce;

    // HMAC-SHA256で署名を生成し、Base64エンコード後に大文字に変換
    const sign = crypto
        .createHmac('sha256', SECRET)
        .update(Buffer.from(data, 'utf-8'))
        .digest('base64');

    return {
        'Authorization': TOKEN,
        'sign': sign,
        'nonce': nonce,
        't': t,
        'Content-Type': 'application/json',
    };
}

async function requestAPI(api, method, payload = '') {

    const headers = generateHeaders();
    const url = BASE_API_URL + api;
    console.log("url: ", url);

    const options = {
        url: url,
        method: method,
        headers: headers,
        data: payload,
    };

    try {
        return await axios(options);
    } catch (error) {
        console.error('APIの呼び出しに失敗しました:', error.response ? error.response.data : error.message);
    }
}

async function getDeviceData() {
    const response = await requestAPI('/v1.1/devices', 'GET');
    const { deviceList } = response.data.body;

    // deviceTypeが "Plug Mini (JP)" のものを抽出
    const plugMiniJPDevices = deviceList.filter(
        device => device.deviceType === 'Plug Mini (JP)'
    );

    console.log('--- Plug Mini (JP) Devices ---');
    console.log(JSON.stringify(plugMiniJPDevices, null, 2));
}

async function togglePlugPower() {
    const api = '/v1.1/devices/' + PLUG_DEVICE_ID + '/commands';
    console.log('api: ', api);
    const body = JSON.stringify({
        "command": "toggle",
        "parameter": "default",
        "commandType": "command",
    });
    const response = await requestAPI(api, 'POST', body);
    // const { message } = response.data.message;
    const resData = response.data;

    console.log('--- Response Message (toggle plug power) ---');
    console.log(JSON.stringify(resData, null, 2));
}
