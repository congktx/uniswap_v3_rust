import 'dotenv/config'
import { SigningCosmWasmClient, Secp256k1HdWallet } from "cosmwasm"
import * as fs from "fs";
import { Decimal } from "@cosmjs/math";

// This is your rpc endpoint
const rpcEndpoint = "https://testnet-rpc.orai.io"

// Using a random generated mnemonic
const deploy_mnemonic = process.env.DEPLOY_MNEMONIC

async function main() {
    const deploy_wallet = await Secp256k1HdWallet.fromMnemonic(deploy_mnemonic, { prefix: "orai" })
    const deploy_client = await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        deploy_wallet,
        {
            gasPrice: {
                denom: "orai",
                amount: Decimal.fromUserInput("0.001", 6)
            }
        }
    );
    const deploy_account = await deploy_wallet.getAccounts()
    const deploy_address = deploy_account[0].address
    console.log(await deploy_client.getBalance(deploy_address, "orai"))

    const cw20_address = process.env.CW20_CONTRACT_ADDRESS
    // const contract_address = process.env.CONTRACT_ADDRESS

    const fee = "auto"
    //=====================================DEPLOY========================================

    //wasm -> wasmCode
    const path = "artifacts/cosmwasm_base.wasm"
    const wasmCode = new Uint8Array(fs.readFileSync(path))

    //upload
    const upload = await deploy_client.upload(deploy_address, wasmCode, fee)
    //instantiate msg
    const instantiate_msg = {
    }
    const res = await deploy_client.instantiate(deploy_address, upload.codeId, instantiate_msg, "LABEL_SOMETHING", fee)
    console.log(res)
    //===================================================================================


    // //=====================================EXECUTE=======================================
    // //OWNER
    // const execute_something_owner = await deploy_client.execute(
    //     deploy_address, contract_address, { msg: {} }, fee)
    // console.log(execute_something_owner)

    // //USER
    // const execute_something_user = await user_client.execute(
    //     user_address, contract_address, { msg: {} }, fee)
    // console.log(execute_something_user)

    // //EXECUTE WITH TOKEN
    // const execute_native_deposit = await user_client.execute(
    //     user_address, contract_address,
    //     { msg: {} }, fee, null, [{ denom: "orai", amount: (0.1 * (10 ** 6)).toString() }])
    // console.log(execute_native_deposit)
    // //===================================================================================

    // //======================================QUERY========================================
    // const query_something = await user_client.queryContractSmart(
    //     contract_address, { msg: {} })
    // console.log(query_something)

    // //===================================================================================
}


main();