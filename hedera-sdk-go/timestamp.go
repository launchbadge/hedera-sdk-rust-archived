package hedera

// #include "hedera-timestamp.h"
import "C"
import "unsafe"

type Timestamp struct {
	Seconds int64
	Nanos   int32
	// Added by `go tool cgo` to make this struct C compatible
	padding [4]byte
}

func NewTimestamp() Timestamp {
	response := C.hedera_timestamp_new()
	return *((*Timestamp)(unsafe.Pointer(&response)))
}
