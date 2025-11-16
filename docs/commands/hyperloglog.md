# HyperLogLog Commands

HyperLogLog is a probabilistic data structure used for estimating the cardinality (number of unique elements) of a set.

| Command | Usage | Summary |
| --- | --- | --- |
| `PFADD` | `PFADD key element [element ...]` | Adds the specified elements to the specified HyperLogLog. |
| `PFCOUNT` | `PFCOUNT key [key ...]` | Returns the approximated cardinality of the set(s) observed by the HyperLogLog(s). |
| `PFMERGE` | `PFMERGE destinationkey sourcekey [sourcekey ...]` | Merges multiple HyperLogLog values into a single one. |
