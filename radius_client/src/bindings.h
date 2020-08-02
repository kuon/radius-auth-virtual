#include "includes.h"

#include "win_patch.h"

#include "common.h"
#include "os.h"
#include "base64.h"

#include "eloop.h"
#include "radius/radius.h"
#include "radius/radius_client.h"

typedef struct client *rc_client;


/*
 * Free buffers coming from rc_* functions
 */
void rc_free(void *ptr);

/*
 * Init program
 * 0 on success
 */
int rc_init(void);

/*
 * Deinit program
 */
void rc_deinit(void);



rc_client rc_create_context(void);
void rc_destroy_context(rc_client ctx);

int rc_add_server(rc_client ctx, const u8 * ip, int ipv6, u16 port);
int rc_set_shared_secret(rc_client ctx, const char *txt);
void rc_enable_debug(rc_client ctx);
int rc_finish_init(rc_client ctx);

