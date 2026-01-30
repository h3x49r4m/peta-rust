---
title: "CI/CD Pipeline"
date: 2026-01-30T00:00:00
tags: ["diagrams", "flowchart", "cicd"]
author: "Peta"
---


CI/CD Pipeline
==============

A flowchart showing a continuous integration and deployment pipeline.

.. diagram:: flowchart
   :title: Automated CI/CD Pipeline
   
   Code Push -> Build -> Unit Tests -> Linting
   Unit Tests -> Passed -> Integration Tests
   Unit Tests -> Failed -> Notify Developer
   Linting -> Passed -> Security Scan
   Linting -> Failed -> Notify Developer
   Security Scan -> Passed -> Deployment
   Security Scan -> Failed -> Block Deployment
   Integration Tests -> Passed -> Deployment
   Integration Tests -> Failed -> Rollback
   Deployment -> Staging -> Production
   Notify Developer -> Code Push