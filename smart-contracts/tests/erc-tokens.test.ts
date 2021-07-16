import { expect } from "chai";
import { artifacts, network, patract } from "redspot";

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

describe("ERC tokens", () => {
  after(() => {
    return api.disconnect();
  });

  async function setup(contractName = 'erc20') {
    await api.isReady
    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const sender = await getRandomSigner(Alice, "10000 UNIT");

    const contractFactory = await getContractFactory(contractName, sender.address);
    const contract = await contractFactory.deploy("new", "1000");
    const abi = artifacts.readArtifact(contractName);
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice };
  }

  it("Assigns initial balance", async () => {
    const erc20 = await setup('erc20');
    const resultErc20 = await erc20.contract.query.balanceOf(erc20.sender.address);
    expect(resultErc20.output).to.equal(1000);
  });

  it("Transfer adds amount to destination account", async () => {
    const { contract, receiver } = await setup();

    await expect(() =>
      contract.tx.transfer(receiver.address, 7)
    ).to.changeTokenBalance(contract, receiver, 7);

    await expect(() =>
      contract.tx.transfer(receiver.address, 7)
    ).to.changeTokenBalances(contract, [contract.signer, receiver], [-7, 7]);
  });

  it("Transfer emits event", async () => {
    const { contract, sender, receiver } = await setup();

    await expect(contract.tx.transfer(receiver.address, 7))
      .to.emit(contract, "Transfer")
      .withArgs(sender.address, receiver.address, 7);
  });

  it("Can not transfer above the amount", async () => {
    const { contract, receiver } = await setup();

    await expect(contract.tx.transfer(receiver.address, 1007)).to.not.emit(
      contract,
      "Transfer"
    );
  });

  it("Can not transfer from empty account", async () => {
    const { contract, Alice, sender } = await setup();

    const emptyAccount = await getRandomSigner(Alice, "10 UNIT");

    await expect(
      contract.connect(emptyAccount).tx.transfer(sender.address, 7)
    ).to.not.emit(contract, "Transfer");
  });
});
