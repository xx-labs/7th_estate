const yaml = require('js-yaml');
const fs   = require('fs');
const Web3 = require('web3');
var Tx = require('ethereumjs-tx').Transaction;
const axios = require('axios');

exports.Blockchain = class Blockchain {
    constructor(configfile) {
        this.config = configfile;
        this.doc = this.loadConfig(this.config);
        this.web3 = new Web3(Web3.givenProvider || this.doc['node']); 
        this.account = this.web3.eth.accounts.privateKeyToAccount(this.doc['key']);
        this.apikey = this.doc['apikey']   
    }

    loadConfig(configfile) {
        try {
            return yaml.load(fs.readFileSync(configfile, 'utf8'));
        } catch (e) {
            console.log(e);
            return null;
        }
                
    }

    async postToBlockchain (data) {
        try{
            var data = this.web3.utils.toHex(data);

            var gas = await this.web3.eth.estimateGas({
                to: this.account.address,
                data: data,
            });


            var txcount = await this.web3.eth.getTransactionCount(this.account.address);

            var rawTx = {
                to: this.account.address,
                value: '0x00',
                data: data,
                nonce: txcount,
                gas: gas,
                gasLimit: this.web3.utils.toHex(314150),
                gasPrice: this.web3.utils.toHex(this.web3.utils.toWei('10', 'gwei'))
            }
            
            var tx = new Tx(rawTx, { chain: this.doc['chain'] });
            var pkeybuff = Buffer.from(this.account.privateKey.slice(2), 'hex');
            
            tx.sign(pkeybuff);
            tx = tx.serialize();

            return new Promise(async (resolve, reject) => {
                this.web3.eth
                    .sendSignedTransaction(this.web3.utils.toHex(tx))
                    .on("transactionHash", (hash) => {
                        resolve(hash);
                    })
                    .catch((err) => reject (err))
            })
        } catch(err) {
            console.log(err);
            throw(err)
        }

    }

    async getData () {
        var url = "https://api-ropsten.etherscan.io/api?"
        var uri = url + `module=account&action=txlist&address=${this.account.address}&startblock=0&endblock=99999999&sort=asc&apikey=${this.apikey}`
        
        var res = await axios.get(uri);
        return res.data

    }

    async getDataTx(data) {
        var datahex = this.web3.utils.toHex(data);
        var dataposted = await this.getData();
        
        for (var tx of dataposted['result']) {
            if (tx.input == datahex)
                return tx;
        }
    
        return null;
    }

    async checkReceipt(hash) {
        return await this.web3.eth.getTransactionReceipt(hash);
    }

}

