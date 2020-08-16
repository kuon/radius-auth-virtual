/*
 * This patch in added at the very start of the compilation to override the
 * Windows version and allow us to use newer API.
 */

#ifdef CONFIG_NATIVE_WINDOWS
#define WINVER 0x0600
#define _WIN32_WINNT 0x0600
#include <winsock2.h>
#include <wspiapi.h>
#endif

