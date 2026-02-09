# `s/`

The `s/` directory contains some simple convenience scripts to speed up and standardise working with this repository.

## `s/generate`

Generate many GitEHR repos for performance testing. Runs from the store root and will prompt for confirmation.

Example:
```
s/generate -repos 10000 -journal-entries 1000
```

Optional:
```
s/generate -repos 100 -journal-entries 10 --gitehr ./target/debug/gitehr
```

Skip journal creation:
```
s/generate -repos 10000 -journal-entries 1 --no-journal
```

Parallel repo creation (use with care):
```
s/generate -repos 1000 -journal-entries 100 --parallel 4
```
