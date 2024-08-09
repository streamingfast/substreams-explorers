# Beacon Explorer

The Ethereum Beacon Chain Explorer consists of several Substreams modules showcasing the most basic operations that you can perform with Substreams on the Ethereum Beacon blockchain.

## Before You Begin

Make sure you have the [Substreams CLI installed](https://substreams.streamingfast.io/getting-started/installing-the-cli), and you know the [basic structure of a Substreams module](https://substreams.streamingfast.io/documentation/intro-getting-started/intro-evm). You must also provide a Pinax [API key](https://app.pinax.network) as `$SUBSTREAMS_API_KEY` environment variable

## Modules

The substreams package in this repository provides the following substreams modules:
- `map_block_meta` - streams some block metadata, such as slot number, root hash, spec version, timestamp, etc
- `map_block_full` - streams the entire block content to help you explore available fields
- `map_blobs` - streams blobs embedded into the beacon chain block


## Building the Substreams

First, generate the Protobuf code, which is the output of the Substreams:

```bash
> make protogen
```

Then, build the Rust code using the `cargo` command-line tool:

```bash
> make build
```

## Running Substreams modules

And now you can stream the corresponding substreams modules:

```bash
> make run map_block_meta
> make run map_block_full
> make run map_blobs
```

Alternatively, you can take advantage of `substreams gui` command for a nicer user interface:

```bash
> make gui map_block_meta
> make gui map_block_full
> make gui map_blobs
```

Or, you can just use `substreams` command to run it:
```bash
> substreams run -e eth-cl.substreams.pinax.network:443 map_block_meta -s -10
```

### Endpoints

Pinax offers the following beacon chain substreams endpoints:
- eth-cl.substreams.pinax.network:443
- gnosis-cl.substreams.pinax.network:443
- sepolia-cl.substreams.pinax.network:443
- holesky-cl.substreams.pinax.network:443
- chiado-cl.substreams.pinax.network:443