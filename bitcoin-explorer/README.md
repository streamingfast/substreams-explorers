# Bitcoin Explorer

The Bitcoin Explorer consists of several Substreams modules showcasing the most basic operations that you can perform with Substreams on the Bitcoin blockchain.

## Before You Begin

Make sure you have the [Substreams CLI installed](https://substreams.streamingfast.io/getting-started/installing-the-cli), and you know the [basic structure of a Substreams module](https://substreams.streamingfast.io/getting-started/quickstart). You must also provide an [authentication token](https://substreams.streamingfast.io/reference-and-specs/authentication).

## Modules

The modules in this repository answer some interesting questions when developing a blockchain application:

### How Can You Get the Basic Information of a Block?

For every block, the `map_block_meta` module retrieves the most relevant information of the block (number, hash, and parent hash).

### How Can You Retrieve Transactions

For every block, the `map_transactions` modules summaries transaction number of vin and vout, the total valut of btc sent 


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

In the following command, you retrieve the metadata of the block with number `819113`. You specify the starting block by using the `--start-block` parameter.

```bash
> substreams run -e mainnet.btc.streamingfast.io:443 substreams.yaml map_block_meta --start-block 819113 --stop-block +1
Connected (trace ID dabf2d0d5f8074a36da855579279e23c)
Progress messages received: 0 (0/sec)
Backprocessing history up to requested target block 819113:
(hit 'm' to switch mode)





----------- BLOCK #819,113 (00000000000000000003aefe02ab07a4a7c5a27bb674b062732286a71ab1ab51) ---------------
{
  "@module": "map_block_meta",
  "@block": 819113,
  "@type": "btc.block_meta.v1.BlockMeta",
  "@data": {
    "number": "819113",
    "hash": "00000000000000000003aefe02ab07a4a7c5a27bb674b062732286a71ab1ab51",
    "parentHash": "000000000000000000001be3e5929be506d4e2c8cd1b7ba4fa9422032434368f"
  }
}

all done
```