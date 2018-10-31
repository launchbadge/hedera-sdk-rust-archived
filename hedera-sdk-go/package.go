package hedera

// #cgo CFLAGS: -I ../hedera-sdk-rust/src/bridge/include
// #cgo LDFLAGS: -L ../target/release -l hedera
// #cgo linux LDFLAGS: -l openssl -l c++
// #cgo darwin LDFLAGS: -framework Security -l c++
import "C"
