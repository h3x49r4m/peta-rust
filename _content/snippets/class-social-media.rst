---
title: "Social Media Platform"
date: 2026-01-30T00:00:00
tags: ["diagrams", "class", "architecture"]
author: "Peta"
---


Social Media Architecture
==========================

A class diagram showing the relationships in a social media platform.

.. diagram:: class-diagram
   :title: Social Media Platform Model
   
   User |+| Post
   User |+| Comment
   User |+| Like
   User |+| Follow
   Post |+| Comment
   Post |+| Like
   Post |o| Media
   Comment |+| Like
   Comment |o| Media
   Group |+| Post
   Group |+| User
   Notification |o| User
   Notification |o| Post