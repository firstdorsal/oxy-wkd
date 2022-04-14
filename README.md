# oxy-wkd

A ultra small(**316KB**) container running a ultra fast rust-hyper webserver to easily serve your armored public pgp keys from plain text files via wkd (Web Key Directory)

## `firstdorsal/oxy-wkd`

[dockerhub](https://hub.docker.com/r/firstdorsal/oxy-wkd)

# Getting started

## install docker, docker-compose, and use traefik for maximum convenience

## get the public key

### thunderbird

menu > extras > manage OpenPGP keys > right click the key > copy public key

### command line

```sh
gpg -a --export paul@example.com > public-keys/paul@example.com
```

## copy your public keys in a folder

create a file in a new public keys directory with the name of your key id (for example: paul@example.com) and insert your armored public key into the file.

**./public-keys/paul@example.com**

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

mQINBGEEBj4BEADU4TZNYDN2VkxUUrZtNtMF1UVBL9ZuEfGdJ/z6IHWmQGnyYcEt
...
8Z/5F32mtK+q0XrpC7DkHcqfLYqtDp3r2JJtKgjDs/jHy/9joOxMqUg0QEkHt4Q=
=6Wrc
-----END PGP PUBLIC KEY BLOCK-----

```

## create a docker compose file

`rp` is a network that traefik has access to. Your situation might look different <br/>
Of course you can use any reverse proxy for this.

**./docker-compose.yml**

```yaml
version: "3.7"
services:
    wkd:
        container_name: wkd
        restart: always
        image: firstdorsal/oxy-wkd
        volumes:
            - ./public-keys/:/public_pgp_keys
        labels:
            - traefik.enable=true
            - traefik.http.routers.wkd.tls.certresolver=default
            - traefik.http.routers.wkd.tls.domains[0].main=example.com
            - traefik.http.routers.wkd.tls.domains[0].sans=*.example.com
            - traefik.http.routers.wkd.rule=PathPrefix(`/.well-known/openpgpkey`)
            - traefik.http.routers.wkd.tls=true
            - traefik.http.routers.wkd.priority=3
            - traefik.http.routers.wkd.entrypoints=websecure
            - traefik.http.services.wkd.loadbalancer.server.port=80
            - traefik.docker.network=rp
        networks:
            - rp
networks:
    rp:
        name: rp
```

## start it

```sh
docker-compose up -d
```

## check here if it works

https://metacode.biz/openpgp/web-key-directory
