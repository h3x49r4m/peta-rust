---
title: "Monitoring and Rollback"
date: 2026-02-07T00:00:00
tags: ["ci-cd", "monitoring", "rollback"]
description: "Monitoring deployments and implementing rollback strategies"
---

Monitoring and Rollback
=======================

Effective monitoring and rollback strategies are essential for maintaining system reliability and confidence in your CI/CD pipeline.

Monitoring
----------

**Application Performance Monitoring (APM)**

Monitor application performance and health.

.. code-block:: python

   from prometheus_client import start_http_server, Counter, Histogram

   # Define metrics
   REQUEST_COUNT = Counter('http_requests_total', 'Total HTTP requests')
   REQUEST_DURATION = Histogram('http_request_duration_seconds', 'HTTP request duration')

   @REQUEST_DURATION.time()
   def handle_request(request):
       REQUEST_COUNT.inc()
       # Handle request
       pass

**Health Checks**

Implement health check endpoints.

.. code-block:: python

   @app.route('/health')
   def health_check():
       try:
           # Check database connection
           db.session.execute('SELECT 1')

           # Check external services
           external_service.ping()

           return jsonify({
               "status": "healthy",
               "timestamp": datetime.utcnow().isoformat()
           }), 200
       except Exception as e:
           return jsonify({
               "status": "unhealthy",
               "error": str(e)
           }), 503

**Log Aggregation**

Centralize logs for easy analysis.

.. code-block:: python

   import logging
   import logging.handlers

   # Configure logging
   logger = logging.getLogger(__name__)
   logger.setLevel(logging.INFO)

   # Add file handler
   file_handler = logging.handlers.RotatingFileHandler(
       'app.log',
       maxBytes=10485760,
       backupCount=5
   )
   file_handler.setFormatter(logging.Formatter(
       '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
   ))
   logger.addHandler(file_handler)

**Error Tracking**

Track errors and exceptions.

.. code-block:: python

   import sentry_sdk

   sentry_sdk.init(
       dsn="your-sentry-dsn",
       traces_sample_rate=1.0,
       environment="production"
   )

   # Errors are automatically captured

Monitoring Metrics
------------------

**Key Performance Indicators (KPIs)**

Track critical metrics:

- Response time
- Error rate
- Throughput
- Resource utilization

.. code-block:: python

   # Collect metrics
   metrics = {
       "response_time": get_response_time(),
       "error_rate": calculate_error_rate(),
       "throughput": calculate_throughput(),
       "cpu_usage": get_cpu_usage(),
       "memory_usage": get_memory_usage()
   }

**Business Metrics**

Track metrics that matter to your business:

- Active users
- Conversion rate
- Revenue
- Customer satisfaction

.. code-block:: python

   business_metrics = {
       "active_users": get_active_users(),
       "conversion_rate": calculate_conversion_rate(),
       "revenue": calculate_revenue(),
       "nps_score": calculate_nps()
   }

**Deployment Metrics**

Track deployment success and impact:

- Deployment frequency
- Lead time for changes
- Time to restore service
- Change failure rate

.. code-block:: python

   deployment_metrics = {
       "deployments_today": count_deployments_today(),
       "lead_time": calculate_lead_time(),
       "mean_time_to_restore": calculate_mttr(),
       "failure_rate": calculate_failure_rate()
   }

Alerting
--------

**Alerting Strategy**

Define clear alerting rules and thresholds.

.. code-block:: yaml

   # Prometheus alerting rules
   groups:
     - name: application_alerts
       rules:
         - alert: HighErrorRate
           expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
           for: 5m
           annotations:
             summary: "High error rate detected"

         - alert: HighResponseTime
           expr: histogram_quantile(0.95, http_request_duration_seconds) > 1
           for: 5m
           annotations:
             summary: "95th percentile response time is high"

**Alert Escalation**

Implement escalation policies for critical alerts.

.. code-block:: python

   def handle_alert(alert):
       if alert.severity == "critical":
           # Page on-call engineer
           page_on_call(alert)

           # Notify team
           notify_team(alert)

       elif alert.severity == "warning":
           # Send notification
           send_notification(alert)

**Silencing Alerts**

Allow temporary silencing of alerts during maintenance.

.. code-block:: python

   def is_alert_silenced(alert):
       silences = get_active_silences()
       for silence in silences:
           if silence.matches(alert):
               return True
       return False

Rollback Strategies
-------------------

**Blue-Green Rollback**

Instant rollback by switching traffic back.

.. code-block:: yaml

   rollback:
     stage: rollback
     script:
       - kubectl patch service myapp -p '{"spec":{"selector":{"version":"green"}}}'

**Canary Rollback**

Gradual rollback by reducing canary traffic.

.. code-block:: yaml

   rollback-canary:
     stage: rollback
     script:
       - kubectl scale deployment myapp-canary --replicas=0

**Database Rollback**

Revert database migrations if needed.

.. code-block:: python

   # Alembic rollback
   def downgrade():
       op.drop_column('users', 'new_field')

   # Run rollback
   alembic downgrade -1

**Feature Flag Rollback**

Disable problematic features without deployment.

.. code-block:: python

   # Disable feature flag
   feature_flags.disable('problematic_feature')

Automated Rollback
------------------

**Health-Based Rollback**

Automatically rollback if health checks fail.

.. code-block:: yaml

   deploy:
     stage: deploy
     script:
       - ./deploy.sh
       - ./verify-health.sh
     on_failure:
       - ./rollback.sh

**Metrics-Based Rollback**

Automatically rollback based on metrics thresholds.

.. code-block:: yaml

   verify:
     stage: verify
     script:
       - ERROR_RATE=$(get_error_rate)
       - if (( $(echo "$ERROR_RATE > 0.05" | bc -l) )); then
           echo "Error rate too high, rolling back"
           ./rollback.sh
           exit 1
         fi

**Time-Based Rollback**

Rollback if verification takes too long.

.. code-block:: yaml

   verify:
     stage: verify
     timeout: 10 minutes
     script:
       - ./verify.sh
     on_failure:
       - ./rollback.sh

Manual Rollback
---------------

**One-Click Rollback**

Provide a simple rollback mechanism.

.. code-block:: yaml

   rollback:
     stage: rollback
     script: ./rollback.sh
     when: manual
     allow_failure: true

**Rollback Validation**

Validate that rollback was successful.

.. code-block:: python

   def verify_rollback():
       # Check application health
       if not check_health():
           return False

       # Verify metrics are back to normal
       if metrics.error_rate > threshold:
           return False

       return True

Best Practices
--------------

**Set Clear Thresholds**

Define clear thresholds for alerts and rollbacks.

.. code-block:: python

   ALERT_THRESHOLDS = {
       "error_rate": 0.05,
       "response_time": 1.0,
       "cpu_usage": 0.8,
       "memory_usage": 0.9
   }

**Test Rollback Procedures**

Regularly test rollback procedures.

.. code-block:: yaml

   test-rollback:
     stage: test
     script:
       - ./deploy.sh
       - ./rollback.sh
       - ./verify-rollback.sh

**Document Rollback Procedures**

Maintain clear documentation for rollback procedures.

.. code-block:: markdown

   # Rollback Procedure

   1. Identify the failed deployment
   2. Run rollback script: `./rollback.sh <deployment-id>`
   3. Verify rollback: `./verify-rollback.sh`
   4. Monitor metrics
   5. Notify team

**Learn from Rollbacks**

Analyze rollback causes to prevent future issues.

.. code-block:: python

   def analyze_rollback(deployment_id):
       # Collect metrics
       metrics = get_deployment_metrics(deployment_id)

       # Analyze logs
       logs = get_deployment_logs(deployment_id)

       # Identify root cause
       root_cause = analyze_failure(metrics, logs)

       # Document findings
       document_rollback_analysis(deployment_id, root_cause)

Common Pitfalls
---------------

**No Rollback Plan**

Deploying without a rollback plan is risky.

**Slow Rollback**

Slow rollbacks prolong downtime.

**Incomplete Rollback**

Partial rollbacks can cause issues.

**No Monitoring**

Deploying without monitoring makes rollbacks harder.

**Ignoring Alerts**

Ignoring alerts leads to larger problems.