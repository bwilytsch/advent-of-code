# Run
Fill in some stuff here

# Benchmark
```bash
$ go test -bench <dir> -benchmem
```

## Write to file
```bash
$ go test -bench <dir> -benchmem > <file.txt>
```

## View benchmarks

```bash
$ benchstat <file_1> <file_2>
```
# Run specific tests
```bash
$ go test <test_file> ...<any_file_used_in_test_file>
```
