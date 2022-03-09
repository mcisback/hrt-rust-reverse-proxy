# Simple And Minimal Rust Reverse Proxy

## How to test it
Add this entries to */etc/hosts*:
```
test1.local 127.0.0.1
test2.local 127.0.0.1
test3.local 127.0.0.1
```

Open 3 terminals:

Terminal One:
```
cd hrt-reverse-proxy
cargo run
....
LISTEN on 127.0.0.1:13900
```

Terminal Two:
```
# test1.local
cd hrt-reverse-proxy/html
php -S localhost:13901
```

Terminal Three:
```
curl -L -X GET http://test1.local:13900
```

And you will see that the request will be redirected according to your host.

## How To Configure:
Go to *config/config.json* and change your configuration to your needs.
You can *redirect* request by *host header* or by *path*.
There are no other options for now.
You can change the bind address and port to your needs.
If *port* is *80*, it must be run as *root*.
For example, if you want to redirect *mysite.com* to *localhost:8080*,
you will write this in *config/config.json*:
```
{
    "bind_to_host": "127.0.0.1",
    "bind_to_port": "80",
    "routes": [
        {
            "match": "mysite.com",
            "type": {
                "what": "header"
            },
            "forward_to": "http://localhost:8080"
        }
    ]
}
```

## Build Release
```
cargo build --release
```