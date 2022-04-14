# oxy-wkd

A ultra small(**316KB**) container running a ultra fast rust-hyper webserver to easily serve your armored public pgp keys from plain text files via wkd (Web Key Directory)

You will need some reverse proxy for this.

## `firstdorsal/oxy-wkd`

[dockerhub](https://hub.docker.com/r/firstdorsal/oxy-wkd)

# Getting started

## Install docker and optionally docker-compose and traefik for maximum convenience

## Get the public key

### Thunderbird

menu > extras > manage OpenPGP keys > right click the key > copy public key

### Command line

```sh
gpg -a --export paul@example.com > public-keys/paul@example.com
```

## Copy your public keys in a folder

Create a file in a new public keys directory with the name of your key id (for example: paul@example.com) and insert your armored public key into the file.

**./public-keys/paul@example.com**

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

mQINBGEEBj4BEADU4TZNYDN2VkxUUrZtNtMF1UVBL9ZuEfGdJ/z6IHWmQGnyYcEt
...
8Z/5F32mtK+q0XrpC7DkHcqfLYqtDp3r2JJtKgjDs/jHy/9joOxMqUg0QEkHt4Q=
=6Wrc
-----END PGP PUBLIC KEY BLOCK-----

```

## Create a docker compose file

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

## Start it

```sh
docker-compose up -d
```

## Check here if it works

https://metacode.biz/openpgp/web-key-directory

## Helpful resources

https://www.kuketz-blog.de/gnupg-web-key-directory-wkd-einrichten/

# License MIT

Copyright 2022 Paul Colin Hennig

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
