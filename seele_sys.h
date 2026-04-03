#ifndef seele_sys_h
#define seele_sys_h

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "stdint.h"
#include "stddef.h"

constexpr static const uint64_t seele_AF_UNIX = 1;

constexpr static const uint64_t seele_SOCK_STREAM = 1;

constexpr static const uint64_t seele_SOCK_DGRAM = 2;

constexpr static const uint64_t seele_SOCK_NONBLOCK = 2048;

constexpr static const uint64_t seele_SOCK_CLOEXEC = 524288;

constexpr static const uintptr_t seele_SIGNAL_AMOUNT = 22;

#endif  // seele_sys_h
