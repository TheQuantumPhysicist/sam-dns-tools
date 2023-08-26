#!/bin/bash

echo "Running auth hook cleanup for domain: $CERTBOT_DOMAIN with validation string $CERTBOT_VALIDATION"

# the env var PROXY_FOR_CERTBOT_DNS_HOOK is the proxy to be used
if [ -z "$PROXY_FOR_CERTBOT_DNS_HOOK" ]; then
    echo "No proxy provided."
    cargo run -- --operation cleanup --domain-name $CERTBOT_DOMAIN --validation-string "$CERTBOT_VALIDATION"
else
    echo "Proxy provided: $PROXY_FOR_CERTBOT_DNS_HOOK"
    cargo run -- --operation cleanup --domain-name $CERTBOT_DOMAIN --validation-string "$CERTBOT_VALIDATION" --proxy $PROXY_FOR_CERTBOT_DNS_HOOK
fi

echo "Done running auth hook cleanup for domain: $CERTBOT_DOMAIN with validation string $CERTBOT_VALIDATION"

exit 0
