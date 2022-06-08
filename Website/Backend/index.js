const express = require('express')
const config = require('./config.json')
const helmet = require('helmet')
const csrf = require('csurf')
const csrfProtection = csrf({ cookie: true })
const cookieParser = require('cookie-parser')
const fs = require('fs');
const Blockchain = require('./blockchain.js').Blockchain
const compression = require('compression');

CONFIG = "xxn_config.yaml";
const chain = new Blockchain(CONFIG);

// To turn off stack traces, stop information leak
process.env.NODE_ENV = 'production';

const app = express();
app.use(helmet()); // Manage security headers
app.use(compression())

VOTE_CODE_GROUP_SIZE = 4;
VOTE_CODE_GROUP_SIZE_P = VOTE_CODE_GROUP_SIZE + 1;
VOTE_CODE_NUM_GROUPS = 4;
VOTE_CODE_LENGTH = VOTE_CODE_NUM_GROUPS + VOTE_CODE_GROUP_SIZE_P;
VOTES_FILE_PATH = "votes.csv";

fs.access(VOTES_FILE_PATH, fs.F_OK, (err) => {
    if (err) {
        fs.appendFile(VOTES_FILE_PATH, "votecode\n", (err) => {
            return console.log(err);
        });
    }
})

// parse cookies
// we need this because "cookie" is true in csrfProtection
app.use(cookieParser())
app.use(express.json()); // to support JSON-encoded bodies
app.use(express.urlencoded({ extended: true })); // to support URL-encoded bodies

app.get(config.apipath + '/csrf', csrfProtection, (req, response) => {
    response.send({ csrf: req.csrfToken() });
})

app.post(config.apipath, csrfProtection, (req, response) => {
    console.log(req.body);
    let votecode = req.body.votecode;
    try{
        if (!checkVotecode(votecode)) {
            error(response, "Invalid votecode");
        }

        // Check if vote is in blockchain
        voteInBlockchain(votecode)
            .then(tx => {
                if (tx) {
                    error(response, "Vote already in blockchain");
                }
                // Post vote
            postVote(votecode)
                .then((result => {
                    if (!result){
			console.log("Error");
                        error(response, "Error posting in blockchain");
                    }
                
                    // Provide receipt
                    console.log("Hash sent: ", result)
                    sendResponse(response, {
                        hash: result,
                        status: "Validating your vote... You can check the status with the transaction hash"
                    });
                    fs.appendFile(VOTES_FILE_PATH, votecode + '\n', file_err);
                    // Provide proof that vote is on chain
                    // response.end();

                }))
                .catch((err) => {
                    error(response, err.message , err);
                    return;
                })
            })

        
    }
    catch(err) {
        error(response, "Error processing request", err);
        return;
    }
   
})

app.get(config.apipath + '/status/:hash', (req, response) => {
    var hash = req.params.hash;
    checkReceipt(hash)
    .then((status) => {
        console.log(status);
        sendResponse(response, {
            receipt: status ? JSON.stringify(status) : null,
            status:  status? "Your vote was successfully posted to blockchain, here is the transaction receipt": null
        });  

    })
    .catch((err) => {
        error(response, "Error getting tx status:" , err)
    });
})

function error(response, message, err = null) {
    console.log(message, err ? err.message : "");
    sendError(response);
    throw Error(message)
}

function file_err(err) {
    if (err) return console.log(err);

    console.log("Data written to file");
}

function checkVotecode(votecode) {
    console.log("Checking votecode", votecode);
    try{
        let codegroups = votecode.split("-");
        for (let i = 0; i < VOTE_CODE_NUM_GROUPS; i++){
        if (!checkParity(codegroups[i]))
            return false;
        }
        return true
    }
    catch(err) {
        console.log(err);
        return false;
    }
  }

function checkParity(code) {
    if (code.length != VOTE_CODE_GROUP_SIZE_P)
        return false;
    try {
        let parity = +code.slice(-1);
        let code_sum = code.slice(0, -1)
                        .split('')
                        .map(c => +c)
                        .reduce((sum, cur) => {
                            return sum + cur 
                        });
        let code_parity = (10 * VOTE_CODE_NUM_GROUPS - code_sum) % 10;
        return parity == code_parity;
    }
    catch(err) {
        console.log(err.message);
        return false;
    }
}

function sendError(response) {
    sendResponse(response, {errormessage: "There has been an error processing your vote"});
}

function sendResponse(response, messages) {
    response.end(JSON.stringify(messages))
}

async function voteInBlockchain(votecode) {

    console.log("Checking if votecode " + votecode + " is in blockchain");
    var data = {votecode:votecode};
    var tx = await chain.getDataTx(data);
    console.log("Found ", tx)

    return tx;
}

async function postVote(votecode) {
    console.log("Vote submitted: " + votecode);
    return await chain.postToBlockchain(JSON.stringify({votecode: votecode}));
}

async function checkReceipt(hash) {
    console.log("Checking status of tx " + hash);
    return await chain.checkReceipt(hash);
}

app.listen(config.port, config.host,() => console.log(`NodeServer started.`))
