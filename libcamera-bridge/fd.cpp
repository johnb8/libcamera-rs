#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

#include <sys/mman.h>
#include <unistd.h>

size_t fd_len(int fd) {
  long ret = lseek(fd, 0, SEEK_END);
  if (ret < 0) {
    throw error_from_code(errno);
  }
  return ret;
}

BindMemoryBuffer mmap_plane(int fd, size_t len) {
  void *address = mmap(nullptr, len, PROT_READ, MAP_SHARED, fd, 0);
  // MAP_FAILED expands to a silly C-style cast.
  // NOLINTNEXTLINE(cppcoreguidelines-pro-type-cstyle-cast)
  if ((address == nullptr) || address == MAP_FAILED) {
    throw error_from_code(errno);
  }
  BindMemoryBuffer buffer{.inner = std::make_unique<MemoryBuffer>(
                              static_cast<unsigned char *>(address), len)};
  return buffer;
}
