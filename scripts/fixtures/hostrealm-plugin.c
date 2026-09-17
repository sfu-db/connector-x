#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <krb5/hostrealm_plugin.h>

static krb5_error_code
default_realm(krb5_context context, krb5_hostrealm_moddata data, char ***out)
{
    char **realms = calloc(2, sizeof(*realms));
    if (realms == NULL)
        return ENOMEM;
    realms[0] = strdup(TEST_REALM);
    if (realms[0] == NULL) {
        free(realms);
        return ENOMEM;
    }
    *out = realms;
    return 0;
}

static void
free_list(krb5_context context, krb5_hostrealm_moddata data, char **realms)
{
    free(realms[0]);
    free(realms);
}

krb5_error_code
hostrealm_connectorx_test_initvt(krb5_context context, int major, int minor,
                               krb5_plugin_vtable table)
{
    krb5_hostrealm_vtable vt = (krb5_hostrealm_vtable)table;
    if (major != 1)
        return KRB5_PLUGIN_VER_NOTSUPP;
    memset(vt, 0, sizeof(*vt));
    vt->name = "connectorx_test";
    vt->default_realm = default_realm;
    vt->free_list = free_list;
    return 0;
}
