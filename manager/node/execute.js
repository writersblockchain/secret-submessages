import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar3.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

const contractAddress = "secret1pax555s93vnrawfzhpdgd2yc4w7kt7mefzxy7h";
const contractCodeHash =
  "83c09e8c3114a93b6e989a8a677d8c5b969b6d49e71452ac010c91524b75d8ca";

let increase_count = async () => {
  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        increment: {
          contract: "secret14q0jeyflxsd43zq3j82vkp08vp47r5ftt3glfr",
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};

increase_count();
