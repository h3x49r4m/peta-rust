---
title: "Payment Processing"
date: 2026-01-30T00:00:00
tags: ["diagrams", "sequence", "payment"]
author: "Peta"
---


Payment Processing Flow
=======================

A sequence diagram showing the payment processing workflow.

.. diagram:: sequence
   :title: Online Payment Processing Flow
   
   Customer -> Checkout: Submit Payment
   Checkout -> Payment Gateway: Process Transaction
   Payment Gateway -> Bank: Authorization Request
   Bank -> Payment Gateway: Authorization Response
   Payment Gateway -> Checkout: Payment Status
   Checkout -> Inventory: Reserve Items
   Inventory -> Checkout: Reservation Confirmed
   Checkout -> Customer: Payment Confirmation
   Checkout -> Email Service: Send Receipt
   Email Service -> Customer: Receipt Email