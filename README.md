# install docker docker-compose and use traefik for maximum convenience

# execute

# copy your public keys

create a file in the public-keys directory with the name of your key id (for example: paul@example.com) and insert your public key into the file.

# get the public key:

## thunderbird

menu > extras > manage OpenPGP keys > right click the key > copy public key

## cmd line

```sh
gpg -a --export paul@example.com > public-keys/paul@example.com
```

or

```sh
gpg -a --export paul@example.com > paul@example.com
```

# start the server

```sh
docker-compose up -d --build
```

# check here

https://metacode.biz/openpgp/web-key-directory

thanks @www.kuketz-blog.de
https://www.kuketz-blog.de/gnupg-web-key-directory-wkd-einrichten/
