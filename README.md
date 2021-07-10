# oidc-tester-rs

Simple OIDC token getting test in Rust.

# cURL example

curl -L -X POST "${PROVIDER_URL}/auth/realms/${REALM}/protocol/openid-connect/token" \
-H "Content-Type: application/x-www-form-urlencoded" \
--data-urlencode "client_id=$CLIENT_ID" \
--data-urlencode "grant_type=$GRANT_TYPE" \
--data-urlencode "client_secret=$CLIENT_SECRET" \
--data-urlencode "scope=$SCOPE" \
--data-urlencode "username=$USERNAME" \
--data-urlencode "password=$PASSWORD"