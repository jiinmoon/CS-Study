# Intro to System Design Topics

## Scalability Introduction

Lecture from CS75 (Scalability Harvard Web Devlopment David Malan).


Basically, _scalability_ is trying to understand and to solve problems arising as
we try to service many users for our application.


Suppose we have a web site that is going viral and really popular. This web
site is comprised of static web pages as well as some dynamic content (PHP) and
some database interactions. How can we "scale" so that our web site can handle
more users?

1. Vertical Scaling

    - Simply increase the computation power (throw more resources at it).
    - i.e. allocate more resources such as processing power (CPU), RAMs, disk space and so on.
    - But it is not often a complete solution to scalability problem as we are
      limited by the hardware. 

2. Horizontal Scaling

    - Instead of getting a few good machines, get a bunch of slower machines.
    - Problem can arise as if we have multiple servers, how do we distribute
      inbound HTTP requests? This is solved with "Load Balancing".

### Load Balancer

Typical set up would be the load balancer would sit in the middle of the
traffics between the clients and servers. All the traffics must go through the
load balancer which will determine to which server they will be directed to.

The load balancer would have a public IP address (and DNS name associated) so
that clients will access the resources via load balancer. Now, our servers at
the backend does not have to have exposed public IPs - but instead have private
IPs.

The challenge is exactly how would load balancer determine where to route the
traffics. Load balancer would perform constant health checks on backend servers
to see how busy they are based on various stats (i.e. CPU usage).

Or alternatively, we do not even need a separate load balancer but set up our
DNS server such that on every request, we give different IPs back to the users
for our backend servers (round-robin). For example, for first request, client
will get a server 1 IP - and so on. This approach can have a problem where one
server will get overwhelmed and we cannot do anything about it. Also, most of
the time, DNS responses will be cache'd - hence, it can contribute to the
disproportionate load across the backend servers.

Also, we have to be mindful with how "sessions" work with load balancers. Each
session tends to be bound to the given machine. One user may was interacting
with the server 1 then gets routed to server 2 which will not have user's
session saved (i.e. log-in again or lose saved information such as checkout).

One way to approach above problem would be to factor out the session so that we
may have a separate file server outside of the servers so that we store the
session data there and have backend servers lookup and share states. The
weakness is obviously that if this server goes down, all our session is lost.
We may use RAID - RAID0 stripes data to multiple disks to speed up servicing data
IOs; RAID1 mirrors the data to others so that we are effectively creating
clone. This only tackles the hard-disk but at least provides some form of
redundancy. One obvious solution would be to have backup servers that will be
in sync with the original so that in case of emergency, it will take over
seamlessly.

Another way to achieve Sticky Session is to have cookie and assign a server to
a client; and have each request tagged with header so that load balancer can
read and know where to route the request to.

### Examples of Load Balancers

- Software Based:
    - AWS Elastic Load Balancer
    - HAProxy
    - Linux Virtual Server
    - ...

- Hardware Based:
    - Barracuda
    - Cisco
    - ...

## Idea of Caching

- Static webpages (.html files)
- MySQL query cache
- memcached or REDIS (RAM storage)

Using cache and its implications - when to update our cache'd data? Depending
on the data.

## Replication

Master data server and its data are replicated onto the several (one or more)
slave data servers simultaneously. 

This solves the problem of the master dying; we can promote one of the slave as
our back up while master can be fixed - redundancy.

Also, for global services, we can reduce the query times as we can redirect
users queries spread out many servers (and also local - physically closest).

Downside of this approach is that we still have a single point of failure where
if the master server dies, we cannot write anymore while backup takes over.


