package main

/*
#include <stdint.h>
#include <stddef.h>
*/
import "C"

import (
	"sync"
	"unsafe"

	"github.com/metacubex/mihomo/hub"
	"github.com/metacubex/mihomo/hub/executor"
)

var (
	startMu   sync.Mutex
	started   bool
	lastErrMu sync.Mutex
	lastErr   string
)

func setLastErr(err string) {
	lastErrMu.Lock()
	lastErr = err
	lastErrMu.Unlock()
}

func clearLastErr() {
	lastErrMu.Lock()
	lastErr = ""
	lastErrMu.Unlock()
}

//export mihomo_start
func mihomo_start(config *C.uchar, config_len C.size_t) C.int {
	startMu.Lock()
	defer startMu.Unlock()

	clearLastErr()

	if started {
		setLastErr("mihomo already started")
		return C.int(-1)
	}
	if config == nil || config_len == 0 {
		setLastErr("config is empty")
		return C.int(-1)
	}

	// Convert C.size_t to a safe Go int length for GoBytes.
	maxInt := uint64(^uint(0) >> 1)
	if uint64(config_len) > maxInt {
		setLastErr("config too large")
		return C.int(-1)
	}

	n := int(uint(config_len))
	buf := C.GoBytes(unsafe.Pointer(config), C.int(n))

	if err := hub.Parse(buf); err != nil {
		setLastErr(err.Error())
		return C.int(-1)
	}

	started = true
	return 0
}

//export mihomo_stop
func mihomo_stop() C.int {
	startMu.Lock()
	defer startMu.Unlock()

	if !started {
		return 0
	}

	executor.Shutdown()
	started = false
	return 0
}

//export mihomo_last_error_copy
func mihomo_last_error_copy(out *C.char, out_len C.size_t) C.size_t {
	lastErrMu.Lock()
	errStr := lastErr
	lastErrMu.Unlock()

	b := []byte(errStr)
	if out == nil || out_len == 0 {
		return C.size_t(len(b))
	}

	max := int(out_len)
	// Reserve space for '\0'.
	if max <= 0 {
		return 0
	}
	if max == 1 {
		*(*byte)(unsafe.Pointer(out)) = 0
		return 0
	}

	// Clamp to avoid overly-large unsafe slice construction.
	const upperBound = 1 << 30
	if max > upperBound {
		max = upperBound
	}

	dst := (*[upperBound]byte)(unsafe.Pointer(out))[:max:max]
	toCopy := len(b)
	if toCopy > max-1 {
		toCopy = max - 1
	}
	copy(dst[:toCopy], b[:toCopy])
	dst[toCopy] = 0
	return C.size_t(toCopy)
}

func main() {}
