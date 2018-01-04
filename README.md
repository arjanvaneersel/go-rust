# go-rust
Invoking a Rust library from Go via CGO

## Structure
lib/hello/src contains the Rust source of the library

main.go contains the Go source code

## Usage
Easiest way is to use the Makefile

**make library**    will compile the Rust library and copy the .so file to the lib directory

**make build**      will build the Go binary

**make all**        will perform all the above mentioned tasks

**make run**        will execute the binary

**make clean**      will clean the build files

## Explanation
Invoking the Rust library is done via CGO.

```
/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
*/
import "C"
```
1. Tell CGO to include the library
2. Include the .h file
3. Import C

*Important: The import "C" statement should directly follow after the CGO block. No blank line should be between the block and the statement.*
