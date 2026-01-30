---
title: "Document Workflow"
date: 2026-01-30T00:00:00
tags: ["diagrams", "state", "workflow"]
author: "Peta"
---


Document Approval Workflow
===========================

A state diagram showing the document approval lifecycle.

.. diagram:: state
   :title: Document Approval Lifecycle
   
   Draft -> Review : Submit
   Review -> Approved : Approve
   Review -> Rejected : Reject
   Review -> Revisions : Request Changes
   Revisions -> Review : Resubmit
   Approved -> Published : Publish
   Published -> Archived : Archive
   Rejected -> Draft : Revise
   Draft -> Archived : Delete
   Published -> Draft : Update