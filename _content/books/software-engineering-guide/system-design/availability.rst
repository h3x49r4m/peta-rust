---
title: "Availability"
date: 2026-02-07T00:00:00
tags: ["system-design", "availability", "reliability"]
description: "System availability and reliability strategies"
---

Availability
============

Availability is the proportion of time a system is in a functioning condition. High availability is critical for modern applications.

Availability Metrics
--------------------

**Uptime Percentage**

Calculate availability as a percentage of time the system is operational.

.. code-block:: python

   def calculate_availability(uptime_minutes, total_minutes):
       """
       Calculate availability percentage
       """
       return (uptime_minutes / total_minutes) * 100

   # Examples
   # 99.9% availability = 8.76 hours downtime per year
   # 99.99% availability = 52.56 minutes downtime per year
   # 99.999% availability = 5.26 minutes downtime per year

**Mean Time Between Failures (MTBF)**

Average time between system failures.

.. code-block:: python

   def calculate_mtbf(total_uptime, number_of_failures):
       """
       Calculate Mean Time Between Failures
       """
       return total_uptime / number_of_failures

**Mean Time To Repair (MTTR)**

Average time to restore service after a failure.

.. code-block:: python

   def calculate_mttr(total_downtime, number_of_failures):
       """
       Calculate Mean Time To Repair
       """
       return total_downtime / number_of_failures

Redundancy Strategies
---------------------

**Active-Active Redundancy**

Multiple instances handle traffic simultaneously.

.. code-block:: python

   # Load balancer distributes traffic to all active instances
   class ActiveActiveLoadBalancer:
       def __init__(self, instances):
           self.instances = instances

       def route_request(self, request):
           # Distribute load across all instances
           instance = self.select_instance()
           return instance.handle(request)

       def select_instance(self):
           # Round-robin or least-loaded selection
           return self.instances[self.current_index % len(self.instances)]

**Active-Passive Redundancy**

One instance handles traffic while others stand by.

.. code-block:: python

   # Failover configuration
   class FailoverManager:
       def __init__(self, primary, secondary):
           self.primary = primary
           self.secondary = secondary
           self.current = primary

       def handle_request(self, request):
           try:
               return self.current.handle(request)
           except Exception as e:
               # Failover to secondary
               self.current = self.secondary
               return self.secondary.handle(request)

**Geographic Redundancy**

Distribute instances across multiple geographic regions.

.. code-block:: python

   # Geographic load balancing
   class GeographicLoadBalancer:
       def __init__(self, regions):
           self.regions = regions

       def route_request(self, request, user_location):
           # Route to nearest region
           nearest_region = self.find_nearest_region(user_location)
           return nearest_region.handle(request)

       def find_nearest_region(self, location):
           # Calculate geographic distance
           pass

Failover Mechanisms
-------------------

**Health Checks**

Regularly check service health.

.. code-block:: python

   class HealthChecker:
       def __init__(self, service, check_interval=30):
           self.service = service
           self.check_interval = check_interval

       def check_health(self):
           try:
               response = self.service.ping()
               return response.status_code == 200
           except Exception:
               return False

       def monitor(self):
           while True:
               if not self.check_health():
                   self.handle_failure()
               time.sleep(self.check_interval)

**Automatic Failover**

Automatically switch to backup when primary fails.

.. code-block:: python

   class AutomaticFailover:
       def __init__(self, primary, backup):
           self.primary = primary
           self.backup = backup
           self.healthy = True

       def execute(self, operation):
           try:
               if not self.healthy:
                   return self.backup.execute(operation)
               return self.primary.execute(operation)
           except Exception as e:
               self.healthy = False
               return self.backup.execute(operation)

**Graceful Degradation**

Reduce functionality rather than failing completely.

.. code-block:: python

   class ServiceWithGracefulDegradation:
       def __init__(self, primary_service, fallback_service):
           self.primary_service = primary_service
           self.fallback_service = fallback_service

       def get_user_data(self, user_id):
           try:
               # Try primary service first
               return self.primary_service.get_user(user_id)
           except Exception:
               # Fall back to simplified service
               return self.fallback_service.get_basic_user_data(user_id)

CAP Theorem
-----------

Understanding the trade-offs between Consistency, Availability, and Partition Tolerance.

**Consistency (C)**

All nodes see the same data at the same time.

.. code-block:: python

   # Strong consistency
   class ConsistentCache:
       def __init__(self):
           self.data = {}
           self.lock = threading.Lock()

       def set(self, key, value):
           with self.lock:
               self.data[key] = value
               # Propagate to all nodes immediately
               self.propagate_to_all_nodes(key, value)

       def get(self, key):
           with self.lock:
               return self.data.get(key)

**Availability (A)**

Every request receives a response (success or failure).

.. code-block:: python

   # High availability
   class AvailableCache:
       def get(self, key):
           try:
               return self.primary_node.get(key)
           except Exception:
               # Return stale data if primary is unavailable
               return self.stale_cache.get(key, None)

**Partition Tolerance (P)**

System continues operating despite network failures.

.. code-block:: python

   # Partition tolerant
   class PartitionTolerantCache:
       def __init__(self):
           self.nodes = []

       def get(self, key):
           # Try all nodes until one responds
           for node in self.nodes:
               try:
                   return node.get(key)
               except Exception:
                   continue
           return None

**CAP Trade-offs**

Choose two of the three properties:

- **CA**: Consistency + Availability (no partition tolerance)
- **CP**: Consistency + Partition Tolerance (may not be available)
- **AP**: Availability + Partition Tolerance (may be inconsistent)

Disaster Recovery
-----------------

**Backup Strategies**

Regular backups and point-in-time recovery.

.. code-block:: python

   class BackupManager:
       def __init__(self, database):
           self.database = database
           self.backup_schedule = 'daily'

       def create_backup(self):
           # Create full backup
           backup_file = f"backup_{datetime.now().strftime('%Y%m%d')}.sql"
           self.database.dump(backup_file)

           # Upload to cloud storage
           cloud_storage.upload(backup_file)

       def restore_backup(self, backup_date):
           backup_file = f"backup_{backup_date}.sql"
           cloud_storage.download(backup_file)
           self.database.restore(backup_file)

**Multi-Region Deployment**

Deploy across multiple regions for disaster recovery.

.. code-block:: python

   # Multi-region configuration
   class MultiRegionManager:
       def __init__(self, regions):
           self.regions = regions
           self.primary_region = regions[0]

       def write_data(self, data):
           # Write to primary region
           self.primary_region.write(data)

           # Replicate to backup regions asynchronously
           for region in self.regions[1:]:
               thread = Thread(target=region.write, args=(data,))
               thread.start()

       def read_data(self, key):
           # Try primary region first
           try:
               return self.primary_region.read(key)
           except Exception:
               # Fall back to backup regions
               for region in self.regions[1:]:
                   try:
                       return region.read(key)
                   except Exception:
                       continue

**Recovery Time Objective (RTO)**

Maximum acceptable time to restore service.

.. code-block:: python

   def check_rto_compliance(actual_recovery_time, rto_threshold):
       """
       Check if recovery meets RTO requirement
       """
       return actual_recovery_time <= rto_threshold

   # Example: RTO of 1 hour
   rto_threshold = 3600  # seconds
   actual_recovery_time = 1800  # seconds
   is_compliant = check_rto_compliance(actual_recovery_time, rto_threshold)

**Recovery Point Objective (RPO)**

Maximum acceptable data loss.

.. code-block:: python

   def check_rpo_compliance(last_backup_time, rpo_threshold):
       """
       Check if data loss meets RPO requirement
       """
       time_since_backup = datetime.now() - last_backup_time
       return time_since_backup.total_seconds() <= rpo_threshold

   # Example: RPO of 15 minutes
   rpo_threshold = 900  # seconds
   time_since_backup = datetime.now() - last_backup_time
   is_compliant = check_rpo_compliance(time_since_backup, rpo_threshold)

Circuit Breakers
----------------

Prevent cascading failures by stopping calls to failing services.

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

Monitoring and Alerting
-----------------------

**Uptime Monitoring**

Continuous monitoring of service availability.

.. code-block:: python

   class UptimeMonitor:
       def __init__(self, service, check_interval=60):
           self.service = service
           self.check_interval = check_interval

       def monitor(self):
           while True:
               try:
                   response = self.service.ping()
                   if response.status_code != 200:
                       self.alert_admin(f"Service is down: {response.status_code}")
               except Exception as e:
                   self.alert_admin(f"Service is unreachable: {str(e)}")

               time.sleep(self.check_interval)

       def alert_admin(self, message):
           # Send alert via email, SMS, or pager
           pass

**SLA Monitoring**

Track compliance with Service Level Agreements.

.. code-block:: python

   class SLAMonitor:
       def __init__(self, sla_threshold=99.9):
           self.sla_threshold = sla_threshold
           self.uptime_minutes = 0
           self.total_minutes = 0

       def record_uptime(self, uptime):
           self.uptime_minutes += uptime
           self.total_minutes += uptime

       def record_downtime(self, downtime):
           self.total_minutes += downtime

       def check_sla_compliance(self):
           availability = (self.uptime_minutes / self.total_minutes) * 100
           return availability >= self.sla_threshold

Best Practices
--------------

**Design for Failure**

Assume components will fail and plan accordingly.

.. code-block:: python

   def handle_request_with_retry(request, max_retries=3):
       for attempt in range(max_retries):
           try:
               return service.process(request)
           except Exception as e:
               if attempt == max_retries - 1:
                   raise
               time.sleep(2 ** attempt)  # Exponential backoff

**Implement Health Checks**

Regular health checks to detect failures early.

**Use Circuit Breakers**

Prevent cascading failures.

**Test Failover**

Regularly test failover procedures.

**Document Recovery Procedures**

Maintain clear documentation for disaster recovery.

Common Pitfalls
---------------

**Single Point of Failure**

No single component should be critical.

**Insufficient Testing**

Regularly test disaster recovery procedures.

**Ignoring Monitoring**

Without monitoring, you won't know about failures.

**Poor Communication**

Clear communication during outages is critical.

**Complex Dependencies**

Minimize dependencies to reduce failure surface.