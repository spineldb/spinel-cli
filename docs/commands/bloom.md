# Bloom Filter Commands

Bloom filters are probabilistic data structures used to test whether an element is a member of a set. False positives are possible, but false negatives are not.

| Command | Usage | Summary |
| --- | --- | --- |
| `BF.ADD` | `BF.ADD key item` | Adds an item to a Bloom filter. |
| `BF.EXISTS` | `BF.EXISTS key item` | Checks if an item exists in a Bloom filter. |
| `BF.MADD` | `BF.MADD key item [item ...]` | Adds multiple items to a Bloom filter. |
| `BF.MEXISTS` | `BF.MEXISTS key item [item ...]` | Checks if multiple items exist in a Bloom filter. |
| `BF.RESERVE` | `BF.RESERVE key error_rate capacity` | Creates a new Bloom filter with a given error rate and capacity. |
