package main

/*
#cgo darwin CFLAGS: -I./lib
#cgo darwin LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
*/
import (
	"C"
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
