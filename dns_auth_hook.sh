#!/bin/bash

echo "Running auth hook for domain: $CERTBOT_DOMAIN with validation string $CERTBOT_VALIDATION"

# the env var PROXY_FOR_CERTBOT_DNS_HOOK is the proxy to be used
if [ -z "$PROXY_FOR_CERTBOT_DNS_HOOK" ]; then
    echo "No proxy provided."
    cargo run -- certbot --operation=set-record --domain-name=$CERTBOT_DOMAIN --validation-string=$CERTBOT_VALIDATION
else
    echo "Proxy provided: $PROXY_FOR_CERTBOT_DNS_HOOK"
    cargo run -- certbot --operation=set-record --domain-name=$CERTBOT_DOMAIN --validation-string=$CERTBOT_VALIDATION --proxy=$PROXY_FOR_CERTBOT_DNS_HOOK
fi

echo "Done running auth hook for domain: $CERTBOT_DOMAIN with validation string $CERTBOT_VALIDATION"

exit 0
