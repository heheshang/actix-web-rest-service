head -c16 /dev/urandom > secret.key
cat /dev/urandom | tr -dc 0-1 | head -c 16
