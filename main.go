package main

/*
#cgo darwin LDFLAGS: -L./lib -lhello_ristretto
#include "./lib/hello_ristretto.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func main() {
	generateRistrettoPoint()
}

func generateRistrettoPoint() {
	buf := make([]byte, 32, 32)
	ptr := (*C.char)(unsafe.Pointer(&buf[0]))
	len := C.size_t(len(buf))
	C.generate(ptr, len)
	fmt.Printf("%v", buf)
}
