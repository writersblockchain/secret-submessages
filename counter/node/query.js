import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet("desk pigeon hammer sleep only mistake stool december offer patrol once vacant");

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar3.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

const contractAddress = "secret14q0jeyflxsd43zq3j82vkp08vp47r5ftt3glfr";
const contractCodeHash =
  "d3474b3c15ce262c78746f3536cd5f50657f0bc0b4020963947005134583e593";

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
