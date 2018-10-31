package hedera

// #cgo CFLAGS: -I ./include
// #cgo LDFLAGS: -l hedera
// #cgo darwin LDFLAGS: -L libs/x86_64-apple-darwin -framework Security -l c++
import "C"
