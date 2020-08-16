#include "includes.h"

#include "win_patch.h"

#include "common.h"
#include "os.h"

#include "eloop.h"
#include "radius/radius.h"
#include "radius/radius_client.h"

typedef struct context* rc_ctx;

/*
 * Free buffers coming from rc_* functions
 */
void rc_free(void* ptr);

/*
 * Init program
 * 0 on success
 */
int rc_init(void);

/*
 * Deinit program
 */
void rc_deinit(void);

rc_ctx rc_create_context(void);
void rc_destroy_context(rc_ctx ctx);

void rc_enable_debug(rc_ctx ctx);
int rc_add_attribute(rc_ctx ctx, u32 vendor, u8 subtype);
int rc_add_server(rc_ctx ctx,
                  const char* shared_secret,
                  const u8* ip,
                  int ipv6,
                  u16 port,
                  u16 timeout);


int rc_authenticate(rc_ctx ctx, const char* username, const char* password);
struct vendor_attr * rc_get_attributes(rc_ctx ctx, int *count);
