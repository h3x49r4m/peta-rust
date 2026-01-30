Gantt Charts
============

Gantt charts are perfect for visualizing project timelines, schedules, and task dependencies. They display tasks as horizontal bars positioned along a timeline.

Basic Syntax
------------

The fundamental syntax for Gantt charts defines tasks with names, start dates, and durations:

.. diagram:: gantt
   :title: Basic Gantt Chart
   
   Task A [2024-01-01] : 5d
   Task B [2024-01-06] : 3d

This creates a timeline showing two sequential tasks.

Task Syntax
-----------

Creating Tasks
~~~~~~~~~~~~~~

Each task line consists of three parts:

1. **Task name**: Descriptive name for the task
2. **Start date**: In YYYY-MM-DD format, enclosed in brackets
3. **Duration**: Number followed by 'd' for days

Syntax:

.. code-block:: rst

   Task Name [YYYY-MM-DD] : Nd

Example:

.. diagram:: gantt
   
   Planning [2024-01-01] : 7d

Date Format
~~~~~~~~~~~

Dates must be in ISO 8601 format (YYYY-MM-DD):
- Year: 4 digits
- Month: 2 digits (01-12)
- Day: 2 digits (01-31)

Duration Syntax
~~~~~~~~~~~~~~~

Duration is specified as a number followed by 'd':
- `1d`: 1 day
- `5d`: 5 days
- `10d`: 10 days
- `30d`: 30 days

Sequential Tasks
----------------

Linear Timeline
~~~~~~~~~~~~~~~

Tasks that follow each other sequentially:

.. diagram:: gantt
   :title: Sequential Tasks
   
   Requirements [2024-01-01] : 5d
   Design [2024-01-06] : 7d
   Development [2024-01-13] : 14d
   Testing [2024-01-27] : 7d
   Deployment [2024-02-03] : 2d

Parallel Tasks
--------------

Overlapping Timelines
~~~~~~~~~~~~~~~~~~~~

Tasks that run concurrently:

.. diagram:: gantt
   :title: Parallel Tasks
   
   Frontend Dev [2024-01-01] : 10d
   Backend Dev [2024-01-01] : 10d
   Integration [2024-01-11] : 5d
   QA Testing [2024-01-16] : 5d

Complex Schedules
-----------------

Mixed Sequential and Parallel
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Combine sequential and parallel tasks:

.. diagram:: gantt
   :title: Complex Project Schedule
   
   Research [2024-01-01] : 10d
   Design [2024-01-08] : 7d
   Frontend Dev [2024-01-15] : 14d
   Backend Dev [2024-01-15] : 14d
   Database Setup [2024-01-15] : 5d
   Integration [2024-01-29] : 7d
   Testing [2024-02-05] : 7d
   Documentation [2024-02-05] : 5d
   Deployment [2024-02-12] : 2d

Practical Examples
------------------

Software Development Lifecycle
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. diagram:: gantt
   :title: Software Development Lifecycle
   
   Planning [2024-01-01] : 5d
   Analysis [2024-01-06] : 5d
   Design [2024-01-11] : 10d
   Implementation [2024-01-21] : 20d
   Testing [2024-02-10] : 10d
   Deployment [2024-02-20] : 3d
   Maintenance [2024-02-23] : 30d

Marketing Campaign
~~~~~~~~~~~~~~~~~~

.. diagram:: gantt
   :title: Marketing Campaign Timeline
   
   Market Research [2024-01-01] : 7d
   Strategy Development [2024-01-05] : 5d
   Content Creation [2024-01-10] : 14d
   Ad Setup [2024-01-15] : 5d
   Campaign Launch [2024-01-20] : 1d
   Monitoring [2024-01-21] : 30d
   Analysis [2024-02-20] : 5d

Product Launch
~~~~~~~~~~~~~~

.. diagram:: gantt
   :title: Product Launch Schedule
   
   Product Development [2024-01-01] : 45d
   Beta Testing [2024-02-01] : 14d
   Bug Fixes [2024-02-10] : 7d
   Marketing Materials [2024-02-01] : 21d
   Sales Training [2024-02-15] : 7d
   Launch Event [2024-02-22] : 1d
   Post-Launch Support [2024-02-23] : 14d

Tips and Best Practices
-----------------------

1. **Realistic timelines**: Estimate durations carefully
2. **Buffer time**: Add buffer between tasks
3. **Clear task names**: Use descriptive, concise names
4. **Logical ordering**: Arrange tasks chronologically
5. **Consider dependencies**: Ensure predecessor tasks complete before successors

Common Patterns
---------------

Sprint Planning
~~~~~~~~~~~~~~

.. diagram:: gantt
   :title: 2-Week Sprint
   
   Sprint Planning [2024-01-01] : 1d
   Development [2024-01-02] : 8d
   Code Review [2024-01-10] : 2d
   Testing [2024-01-12] : 2d
   Bug Fixes [2024-01-14] : 1d
   Sprint Demo [2024-01-15] : 1d

Release Cycle
~~~~~~~~~~~~~

.. diagram:: gantt
   :title: Monthly Release Cycle
   
   Feature Dev [2024-01-01] : 14d
   Integration [2024-01-15] : 3d
   QA Testing [2024-01-18] : 5d
   Staging [2024-01-23] : 2d
   Production [2024-01-25] : 1d
   Hotfix Window [2024-01-26] : 2d

Next Steps
----------

Now that you understand Gantt charts, let's explore sequence diagrams for visualizing interactions in the next chapter.