# Preflight

Automated preflight tests for a site.


The application can be run with:

    $ cargo run


# Building Notes

During dependency install in OSX it might give issues if it cannot find the openssl library. If using brew this can be fixed by setting the following environment varialbles:

    $ export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
    $ export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include

