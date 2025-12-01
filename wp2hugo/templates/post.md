---
title: "{{ post.title }}"
date: {{ post.post_date_gmt }}
featured: true
draft: false
toc: false
# menu: main
usePageBundles: true
featureImage: "{{ post.feature_image }}"
thumbnail: "{{ post.feature_image }}"
codeMaxLines: 10
categories:{% for cat in post.categories %}
  - {{ cat }}{% endfor %}
tags: {% for tag in post.tags %}
  - {{ tag }}{% endfor %}
comment: true
---

{{ post.content }}
