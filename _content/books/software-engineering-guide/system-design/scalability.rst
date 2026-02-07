---
title: "Scalability"
date: 2026-02-07T00:00:00
tags: ["system-design", "scalability", "performance"]
description: "Scalability strategies and techniques"
---

Scalability
===========

Scalability is the ability of a system to handle growing amounts of work by adding resources to the system.

Types of Scaling
----------------

**Vertical Scaling (Scale Up)**

Increasing the capacity of a single resource.

.. code-block:: python

   # Vertical scaling: upgrade server resources
   # - More CPU cores
   # - More RAM
   # - Faster storage
   # - Better network

   # No code changes required, but has limits

**Horizontal Scaling (Scale Out)**

Adding more resources to handle increased load.

.. code-block:: python

   # Horizontal scaling: add more servers
   # - Load balancer distributes traffic
   # - Multiple application servers
   # - Database clustering
   # - Distributed caching

   # Requires code changes for statelessness

Scaling Strategies
------------------

**Load Balancing**

Distribute incoming traffic across multiple servers.

.. code-block:: python

   # Load balancer configuration
   class LoadBalancer:
       def __init__(self, servers):
           self.servers = servers
           self.current_index = 0

       def round_robin(self, request):
           server = self.servers[self.current_index]
           self.current_index = (self.current_index + 1) % len(self.servers)
           return server.handle(request)

       def least_connections(self, request):
           server = min(self.servers, key=lambda s: s.active_connections)
           return server.handle(request)

**Caching**

Store frequently accessed data in fast storage.

.. code-block:: python

   # Multi-level caching strategy
   class CacheManager:
       def __init__(self):
           self.l1_cache = {}  # In-memory cache
           self.l2_cache = RedisClient()  # Distributed cache
           self.database = Database()

       def get(self, key):
           # Check L1 cache
           if key in self.l1_cache:
               return self.l1_cache[key]

           # Check L2 cache
           value = self.l2_cache.get(key)
           if value:
               self.l1_cache[key] = value
               return value

           # Query database
           value = self.database.get(key)
           if value:
               self.l1_cache[key] = value
               self.l2_cache.set(key, value)
           return value

**Database Scaling**

Scale database reads and writes separately.

.. code-block:: python

   # Read replicas for read scalability
   class DatabaseManager:
       def __init__(self, primary, replicas):
           self.primary = primary
           self.replicas = replicas

       def read(self, query):
           # Distribute reads across replicas
           replica = self.select_replica()
           return replica.execute(query)

       def write(self, query):
           # All writes go to primary
           return self.primary.execute(query)

       def select_replica(self):
           # Round-robin or least-loaded selection
           return self.replicas[random.randint(0, len(self.replicas) - 1)]

**Asynchronous Processing**

Offload time-consuming tasks to background workers.

.. code-block:: python

   # Task queue for asynchronous processing
   import celery

   @celery.task
   def send_welcome_email(user_id):
       user = get_user(user_id)
       email_service.send(user.email, "Welcome!")

   # In request handler
   def register_user(user_data):
       user = create_user(user_data)
       send_welcome_email.delay(user.id)  # Asynchronous
       return user

Microservices for Scalability
------------------------------

**Service Decomposition**

Break monolithic application into smaller services.

.. code-block:: python

   # Monolithic approach
   class MonolithicApp:
       def handle_request(self, request):
           if request.type == 'user':
               return self.handle_user_request(request)
           elif request.type == 'order':
               return self.handle_order_request(request)
           # All functionality in one service

   # Microservices approach
   class UserService:
       def handle_request(self, request):
           # Only handles user-related requests
           pass

   class OrderService:
       def handle_request(self, request):
           # Only handles order-related requests
           pass

**Independent Scaling**

Scale services independently based on demand.

.. code-block:: python

   # Kubernetes deployment configuration
   # User service: 3 replicas
   apiVersion: apps/v1
   kind: Deployment
   metadata:
     name: user-service
   spec:
     replicas: 3  # Scale based on user load

   ---
   # Order service: 10 replicas
   apiVersion: apps/v1
   kind: Deployment
   metadata:
     name: order-service
   spec:
     replicas: 10  # Scale based on order load

Performance Optimization
------------------------

**Database Optimization**

Optimize database queries and indexes.

.. code-block:: sql

   -- Add indexes for frequently queried columns
   CREATE INDEX idx_user_email ON users(email);

   -- Optimize complex queries
   SELECT u.name, COUNT(o.id) as order_count
   FROM users u
   LEFT JOIN orders o ON u.id = o.user_id
   WHERE u.created_at > '2026-01-01'
   GROUP BY u.id, u.name;

**Connection Pooling**

Reuse database connections to reduce overhead.

.. code-block:: python

   from sqlalchemy import create_engine
   from sqlalchemy.pool import QueuePool

   # Create connection pool
   engine = create_engine(
       'postgresql://user:password@localhost/db',
       poolclass=QueuePool,
       pool_size=10,
       max_overflow=20,
       pool_timeout=30
   )

**Content Delivery Network (CDN)**

Distribute static content globally.

.. code-block:: python

   # CDN configuration
   CDN_URL = "https://cdn.example.com"

   def get_asset_url(asset_path):
       return f"{CDN_URL}/{asset_path}"

   # Use CDN for static assets
   <img src="{{ get_asset_url('images/logo.png') }}">

**Compression**

Compress responses to reduce bandwidth.

.. code-block:: python

   from flask import Flask, jsonify, request
   import gzip

   app = Flask(__name__)

   @app.before_request
   def compress_response():
       if request.accept_encodings and 'gzip' in request.accept_encodings:
           # Enable compression
           pass

   @app.route('/api/data')
   def get_data():
       response = jsonify(large_data)
       response.headers['Content-Encoding'] = 'gzip'
       return response

Auto-scaling
------------

**Horizontal Pod Autoscaler (HPA)**

Automatically scale based on CPU/memory usage.

.. code-block:: yaml

   apiVersion: autoscaling/v2
   kind: HorizontalPodAutoscaler
   metadata:
     name: app-hpa
   spec:
     scaleTargetRef:
       apiVersion: apps/v1
       kind: Deployment
       name: app
     minReplicas: 2
     maxReplicas: 10
     metrics:
     - type: Resource
       resource:
         name: cpu
         target:
           type: Utilization
           averageUtilization: 70

**Queue-based Scaling**

Scale based on queue length.

.. code-block:: python

   class QueueScaler:
       def __init__(self, queue, min_workers, max_workers):
           self.queue = queue
           self.min_workers = min_workers
           self.max_workers = max_workers
           self.workers = []

       def monitor_and_scale(self):
           queue_length = self.queue.size()
           current_workers = len(self.workers)

           if queue_length > current_workers * 10 and current_workers < self.max_workers:
               # Add workers
               self.add_worker()

           elif queue_length < current_workers * 2 and current_workers > self.min_workers:
               # Remove workers
               self.remove_worker()

Monitoring and Metrics
----------------------

**Performance Metrics**

Track key performance indicators.

.. code-block:: python

   from prometheus_client import Counter, Histogram, Gauge

   # Define metrics
   REQUEST_COUNT = Counter('http_requests_total', 'Total HTTP requests')
   REQUEST_DURATION = Histogram('http_request_duration_seconds', 'HTTP request duration')
   ACTIVE_CONNECTIONS = Gauge('active_connections', 'Number of active connections')

   # Use metrics
   @REQUEST_DURATION.time()
   def handle_request(request):
       REQUEST_COUNT.inc()
       ACTIVE_CONNECTIONS.inc()
       try:
           return process_request(request)
       finally:
           ACTIVE_CONNECTIONS.dec()

**Capacity Planning**

Plan for future growth.

.. code-block:: python

   def estimate_capacity(current_users, growth_rate, months):
       """
       Estimate required capacity based on growth projections
       """
       future_users = current_users * (1 + growth_rate) ** months
       required_capacity = future_users * 100  # Assume 100 requests per user
       return required_capacity

   # Plan for 6 months of growth
   current_users = 10000
   growth_rate = 0.1  # 10% monthly growth
   capacity_needed = estimate_capacity(current_users, growth_rate, 6)

Best Practices
--------------

**Design for Statelessness**

Make services stateless for easy scaling.

.. code-block:: python

   # Stateless request handler
   def handle_request(request):
       # All state is in the request or database
       user = get_user(request.user_id)
       return process(user)

   # Avoid: storing state in memory
   # state = {}  # Don't do this

**Use Asynchronous Communication**

Use message queues for loose coupling.

.. code-block:: python

   # Asynchronous communication
   def process_order(order):
       queue.publish('order.processed', order)
       return {'status': 'processing'}

   # Consumer handles the order asynchronously
   def handle_order_processing(order):
       # Process order
       pass

**Implement Circuit Breakers**

Prevent cascading failures.

.. code-block:: python

   class CircuitBreaker:
       def __init__(self, failure_threshold=5, timeout=60):
           self.failure_threshold = failure_threshold
           self.timeout = timeout
           self.failures = 0
           self.last_failure_time = None
           self.state = 'closed'  # closed, open, half-open

       def call(self, func, *args, **kwargs):
           if self.state == 'open':
               if time.time() - self.last_failure_time > self.timeout:
                   self.state = 'half-open'
               else:
                   raise Exception('Circuit breaker is open')

           try:
               result = func(*args, **kwargs)
               if self.state == 'half-open':
                   self.state = 'closed'
                   self.failures = 0
               return result
           except Exception as e:
               self.failures += 1
               self.last_failure_time = time.time()
               if self.failures >= self.failure_threshold:
                   self.state = 'open'
               raise

Common Pitfalls
---------------

**Premature Optimization**

Don't optimize before measuring actual performance.

**Over-engineering**

Keep solutions simple until complexity is needed.

**Ignoring Bottlenecks**

Identify and address actual bottlenecks first.

**No Monitoring**

Scale based on data, not assumptions.

**Stateful Services**

Stateful services are hard to scale horizontally.