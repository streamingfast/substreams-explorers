# Ethereum Explorer

The Ethereum Explorer consists of several Substreams modules showcasing the most basic operations that you can perform with Substreams on the Ethereum blockchain.

## Before You Begin

Make sure you have the [Substreams CLI installed](https://substreams.streamingfast.io/getting-started/installing-the-cli), and you know the [basic structure of a Substreams module](https://substreams.streamingfast.io/documentation/intro-getting-started/intro-evm). You must also provide an [authentication token](https://substreams.streamingfast.io/documentation/consume/authentication).

## Modules

The modules in this repository answer some interesting questions when developing a blockchain application:

### How Can You Get the Basic Information of a Block?

For every block, the `map_block_meta` module retrieves the most relevant information of the block (number, hash, and parent hash).

### How Can You Retrieve Transactions By Their From or To Fields?

Given any combination of two parameters (`from` and `to`), the `map_filter_transactions` filters a transaction among all transactions in the blockchain. This involves:

1. Providing the filters (only the `from` fields, only the `to` field, both `from` and `to` fields, or none)
1. Iterating over all the transactions.
1. Filter the transactions, according to the parameters provided. For example, `from == tx_from`, `from == tx_from and to == tx_to`.

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

### Running the "map_filter_transactions" Module

To run this module, you must provide a transaction hash, so that Substreams can filter the transactions accordingly. The Substreams manifest (`substreams.yaml`) contains a default transaction hash (`4faa877df84080a9d98b1e28294c4680bb141ec27a1a5dee009c3e02dfa65ab7`) in the `params` section, which you can update.

This module allows you to filter transactions by two fields: `from` (the address that created the transaction) and `to` (the address that received the transaction). The Substreams manifest (`substreams.yaml`) contains a sample filter in the `params` section:

yml
```
map_filter_transactions: "to=0xdAC17F958D2ee523a2206206994597C13D831ec7"
```
This filter only returns the transactions received by the USDT contract address (`0xdAC17F958D2ee523a2206206994597C13D831ec7`). Other possibilities for the filter include:

- Using just `from`: `map_filter_transactions: "from=0xdAC17F958D2ee523a2206206994597C13D831ec7"`
- Using `from` and `to`: `map_filter_transactions: "from=0x1A384fE78e92F3e41e1B21ad79CB93A557Ab4EfD&to=0xdAC17F958D2ee523a2206206994597C13D831ec7"`
- Without filters (returns all the transactions): remove the `map_filter_transactions` entry in the `params` section of the manifest.

The `4faa877df84080a9d98b1e28294c4680bb141ec27a1a5dee009c3e02dfa65ab7` transaction is at block number `17712040`. In order to avoid iterating over the full blockchain, the following command starts searching at block number `17712038`.

```bash
> substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_filter_transactions --start-block 17712038 --stop-block +3
Connected (trace ID 5d4ab3248f19fa87052f523259353ff2)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 17712038:
(hit 'm' to switch mode)

----------- BLOCK #17,712,038 (b96fc7e71c0daf69b19211c45fbb5c201f4356fb2b5607500b7d88d298599f5b) ---------------
{
  "@module": "map_filter_transactions",
  "@block": 17712038,
  "@type": "eth.transaction.v1.Transactions",
  "@data": {
    "transactions": [
      {
        "from": "b6692f7ae54e89da0269c1bfd685ccdfd41d2bf7",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "933b74565234ac9ca8389f7a49fad80099abf1be77e4bef5af69ade30127f30e"
      },
      {
        "from": "0543ba40d4b8b33dc5f7d163f41c6dc54cf1d923",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "51071d7a94fc6ecfec2aba477c26ff5098db3e36a287d43d13b763b3118b160b"
      },
      {
        "from": "09e52cbb57dce8cd2836effc44686b6008a84914",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "806ef36a1e022d52d00a288bc150676af0cb2bad6b5500378c8fc7253a0434fa"
      },
      {
        "from": "2f13d388b85e0ecd32e7c3d7f36d1053354ef104",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "0a9a5707b5d4047b1e44de9283f0c88606eac49b4eb132a61df0dffc20668ad0"
      },
      {
        "from": "4fac9d83ffad797072db8bd72cc544ad5ec45e4f",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "8466f371eed9b742a2ed869213dde10661e3df22366e258e09f68e37ca47b2c1"
      },
      {
        "from": "48c04ed5691981c42154c6167398f95e8f38a7ff",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "571670afd47e29fe901c1b17ed21fca6088cc9540efd684c5b7b4c1c1e748612"
      },
      {
        "from": "4c8e30406f5dbedfaa18cb6b9d0484cd5390490a",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "558031630b43c8c61e36d742a779f967f3f0102fa290111f6f6f9c2acaadf3ea"
      }
    ]
  }
}

----------- BLOCK #17,712,039 (1385f853d28b16ad7ebc5d51b6f2ef6d43df4b57bd4c6fe4ef8ccb6f266d8b91) ---------------
{
  "@module": "map_filter_transactions",
  "@block": 17712039,
  "@type": "eth.transaction.v1.Transactions",
  "@data": {
    "transactions": [
      {
        "from": "75e89d5979e4f6fba9f97c104c2f0afb3f1dcb88",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "43e0e1b6315c4cc1608d876f98c9bbf09f2a25404aabaeac045b5cc852df0e85"
      },
      {
        "from": "75e89d5979e4f6fba9f97c104c2f0afb3f1dcb88",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "619d94c33b027df694cbf32659aae51743623b4d1cb11c69d7d0e95cad63b712"
      },
      {
        "from": "75e89d5979e4f6fba9f97c104c2f0afb3f1dcb88",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "027cccdba1a127bcfb5bb39b5d89e3552e83c8c3c6dd13cf779d7720241e71b9"
      },
      {
        "from": "3d1d8a1d418220fd53c18744d44c182c46f47468",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "762350dcf3ab62ad515331436ce952ba5b3641bbf87c7d56c1e8a9f21473875c"
      },
      {
        "from": "a45c27ef3df487525b33a70cb0020de792dc7a3f",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "b9e08dfe7b1f4971ea96d1424c32548028bdeb62b2ee7f6775dd55d05c4d4ad6"
      },
      {
        "from": "9696f59e4d72e237be84ffd425dcad154bf96976",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "44f36363290969d8b581bb9a856bc9f2ca9a64e4a12e4db054927a45795480fa"
      },
      {
        "from": "e074f1967080cd7b9352c8cbe2d1d9cd121d4daf",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "8795aa5088fb13a21048c592316ad7da850a8f80f3ce417bc4d7d2bbeca3f596"
      },
      {
        "from": "fb8131c260749c7835a08ccbdb64728de432858e",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "0a907108aecaf909452f7035070a28f9cad6c51896763e760ea1f544a9b9edf3"
      },
      {
        "from": "e41febca31f997718d2ddf6b21b9710c5c7a3425",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "45c03fcbefcce9920806dcd7d638cef262ad405f8beae383fbc2695ad4bc9b1b"
      }
    ]
  }
}

----------- BLOCK #17,712,040 (31ad07fed936990d3c75314589b15cbdec91e4cc53a984a43de622b314c38d0b) ---------------
{
  "@module": "map_filter_transactions",
  "@block": 17712040,
  "@type": "eth.transaction.v1.Transactions",
  "@data": {
    "transactions": [
      {
        "from": "48c04ed5691981c42154c6167398f95e8f38a7ff",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "137799eea9fa8ae410c913e16ebc5cc8a01352a638f3ce6f3f29a283ad918987"
      },
      {
        "from": "7c0a7899f69a7034325ffee90355906cf72aeebb",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "132fc93b8a155c614001665a40381c8de9ad7519034352628c075e17a06d884b"
      },
      {
        "from": "180277c2f8bd489a4e27e261c6fbca079b6fa58f",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "06e74e08b51a0c03219c3aa12a871595516c1d466611ed848ea2ae8cbfb083ea"
      },
      {
        "from": "1440ec793ae50fa046b95bfeca5af475b6003f9e",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "83862ea45a6f777acd81a3469c54e347d3eb527cbee9fb673c6e312f7ae6fb83"
      },
      {
        "from": "89e51fa8ca5d66cd220baed62ed01e8951aa7c40",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "1b9e5059181ca90969ee423beea3073cf99faf8a91b73890303531ebd6c197ec"
      },
      {
        "from": "89e51fa8ca5d66cd220baed62ed01e8951aa7c40",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "ca1750068bee961ccd2e45679c9d9dadc5ba93fd3212c0f31361d39abe3ed36c"
      },
      {
        "from": "82cbcb64a2eb51622fb847c9c957fdac532712ac",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "284b6359cf66a010798738bb764f5cd015658e8f59273a49e19a855731f22bb8"
      },
      {
        "from": "f89d7b9c864f589bbf53a82105107622b35eaa40",
        "to": "dac17f958d2ee523a2206206994597c13d831ec7",
        "hash": "0544143b459969c9ed36741533fba70d6ea7069f156d2019d5362c06bf8d887f"
      }
    ]
  }
}

all done
```

### Running the "map_contract_events" Module

To run this module, you must provide the address of a smart contract. By default, the `params` section of the Substreams manifest contains the `0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d` address, corresponding to the Bored Ape Yacht Club smart contract. You can change this address to track any smart contract.

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
