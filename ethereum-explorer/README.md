# Ethereum Explorer

The Ethereum Explorer consists of several Substreams modules showcasing the most basic operations that you can perform with Substreams on the Ethereum blockchain.

## Before You Begin

Make sure you have the [Substreams CLI installed](https://substreams.streamingfast.io/getting-started/installing-the-cli), and you know the [basic structure of a Substreams module](https://substreams.streamingfast.io/getting-started/quickstart).

## Modules

The modules in this repository answer some interesting questions when developing a blockchain application:

### How Can You Get the Basic Information of a Block?

For every block, the `map_block_meta` module retrieves the most relevant information of the block (number, hash, and parent hash).

### How Can You Retrieve a Specific Transaction By Its Hash?

Given a transaction hash parameter (`tx_hash`), the `map_filter_transaction` filters a transaction among all transactions in the blockchain. This involves:

1. Iterating over all the transactions.
2. Filter the transactions, where the `hash` field is equal to the hash parameter (`hash == tx_hash`).

### How Can You Retrieve All the Events for a Specific Smart Contract?

Given a smart contract address parameter (`contract_address`), the `map_contract_events` module retrieves all the events related to a specific smart contract. This involves:

1. Iterating over all the smart contract transactions,
2. Filtering the transactions, where the `to` field is equal to the smart contract address parameter (`to == contract_address`).
3. For every filtered transaction, retrieve the logs.

## Running the Substreams

First, generate the Protobuf code, which are the outputs of the Substreams:

```
> make protogen
```

Then, build the Rust code using the `cargo` command-line tool:

```
> make build
```

Now, you are ready to run the Substreams. The Substreams contained in this project are independent of each other, so you must specify which Substreams module you want to run.

### Running the "map_block_meta" Module

In the following command, you retrieve the metadata of the block with number `17712040`. You specify the starting block by using the `--start-block` parameter.

```bash
> substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_block_meta --start-block 17712040 --stop-block +1
Connected (trace ID e98f04cd7ebb6191befbbf2e6668eafc)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 17712040:
(hit 'm' to switch mode)

----------- BLOCK #17,712,040 (31ad07fed936990d3c75314589b15cbdec91e4cc53a984a43de622b314c38d0b) ---------------
{
  "@module": "map_block_meta",
  "@block": 17712040,
  "@type": "eth.block_meta.v1.BlockMeta",
  "@data": {
    "number": "17712040",
    "hash": "31ad07fed936990d3c75314589b15cbdec91e4cc53a984a43de622b314c38d0b",
    "parentHash": "1385f853d28b16ad7ebc5d51b6f2ef6d43df4b57bd4c6fe4ef8ccb6f266d8b91"
  }
}

all done
```

### Running the "map_filter_transaction" Module

To run this module, you must provide a transaction hash, so that Substreams can filter the transactions accordingly. The Substreams manifest (`substreams.yaml`) contains a default transaction hash (`4faa877df84080a9d98b1e28294c4680bb141ec27a1a5dee009c3e02dfa65ab7`) in the `params` section, which you can update.

The `4faa877df84080a9d98b1e28294c4680bb141ec27a1a5dee009c3e02dfa65ab7` transaction is at block number `17712040`. In order to avoid iterating over the full blockchain, the following command starts searching at block number `17712038`.

```bash
> substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_filter_transaction --start-block 17712038 --stop-block +10
Connected (trace ID 9dce06621a0ec353213adeaf9f10ef79)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 17712038:
(hit 'm' to switch mode)

----------- BLOCK #17,712,038 (b96fc7e71c0daf69b19211c45fbb5c201f4356fb2b5607500b7d88d298599f5b) ---------------
----------- BLOCK #17,712,039 (1385f853d28b16ad7ebc5d51b6f2ef6d43df4b57bd4c6fe4ef8ccb6f266d8b91) ---------------
----------- BLOCK #17,712,040 (31ad07fed936990d3c75314589b15cbdec91e4cc53a984a43de622b314c38d0b) ---------------
{
  "@module": "map_filter_transaction",
  "@block": 17712040,
  "@type": "eth.transaction.v1.TransactionOption",
  "@data": {
    "transaction": {
      "from": "e93685f3bba03016f02bd1828badd6195988d950",
      "to": "902f09715b6303d4173037652fa7377e5b98089e",
      "hash": "4faa877df84080a9d98b1e28294c4680bb141ec27a1a5dee009c3e02dfa65ab7"
    }
  }
}

----------- BLOCK #17,712,041 (1af541642176c51580b54de214e955fba8bf1b82af569b81d4038956f2402a41) ---------------
----------- BLOCK #17,712,042 (43c6ebe5b89dd689a9f07468a04d0faf5274a46d0763056ea53b8b1e5ac32148) ---------------
----------- BLOCK #17,712,043 (2457f742913dbbdb171a8d8cc3b1ef8a383f8f547982700a646aa97581bfaeb8) ---------------
----------- BLOCK #17,712,044 (af7abef4f80d7c6f3111e761bb86f305e9847e544fb36b55ba5b64e6103bd5d3) ---------------
----------- BLOCK #17,712,045 (db4664944f7ca8aed5de798bf74ebbdbbeda60e58316b4291bfec61c7287fb17) ---------------
----------- BLOCK #17,712,046 (d9bce9b9210b7deb746720435d1eca99b87fe17aaf7d5055fcd54959e0c9932e) ---------------
----------- BLOCK #17,712,047 (3a5ffab8dacbaf89e10b66e9d2f7ebe65bddae4dcb5e5e8739f8b938f16f98ec) ---------------
all done
```

As you can see, the Substreams does not provide an output for the blocks where the transaction is not present.
You can check out the transaction at [Etherscan](https://etherscan.io/tx/0x4faa877df84080a9d98b1e28294c4680bb141ec27a1a5dee009c3e02dfa65ab7).

### Running the "map_filter_transaction" Module

To run this module, you must provide the address of a smart contract. By default, the `params` section of the Substreams manifest contains the `bc4ca0eda7647a8ab7c2061c2e118a18a936f13d` address, corresponding to the Bore Yacht Club smart contract. You can change this address to track any smart contract.

The logs of a smart contract might be split across thousands of Ethereum blocks, so for testing purposes, and to avoid iterating over the full blockchain, the following command limits starts searching at block number `17717995`.

```bash
> substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_contract_events --start-block 17717995 --stop-block +10
Connected (trace ID 834915f19a2b07a8a3aa0159cbbf56da)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 17717995:
(hit 'm' to switch mode)

----------- BLOCK #17,717,995 (bfecb26963a2cd77700754612185e0074fc9589d2d73abb90e362fe9e7969451) ---------------
----------- BLOCK #17,717,996 (7bf431a4f9df67e1d7e385d9a6cba41c658e66a77f0eb926163a7bbf6619ce20) ---------------
----------- BLOCK #17,717,997 (fa5a57231348f1f138cb71207f0cdcc4a0a267e2688aa63ebff14265b8dae275) ---------------
map_contract_events: log: //////////////////
{
  "@module": "map_contract_events",
  "@block": 17717997,
  "@type": "eth.event.v1.Events",
  "@data": {
    "events": [
      {
        "address": "bc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
        "topics": [
          "8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
          "000000000000000000000000e2a83b15fc300d8457eb9e176f98d92a8ff40a49",
          "0000000000000000000000000000000000000000000000000000000000000000",
          "00000000000000000000000000000000000000000000000000000000000026a7"
        ],
        "txHash": "f18291982e955f3c2112de58c1d0a08b79449fb473e58b173de7e0e189d34939"
      },
      {
        "address": "bc4ca0eda7647a8ab7c2061c2e118a18a936f13d",
        "topics": [
          "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
          "000000000000000000000000e2a83b15fc300d8457eb9e176f98d92a8ff40a49",
          "000000000000000000000000c67db0df922238979da0fd00d46016e8ae14cecb",
          "00000000000000000000000000000000000000000000000000000000000026a7"
        ],
        "txHash": "f18291982e955f3c2112de58c1d0a08b79449fb473e58b173de7e0e189d34939"
      }
    ]
  }
}

----------- BLOCK #17,717,998 (372ff635821a434c81759b3b23e8dac59393fc27a7ebb88b561c1e5da3c4643a) ---------------
----------- BLOCK #17,717,999 (43f0878e119836cc789ecaf12c3280b82dc49567600cc44f6a042149e2a03779) ---------------
----------- BLOCK #17,718,000 (439efaf9cc0059890a09d34b4cb5a3fe4b61e8ef96ee67673c060d58ff951d4f) ---------------
----------- BLOCK #17,718,001 (c97ca5fd26db28128b0ec2483645348bbfe998e9a6e19e3a442221198254c9ea) ---------------
----------- BLOCK #17,718,002 (9398569e46a954378b16e0e7ce95e49d0f21e6119ed0e3ab84f1c91f16c0c30e) ---------------
----------- BLOCK #17,718,003 (80bcd4c1131c35a413c32903ffa52a14f8c8fe712492a8f6a0feddbb03b10bba) ---------------
----------- BLOCK #17,718,004 (d27309ac29fe47f09fa4987a318818c325403863a53eec6a3676c2c2f8c069d9) ---------------
all done
```

The block number `17717997` contains a transaction (`f18291982e955f3c2112de58c1d0a08b79449fb473e58b173de7e0e189d34939`) that executes a method of the smart contract, so the Substreams retrieves the corresponding logs.
You can check out the logs at [Etherscan](https://etherscan.io/tx/0xf18291982e955f3c2112de58c1d0a08b79449fb473e58b173de7e0e189d34939#eventlog) and verify that topics are correct.
