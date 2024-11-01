# Test TCP server

Server receives TCP connections, then for each connection it performs next logic:
- Read 2 numbers each 64 bits length. Numbers should be big endian.
  First number represents request ID. Second number is the size of response.
- Send back 2 numbers in the same order.
- Send back requested amount of bytes - 16 bytes. All bytes are `0x01`.

Minimum sent back amount of data is 16 bytes.

# How to test?

To send request for 40 bytes and print response back using `xxd`.

``` shell
SIZE=40 bash -c "printf '%016x%016x' 1 $SIZE | xxd -r -p | nc 127.0.0.1 12345 | xxd -l $SIZE"
```

``` shell
printf '%016x%016x%016x%016x' 1 40 2 50 | xxd -r -p | nc 127.0.0.1 12345 | xxd -l 90
```

