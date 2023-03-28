# Test Results

## Test Machine Info

CPU: Intel(R) Core(TM) i7-11390H CPU @ 4.80GHz  
Memory: 16GB  
OS: Arch Linux Kernel 5.19.3

## Test Info

WRK was used as an HTTP benchmark tool. Each server was tested 5 times and the requests per second are averaged. The command used is `wrk -t6 -c512 -d30s http://localhost:3000`. The Typescript servers were compiled down to Javascript then ran with Node 18. The Rust servers were built for release then ran as binaries. For the Mongo tests I used Mongo running on my local machine. 

## Plain Text

Each server just returns a string of plain text.
Tested with 6 threads, 512 connections, for 30 seconds

Typescript with express:

- 17,170 requests per second
- 16,847 requests per second
- 16,007 requests per second
- 16,338 requests per second
- 14,874 requests per second  
  Average of 16,247 requests per second

Express Javascript:

- 16,969 requests per second
- 15,995 requests per second
- 15,329 requests per second
- 15,062 requests per second
- 13,453 requests per second
  Average of 15,361 requests per second

Rocket Rust:

- 199,130 requests per second
- 199,791 requests per second
- 179,688 requests per second
- 200,037 requests per second
- 183,733 requests per second
  Average of 192,475 requests per second

Actix Rust:

- 369,991 requests per second
- 392,456 requests per second
- 364,849 requests per second
- 346,312 requests per second
- 374,144 requests per second
  Average of 369,550 requests per second

## Mongo Writes

Each server writes a document to a mongo database then returns ok if successful.

This document includes the following fields:

- id: a unique identifier for the document
- created_at: the date and time the document was created
- random: a random number
- name: a set name
- description: a set description  
  Tested with 6 threads, 512 connections, for 30 seconds.

**Note:** I'm not including Javascript in this test since it's basically the same as Typescript and I'd much rather write Typescript.

Express Typescript:

- 6,739 requests per second
- 5,702 requests per second
- 5,498 requests per second
- 5,054 requests per second
- 5,628 requests per second
  Average of 5,724 requests per second

Rocket Rust:

- 36,971 requests per second
- 38,136 requests per second
- 32,934 requests per second
- 37,260 requests per second
- 34,986 requests per second
  Average of 36,057 requests per second

Actix Rust:

- 28,868 requests per second
- 29,286 requests per second
- 28,180 requests per second
- 28,968 requests per second
- 29,538 requests per second
  Average of 28,968 requests per second
  
  
## Tests with Apple Silicon M1

### Plaintext

Tide Rust:
- 115,000 req/s

Rocket Rust:
- 170,000 req/s

Actix Rust:
- 
