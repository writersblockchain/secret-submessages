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

const contractAddress = "secret1tv7g8u3ch5plzsftrhlpeahjflmnzrw22s3r9e";
const contractCodeHash =
  "070d13ab1859622abf0dd04c64bde3d8bb3221dc31c49e8084bd6e614c865983";

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
