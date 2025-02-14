import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet("desk pigeon hammer sleep only mistake stool december offer patrol once vacant");

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://pulsar.lcd.secretnodes.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

const contractAddress = "secret1cldglt6wvueva2akly4x3wvzzlevk2hxzv0cvq";
const contractCodeHash =
  "9c0d7d6d626a09c9b9193b23c22da5b6a3a68121a59e1f9772694b361d518e8a";

let query_count = async () => {
  const count = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_count: {},
    },
    code_hash: contractCodeHash,
  });

  console.log(count);
};
query_count();
