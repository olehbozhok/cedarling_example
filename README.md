# Example of usage Cedarling

## Example with simple policy store

To run simple example with simple policy store you can run

```bash
cargo run --example basic
```

## Example with real schema and policies

### Build policy

For simplicity algorithm of creating the policy store was described in the `build_policy.py` file.
You can execute it by running:

```bash
python build_policy.py
```

### Run rust script

In result you will get updated file `policy_store.json`.

To run main rust file execute next command:

```bash
cargo run
```

Eventually you will see the result of execution.
But it is error. Because we need to update JWT tokens and
provided schema is too complicated and current cedarling is not support nested types and parsing nested entities.
