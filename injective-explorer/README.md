# Injective Explorer Substreams

### Generate protos
```bash
make protogen
```

### Build substreams
```bash
make build
```

### Run substreams
```bash
substreams run substreams.yaml block_to_stats -e mainnet.injective.streamingfast.io:443 -s 64987400 rx-t +1000
```

To generate the `MsgExec` Protobuf:

```bash
buf generate buf.build/cosmos/cosmos-sdk --type="cosmos.authz.v1beta1.MsgExec"
```