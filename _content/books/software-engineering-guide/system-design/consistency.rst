---
title: "Consistency"
date: 2026-02-07T00:00:00
tags: ["system-design", "consistency", "data-integrity"]
description: "Data consistency models and strategies"
---

Consistency
===========

Consistency ensures that all nodes in a distributed system see the same data at the same time. It's a critical aspect of system design.

Consistency Models
------------------

**Strong Consistency**

All reads return the most recent write.

.. code-block:: python

   # Strong consistency with synchronous replication
   class StrongConsistentStore:
       def __init__(self, nodes):
           self.nodes = nodes

       def write(self, key, value):
           # Write to all nodes synchronously
           for node in self.nodes:
               node.write(key, value)
           # Only return after all nodes confirm

       def read(self, key):
           # Read from any node (all have same data)
           return self.nodes[0].read(key)

**Eventual Consistency**

All nodes eventually converge to the same state.

.. code-block:: python

   # Eventual consistency with asynchronous replication
   class EventualConsistentStore:
       def __init__(self, nodes):
           self.nodes = nodes

       def write(self, key, value):
           # Write to primary node
           self.nodes[0].write(key, value)
           # Return immediately
           # Replicate asynchronously in background

       def read(self, key):
           # Read from any node
           # May return stale data temporarily
           return random.choice(self.nodes).read(key)

**Causal Consistency**

Operations that are causally related are seen by all nodes in the same order.

.. code-block:: python

   # Causal consistency with vector clocks
   class CausalConsistentStore:
       def __init__(self):
           self.data = {}
           self.vector_clocks = {}

       def write(self, key, value, client_id):
           # Increment client's clock
           self.vector_clocks[client_id] = self.vector_clocks.get(client_id, 0) + 1

           # Store data with vector clock
           self.data[key] = {
               'value': value,
               'vector_clock': self.vector_clocks.copy()
           }

       def read(self, key):
           # Return data with vector clock
           return self.data.get(key)

       def resolve_conflicts(self, updates):
           # Resolve conflicts using vector clocks
           pass

**Read Your Writes Consistency**

A client always sees their own writes.

.. code-block:: python

   # Read-your-writes consistency with session consistency
   class ReadYourWritesStore:
       def __init__(self, nodes):
           self.nodes = nodes
           self.client_writes = {}

       def write(self, key, value, client_id):
           # Write to all nodes
           for node in self.nodes:
               node.write(key, value)

           # Track client's writes
           self.client_writes[client_id] = self.client_writes.get(client_id, {})
           self.client_writes[client_id][key] = value

       def read(self, key, client_id):
           # Check if client has written this key
           if client_id in self.client_writes and key in self.client_writes[client_id]:
               # Return client's write
               return self.client_writes[client_id][key]

           # Otherwise, read from any node
           return self.nodes[0].read(key)

Conflict Resolution
-------------------

**Last Write Wins (LWW)**

Resolve conflicts by choosing the most recent write.

.. code-block:: python

   class LastWriteWinsResolver:
       def resolve_conflict(self, updates):
           # Choose the update with latest timestamp
           return max(updates, key=lambda u: u['timestamp'])

**Version Vectors**

Use version vectors to track causality.

.. code-block:: python

   class VersionVector:
       def __init__(self):
           self.vector = {}

       def increment(self, node_id):
           self.vector[node_id] = self.vector.get(node_id, 0) + 1

       def merge(self, other):
           for node_id, counter in other.vector.items():
               self.vector[node_id] = max(self.vector.get(node_id, 0), counter)

       def is_equal(self, other):
           return self.vector == other.vector

**Operational Transformation**

Transform operations to resolve conflicts.

.. code-block:: python

   class OperationalTransformer:
       def transform_insert(self, insert_op, concurrent_op):
           # Transform insert operation based on concurrent operation
           if concurrent_op['type'] == 'insert':
               if concurrent_op['position'] <= insert_op['position']:
                   insert_op['position'] += 1
           return insert_op

       def transform_delete(self, delete_op, concurrent_op):
           # Transform delete operation based on concurrent operation
           if concurrent_op['type'] == 'insert':
               if concurrent_op['position'] < delete_op['position']:
                   delete_op['position'] += 1
           return delete_op

**Conflict-Free Replicated Data Types (CRDTs)**

Data structures that automatically resolve conflicts.

.. code-block:: python

   # G-Counter (Grow-only Counter)
   class GCounter:
       def __init__(self):
           self.counters = {}

       def increment(self, node_id, delta=1):
           self.counters[node_id] = self.counters.get(node_id, 0) + delta

       def value(self):
           return sum(self.counters.values())

       def merge(self, other):
           for node_id, counter in other.counters.items():
               self.counters[node_id] = max(self.counters.get(node_id, 0), counter)

   # LWW-Register (Last-Write-Wins Register)
   class LWWRegister:
       def __init__(self):
           self.value = None
           self.timestamp = 0

       def assign(self, value, timestamp):
           if timestamp > self.timestamp:
               self.value = value
               self.timestamp = timestamp

       def merge(self, other):
           if other.timestamp > self.timestamp:
               self.value = other.value
               self.timestamp = other.timestamp

Data Replication Strategies
---------------------------

**Synchronous Replication**

All replicas confirm writes before returning.

.. code-block:: python

   class SynchronousReplicator:
       def __init__(self, replicas):
           self.replicas = replicas

       def write(self, key, value):
           # Write to all replicas
           futures = []
           for replica in self.replicas:
               future = replica.write_async(key, value)
               futures.append(future)

           # Wait for all replicas to confirm
           for future in futures:
               future.wait()

           # Return success
           return True

**Asynchronous Replication**

Write to primary, replicate to others in background.

.. code-block:: python

   class AsynchronousReplicator:
       def __init__(self, primary, replicas):
           self.primary = primary
           self.replicas = replicas

       def write(self, key, value):
           # Write to primary
           self.primary.write(key, value)

           # Replicate asynchronously
           for replica in self.replicas:
               Thread(target=replica.write, args=(key, value)).start()

           # Return immediately
           return True

**Quorum-based Replication**

Require writes and reads from a subset of replicas.

.. code-block:: python

   class QuorumReplicator:
       def __init__(self, replicas, write_quorum, read_quorum):
           self.replicas = replicas
           self.write_quorum = write_quorum
           self.read_quorum = read_quorum

       def write(self, key, value):
           # Write to write_quorum replicas
           successes = 0
           for replica in self.replicas:
               if replica.write(key, value):
                   successes += 1
                   if successes >= self.write_quorum:
                       return True
           return False

       def read(self, key):
           # Read from read_quorum replicas
           values = []
           for replica in self.replicas:
               value = replica.read(key)
               values.append(value)
               if len(values) >= self.read_quorum:
                   break

           # Resolve conflicts
           return self.resolve_conflicts(values)

**Leader-based Replication**

One replica (leader) handles all writes.

.. code-block:: python

   class LeaderReplicator:
       def __init__(self, leader, followers):
           self.leader = leader
           self.followers = followers

       def write(self, key, value):
           # Write to leader
           self.leader.write(key, value)

           # Replicate to followers
           for follower in self.followers:
               follower.write(key, value)

       def read(self, key, consistency='eventual'):
           if consistency == 'strong':
               # Read from leader
               return self.leader.read(key)
           else:
               # Read from any replica
               return random.choice(self.followers).read(key)

Distributed Transactions
------------------------

**Two-Phase Commit (2PC)**

Ensure atomic transactions across distributed nodes.

.. code-block:: python

   class TwoPhaseCommit:
       def __init__(self, participants):
           self.participants = participants

       def commit(self, transaction):
           # Phase 1: Prepare
           prepared = []
           for participant in self.participants:
               if participant.prepare(transaction):
                   prepared.append(participant)
               else:
                   # Abort if any participant fails
                   self.abort(transaction)
                   return False

           # Phase 2: Commit
           for participant in prepared:
               participant.commit(transaction)

           return True

       def abort(self, transaction):
           for participant in self.participants:
               participant.rollback(transaction)

**Three-Phase Commit (3PC)**

Improved version of 2PC that avoids blocking.

.. code-block:: python

   class ThreePhaseCommit:
       def __init__(self, participants):
           self.participants = participants

       def commit(self, transaction):
           # Phase 1: CanCommit
           can_commit = []
           for participant in self.participants:
               if participant.can_commit(transaction):
                   can_commit.append(participant)
               else:
                   return False

           # Phase 2: PreCommit
           pre_committed = []
           for participant in can_commit:
               if participant.pre_commit(transaction):
                   pre_committed.append(participant)
               else:
                   # Abort if any participant fails
                   self.abort(transaction)
                   return False

           # Phase 3: DoCommit
           for participant in pre_committed:
               participant.do_commit(transaction)

           return True

**Saga Pattern**

Distributed transactions without locking.

.. code-block:: python

   class SagaOrchestrator:
       def __init__(self, steps):
           self.steps = steps

       def execute(self, transaction):
           compensations = []

           # Execute each step
           for step in self.steps:
               try:
                   result = step.execute(transaction)
                   compensations.append(step.get_compensation(result))
               except Exception as e:
                   # Rollback using compensations
                   for compensation in reversed(compensations):
                       compensation()
                   raise

           return True

Best Practices
--------------

**Choose Appropriate Consistency Model**

Match consistency model to use case requirements.

.. code-block:: python

   # Financial transactions: Strong consistency
   financial_db = StrongConsistentStore(nodes)

   # Social media feeds: Eventual consistency
   social_feed_db = EventualConsistentStore(nodes)

**Implement Idempotent Operations**

Operations should produce same result on repeated execution.

.. code-block:: python

   class IdempotentOperation:
       def __init__(self, operation_store):
           self.operation_store = operation_store

       def execute(self, operation_id, func, *args):
           # Check if operation already executed
           if self.operation_store.exists(operation_id):
               return self.operation_store.get_result(operation_id)

           # Execute operation
           result = func(*args)

           # Store result
           self.operation_store.save(operation_id, result)

           return result

**Use Version Numbers**

Track data versions for conflict detection.

.. code-block:: python

   class VersionedStore:
       def __init__(self):
           self.data = {}
           self.versions = {}

       def write(self, key, value, expected_version):
           current_version = self.versions.get(key, 0)

           if expected_version is not None and current_version != expected_version:
               raise ConflictError("Version mismatch")

           self.data[key] = value
           self.versions[key] = current_version + 1

           return self.versions[key]

       def read(self, key):
           return {
               'value': self.data.get(key),
               'version': self.versions.get(key, 0)
           }

**Monitor Consistency**

Track consistency metrics and alerts.

.. code-block:: python

   class ConsistencyMonitor:
       def __init__(self, nodes):
           self.nodes = nodes

       def check_consistency(self, key):
           values = [node.read(key) for node in self.nodes]
           unique_values = set(values)

           if len(unique_values) > 1:
               self.alert_inconsistency(key, values)
               return False

           return True

Common Pitfalls
---------------

**Over-reliance on Strong Consistency**

Strong consistency limits availability and scalability.

**Ignoring Network Partitions**

Network partitions can cause consistency issues.

**Poor Conflict Resolution**

Inadequate conflict resolution leads to data loss.

**No Monitoring**

Without monitoring, consistency issues go unnoticed.

**Complex Transaction Logic**

Complex transactions are hard to maintain and debug.