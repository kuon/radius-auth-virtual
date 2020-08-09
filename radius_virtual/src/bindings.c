#include "bindings.h"
#include "ip_addr.h"

struct server_state {
  struct hostapd_radius_server radius_server;
  int timeout;
};

struct vendor_attr {
  u32 vendor;
  u8 subtype;
  u8* data;
  size_t len;
};

struct context {
  struct server_state* servers;
  int server_count;
  int current_server;
  struct hostapd_radius_servers conf;
  struct radius_client_data* radius;

  struct vendor_attr* vendor_attrs;
  int vendor_attrs_count;

  const char* username;
  const char* password;

  int did_timeout;
  int result_code;

  u8 radius_identifier;
};

enum { RC_ACCEPT = 0, RC_REJECT, RC_ERROR, RC_NO_SERV, RC_SERV_TIMEOUT };

/*
 * Many functions can be macro, so we redefine them to ensure all
 * API are functions.
 */
void rc_free(void* ptr) {
  os_free(ptr);
}

static int init_lock = 0;

static inline int rc_init_lock() {
  return __atomic_load_n(&init_lock, __ATOMIC_SEQ_CST);
}

static inline void set_rc_init_lock(int i) {
  __atomic_store_n(&init_lock, i, __ATOMIC_SEQ_CST);
}

int rc_init(void) {
  if (rc_init_lock()) {
    return -1;
  }

  set_rc_init_lock(1);

  int res = os_program_init();
  if (res != 0) {
    return res;
  }

  res = eloop_init();
  if (res != 0) {
    os_program_deinit();
    set_rc_init_lock(0);
    return res;
  }

  wpa_debug_level = MSG_ERROR;

  return 0;
}

void rc_deinit(void) {
  if (!rc_init_lock()) {
    return;
  }
  eloop_destroy();
  os_program_deinit();
  set_rc_init_lock(0);
}

static void logger_cb(void* ctx,
                      const u8* addr,
                      unsigned int module,
                      int level,
                      const char* txt,
                      size_t len) {
  if (addr && *addr) {
    wpa_printf(MSG_DEBUG, "STA " MACSTR ": %s", MAC2STR(addr), txt);
  } else
    wpa_printf(MSG_DEBUG, "%s", txt);
}

rc_ctx rc_create_context(void) {
  rc_ctx ctx = os_zalloc(sizeof(*ctx));
  if (!ctx) {
    return NULL;
  }

  ctx->conf.retry_primary_interval = 10;

  return ctx;
}

void rc_destroy_context(rc_ctx ctx) {
  for (int i = 0; i < ctx->server_count; i++) {
    struct server_state* state = ctx->servers + i;
    os_free(state->radius_server.shared_secret);
  }

  for (int i = 0; i < ctx->vendor_attrs_count; i++) {
    struct vendor_attr* attr = ctx->vendor_attrs + i;
    os_free(attr->data);
  }

  os_free(ctx->servers);
  os_free(ctx->vendor_attrs);
  os_free(ctx);
}

void rc_enable_debug(rc_ctx ctx) {
  hostapd_logger_register_cb(logger_cb);
  ctx->conf.msg_dumps = 1;
  wpa_debug_level = MSG_DEBUG;
}

int rc_add_attribute(rc_ctx ctx, u32 vendor, u8 subtype) {
  int count = ctx->vendor_attrs_count;

  ctx->vendor_attrs =
      os_realloc(ctx->vendor_attrs, (count + 1) * sizeof(struct vendor_attr));

  if (!ctx->vendor_attrs) {
    return -1;
  }
  ctx->vendor_attrs_count++;

  struct vendor_attr* attr = ctx->vendor_attrs + count;

  os_memset(attr, 0, sizeof(*attr));

  attr->vendor = vendor;
  attr->subtype = subtype;

  return 0;
}

int rc_add_server(rc_ctx ctx,
                  const char* shared_secret,
                  const u8* ip,
                  int ipv6,
                  u16 port,
                  u16 timeout) {
  int count = ctx->server_count;

  ctx->servers =
      os_realloc(ctx->servers, (count + 1) * sizeof(struct server_state));

  if (!ctx->servers) {
    return -1;
  }

  ctx->server_count++;

  struct server_state* state = ctx->servers + count;
  state->timeout = timeout;
  struct hostapd_radius_server* srv = &state->radius_server;

  os_memset(srv, 0, sizeof(*srv));

  {
    size_t len = strlen(shared_secret);
    srv->shared_secret = os_malloc(len);

    if (!srv->shared_secret) {
      return -1;
    }

    os_memcpy(srv->shared_secret, shared_secret, len);
    srv->shared_secret_len = len;
  }

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

static void server_timeout(void* eloop_ctx, void* timeout_ctx) {
  rc_ctx ctx = eloop_ctx;

  ctx->did_timeout = 1;
  eloop_terminate();
}

static RadiusRxResult receive_auth(struct radius_msg* msg,
                                   struct radius_msg* req,
                                   const u8* shared_secret,
                                   size_t shared_secret_len,
                                   void* data) {
  rc_ctx ctx = data;

  eloop_cancel_timeout(server_timeout, ctx, NULL);

  struct radius_hdr* hdr = radius_msg_get_hdr(msg);

  switch (hdr->code) {
    case RADIUS_CODE_ACCESS_ACCEPT:
      ctx->result_code = RC_ACCEPT;
      break;
    case RADIUS_CODE_ACCESS_REJECT:
      ctx->result_code = RC_REJECT;
      break;
    default:
      ctx->result_code = RC_ERROR;
      break;
  }
  wpa_printf(MSG_DEBUG, "Received RADIUS Authentication message; code=%d",
             hdr->code);

  eloop_terminate();

  if (ctx->result_code == RC_ACCEPT) {
    for (int i = 0; i < ctx->vendor_attrs_count; i++) {
      struct vendor_attr* attr = ctx->vendor_attrs + i;
      os_free(attr->data);
      attr->data = radius_msg_get_vendor_attr(msg, attr->vendor, attr->subtype,
                                              &attr->len);

      wpa_printf(MSG_DEBUG, "Copied RADIUS attribute; vendor=%d subtype=%d",
                 attr->vendor, attr->subtype);
    }
  }

  return RADIUS_RX_PROCESSED;
}

static void send_auth(void* eloop_ctx, void* timeout_ctx) {
  rc_ctx ctx = eloop_ctx;
  struct radius_msg* msg;

  wpa_printf(MSG_DEBUG, "Sending a RADIUS authentication message");

  ctx->radius_identifier = radius_client_get_id(ctx->radius);
  msg = radius_msg_new(RADIUS_CODE_ACCESS_REQUEST, ctx->radius_identifier);
  if (msg == NULL) {
    wpa_printf(MSG_ERROR, "Could not create net RADIUS packet");
    return;
  }

  radius_msg_make_authenticator(msg);

  if (!radius_msg_add_attr(msg, RADIUS_ATTR_USER_NAME, (u8*)ctx->username,
                           strlen(ctx->username))) {
    wpa_printf(MSG_ERROR, "Could not add User-Name");
    radius_msg_free(msg);
    return;
  }

  if (!radius_msg_add_attr_user_password(
          msg, (u8*)ctx->password, strlen(ctx->password),
          ctx->conf.auth_server->shared_secret,
          ctx->conf.auth_server->shared_secret_len)) {
    wpa_printf(MSG_ERROR, "Could not add User-Password");
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

static int try_auth(rc_ctx ctx) {
  ctx->did_timeout = 0;

  struct server_state* srv = ctx->servers + ctx->current_server;

  ctx->conf.auth_server = &srv->radius_server;
  ctx->conf.auth_servers = &srv->radius_server;
  ctx->conf.num_auth_servers = 1;

  ctx->radius = radius_client_init(&ctx, &ctx->conf);
  if (!ctx->radius) {
    return RC_ERROR;
  }
  int res = radius_client_register(ctx->radius, RADIUS_AUTH, receive_auth, ctx);

  if (res != 0) {
    radius_client_deinit(ctx->radius);
    return RC_ERROR;
  }

  eloop_register_timeout(0, 0, send_auth, ctx, NULL);
  eloop_register_timeout(srv->timeout, 0, server_timeout, ctx, NULL);

  eloop_run();

  radius_client_deinit(ctx->radius);

  if (ctx->did_timeout) {
    return RC_SERV_TIMEOUT;
  }

  return ctx->result_code;
}

int rc_authenticate(rc_ctx ctx, const char* username, const char* password) {
  if (!ctx->server_count) {
    return RC_NO_SERV;
  }

  ctx->username = username;
  ctx->password = password;

  for (int i = 0; i < ctx->server_count; i++) {
    ctx->current_server = i;
    int res = try_auth(ctx);
    if (res != RC_SERV_TIMEOUT) {
      return res;
    }
  }

  return RC_SERV_TIMEOUT;
}

struct vendor_attr* rc_get_attributes(rc_ctx ctx, int* count) {
  *count = ctx->vendor_attrs_count;
  return ctx->vendor_attrs;
}
