Iron depends on openssl, and the openssl crate doesn't build cleanly
on El Capitan unless you point it at homebrew-provided openssl
headers:

export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib