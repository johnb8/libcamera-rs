#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

#include <sys/mman.h>
#include <unistd.h>

size_t fd_len(int fd) { return lseek(fd, 0, SEEK_END); }

BindMemoryBuffer mmap_plane(int fd, size_t len) {
  void *address = mmap(nullptr, len, PROT_READ, MAP_SHARED, fd, 0);
  if (!address || address == MAP_FAILED) {
    throw(BindErrorCode) errno;
  }
  BindMemoryBuffer buffer{.inner = std::make_unique<MemoryBuffer>(
                              static_cast<unsigned char *>(address), len)};
  return buffer;
}
