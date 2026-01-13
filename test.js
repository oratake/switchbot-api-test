const crypto = require('crypto');
const axios = require('axios');
require('dotenv').config();

const token = process.env.TOKEN;
const secret = process.env.SECRET;

async function getPlugMiniJP() {
    const t = Date.now();
    const nonce = crypto.randomUUID(); // ランダムなUUIDを生成
    const data = token + t + nonce;
    
    // HMAC-SHA256で署名を生成し、Base64エンコード後に大文字に変換
    const sign = crypto
        .createHmac('sha256', secret)
        .update(Buffer.from(data, 'utf-8'))
        .digest('base64');

    const options = {
        url: 'https://api.switch-bot.com/v1.1/devices',
        method: 'GET',
        headers: {
            'Authorization': token,
            'sign': sign,
            'nonce': nonce,
            't': t,
            'Content-Type': 'application/json',
        },
    };

    try {
        const response = await axios(options);
        const { deviceList } = response.data.body;

        // deviceTypeが "Plug Mini (JP)" のものを抽出
        const plugMiniJPDevices = deviceList.filter(
            device => device.deviceType === 'Plug Mini (JP)'
        );

        console.log('--- Plug Mini (JP) Devices ---');
        console.log(JSON.stringify(plugMiniJPDevices, null, 2));

    } catch (error) {
        console.error('APIの呼び出しに失敗しました:', error.response ? error.response.data : error.message);
    }
}

getPlugMiniJP();
