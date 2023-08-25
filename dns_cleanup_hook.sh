#!/bin/bash

echo "Running auth hook for domain: $CERTBOT_DOMAIN" with validation string $CERTBOT_VALIDATION

cargo run -- --operation cleanup --domain-name $CERTBOT_DOMAIN --validation-string $CERTBOT_VALIDATION

exit 0
