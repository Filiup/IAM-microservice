#!/bin/sh

# Script is using jwt.sh project to create JWT
# URL: https://github.com/gasconleon/jwt.sh

JWT_SH_URL="https://raw.githubusercontent.com/gasconleon/jwt.sh/refs/heads/master/jwt.sh"

PRIVATE_KEY_FILE="jwtRS256.key"
ALGORITHM="RS256"
CLIENT_ALIAS_ID=1
GROUP_ID=1
ISSUER=test

CURRENT_TIMESTAMP=$(date +%s)
JWT_EXP_TIMESTAMP=$((CURRENT_TIMESTAMP + 3600))

curl -sSL $JWT_SH_URL | bash -s -- \
-a $ALGORITHM \
-P $PRIVATE_KEY_FILE \
-H "{\"alg\": \"$ALGORITHM\", \"typ\": \"JWT\",\"iss\": \"$ISSUER\"}" \
-p "{\"caid\": $CLIENT_ALIAS_ID, \"gid\": $GROUP_ID, \"iss\": \"$ISSUER\", \"exp\": $JWT_EXP_TIMESTAMP, \"sub\": \"\"}"