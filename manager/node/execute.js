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

const contractAddress = "secret1fegxxcn00zv64qduqypspu0zndshkxhus4k5ae";
const contractCodeHash =
  "f43a939a6fd4b7ebfec7e7bb6cdeae7b8798f7545864f61000b821906952937e";

let increase_count = async () => {
  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        increment: {
          contract: "secret1cldglt6wvueva2akly4x3wvzzlevk2hxzv0cvq",
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};

increase_count();
