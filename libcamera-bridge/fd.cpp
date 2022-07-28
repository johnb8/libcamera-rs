#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

#include <sys/mman.h>
#include <unistd.h>

size_t fd_len(int file) {
  long ret = lseek(file, 0, SEEK_END);
  if (ret < 0) {
    throw error_from_code(errno);
  }
  return ret;
}

BindMemoryBuffer mmap_plane(int file, size_t len) {
  void *address = mmap(nullptr, len, PROT_READ, MAP_SHARED, file, 0);
  // MAP_FAILED expands to a silly C-style cast.
  // NOLINTNEXTLINE(cppcoreguidelines-pro-type-cstyle-cast)
  if ((address == nullptr) || address == MAP_FAILED) {
    throw error_from_code(errno);
  }
  BindMemoryBuffer buffer{.inner = std::make_unique<MemoryBuffer>(
                              static_cast<uint8_t *>(address), len)};
  return buffer;
}
