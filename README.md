# ruffman

This is the [Huffman Compression Algorithm](https://en.wikipedia.org/wiki/Huffman_coding) implemented in Rust.

## External Dependencies

- [ordered-float](https://docs.rs/ordered-float/latest/ordered_float/)
- [itertools](https://docs.rs/itertools/latest/itertools/)

## How to use ruffman

To execute the program you do:

`cargo run FILE_PATH`

It is **mandatory** to provide a file path. Ruffman will locate the file to start the encoding and decoding processes.
Here it is an example of output:

```
File path provided: dom_casmurro.txt

Encoded with success!

Compression Rate: 41.40452177483511%

Decoded with success!

The files (original and decompressed) are equal!
```

## Developers

- EduardoLR10
- ribeirotomas1904
- fparadas

## Dr.Nekoma

Builded live on [twitch](https://www.twitch.tv/drnekoma) and archived on [youtube](https://www.youtube.com/channel/UCMyzdYsPiBU3xoqaOeahr6Q)
