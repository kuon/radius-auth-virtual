
#include "bindings.h"
#include "ip_addr.h"

struct client {
  u8* shared_secret;
  size_t shared_secret_len;
  struct hostapd_radius_servers conf;
  struct radius_client_data* radius;
  u8 radius_identifier;
};

/*
 * Many functions can be macro, so we redefine them to ensure all
 * API are functions.
 */

void rc_free(void* ptr) {
  os_free(ptr);
}

static int did_init = 0;

static inline int rc_did_init() {
  return __atomic_load_n(&did_init, __ATOMIC_SEQ_CST);
}

static inline void set_rc_did_init(int i) {
  __atomic_store_n(&did_init, i, __ATOMIC_SEQ_CST);
}

int rc_init(void) {
  if (rc_did_init()) {
    return -1;
  }

  int res = os_program_init();
  if (res != 0) {
    return res;
  }

  res = eloop_init();
  if (res != 0) {
    os_program_deinit();
    return res;
  }
  set_rc_did_init(1);

  return 0;
}

void rc_deinit(void) {
  if (!rc_did_init()) {
    return;
  }
  eloop_destroy();
  os_program_deinit();
  set_rc_did_init(0);
}

static void logger_cb(void* ctx,
                      const u8* addr,
                      unsigned int module,
                      int level,
                      const char* txt,
                      size_t len) {
  printf("%s\n", txt);
}

void rc_enable_debug(rc_client client) {
  hostapd_logger_register_cb(logger_cb);
  client->conf.msg_dumps = 1;
}

rc_client rc_create_context(void) {
  rc_client client = os_zalloc(sizeof(*client));
  if (!client) {
    return NULL;
  }

  return client;
}

void rc_destroy_context(rc_client client) {
  radius_client_deinit(client->radius);
  os_free(client->conf.auth_servers);
  os_free(client->shared_secret);
  os_free(client);
}

int rc_add_server(rc_client client, const u8* ip, int ipv6, u16 port) {
  if (!client->shared_secret) {
    return -1;
  }

  int count = client->conf.num_auth_servers;

  client->conf.auth_servers =
      os_realloc(client->conf.auth_servers,
                 (count + 1) * sizeof(struct hostapd_radius_server));

  if (!client->conf.auth_servers) {
    return -1;
  }

  // Important to do it each time, as realloc moves the pointer
  client->conf.auth_server = client->conf.auth_servers;

  client->conf.num_auth_servers++;

  struct hostapd_radius_server* srv = client->conf.auth_servers + count;

  os_memset(srv, 0, sizeof(*srv));

  srv->shared_secret = client->shared_secret;
  srv->shared_secret_len = client->shared_secret_len;

  if (ipv6) {
    srv->addr.af = AF_INET6;
    os_memcpy(&srv->addr.u.v6, ip, sizeof(struct in6_addr));
  } else {
    srv->addr.af = AF_INET;
    os_memcpy(&srv->addr.u.v4, ip, sizeof(struct in_addr));
  }

  srv->port = port;

  return 0;
}

int rc_set_shared_secret(rc_client client, const char* txt) {
  size_t len = strlen(txt);

  client->shared_secret = os_malloc(len);
  if (!client->shared_secret) {
    return -1;
  }

  os_memcpy(client->shared_secret, txt, len);
  client->shared_secret_len = len;

  return 0;
}

/* Process the RADIUS frames from Authentication Server */
static RadiusRxResult receive_auth(struct radius_msg* msg,
                                   struct radius_msg* req,
                                   const u8* shared_secret,
                                   size_t shared_secret_len,
                                   void* data) {
  /* struct radius_ctx *ctx = data; */
  printf("Received RADIUS Authentication message; code=%d\n",
         radius_msg_get_hdr(msg)->code);

  radius_msg_dump(msg);
  /* We're done for this example, so request eloop to terminate. */
  eloop_terminate();

  return RADIUS_RX_PROCESSED;
}

static void start_example(void* eloop_ctx, void* timeout_ctx) {
  struct client* ctx = eloop_ctx;
  struct radius_msg* msg;

  printf("Sending a RADIUS authentication message\n");

  ctx->radius_identifier = radius_client_get_id(ctx->radius);
  msg = radius_msg_new(RADIUS_CODE_ACCESS_REQUEST, ctx->radius_identifier);
  if (msg == NULL) {
    printf("Could not create net RADIUS packet\n");
    return;
  }

  radius_msg_make_authenticator(msg);

  if (!radius_msg_add_attr(msg, RADIUS_ATTR_USER_NAME, (u8*)"testing", 7)) {
    printf("Could not add User-Name\n");
    radius_msg_free(msg);
    return;
  }

  if (!radius_msg_add_attr_user_password(
          msg, (u8*)"password", 8, ctx->conf.auth_server->shared_secret,
          ctx->conf.auth_server->shared_secret_len)) {
    printf("Could not add User-Password\n");
    radius_msg_free(msg);
    return;
  }

  /*
  if (!radius_msg_add_attr(msg, RADIUS_ATTR_NAS_IP_ADDRESS,
                           (u8*)&ctx->own_ip_addr, 4)) {
    printf("Could not add NAS-IP-Address\n");
    radius_msg_free(msg);
    return;
  }
  */

  if (radius_client_send(ctx->radius, msg, RADIUS_AUTH, NULL) < 0)
    radius_msg_free(msg);
}

int rc_finish_init(rc_client client) {
  client->radius = radius_client_init(&client, &client->conf);
  if (!client->radius) {
    return -1;
  }
  int res = radius_client_register(client->radius, RADIUS_AUTH, receive_auth,
                                   &client);

  if (res != 0) {
    return res;
  }

  // TODO, do that in another function (that take username and password)
  eloop_register_timeout(0, 0, start_example, client, NULL);

  eloop_run();

  return 0;
}
