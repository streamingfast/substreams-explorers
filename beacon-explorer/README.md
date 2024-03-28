# Beacon Explorer

The Ethereum Beacon Chain Explorer consists of several Substreams modules showcasing the most basic operations that you can perform with Substreams on the Ethereum Beacon blockchain.

## Before You Begin

Make sure you have the [Substreams CLI installed](https://substreams.streamingfast.io/getting-started/installing-the-cli), and you know the [basic structure of a Substreams module](https://substreams.streamingfast.io/getting-started/quickstart). You must also provide a Pinax [API key](https://app.pinax.network) as `$SUBSTREAMS_API_KEY` environment variable

## Modules

The substreams package in this repository provides the following substreams modules:
- `map_block_meta` - streams some block metadata, such as slot number, root hash, spec version, timestamp, etc
- `map_block_full` - streams the entire block content to help you explore available fields
- `map_blobs` - streams blobs embedded into the beacon chain block


## Building the Substreams

First, generate the Protobuf code, which is the output of the Substreams:

```
> make protogen
```

Then, build the Rust code using the `cargo` command-line tool:

```
> make build
```

## Running Substreams modules

And now you can stream the corresponding substreams modules:

```
> make run map_block_meta
> make run map_block_full
> make run map_blobs
```

Alternatively, you can take advantage of `substreams gui` command for a nicer user interface:

```
> make gui map_block_meta
> make gui map_block_full
> make gui map_blobs
```