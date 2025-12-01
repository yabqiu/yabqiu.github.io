---
title: "{{ post.title }} | 隔叶黄莺 Yanbin's Blog - 软件编程实践"
date: {{ post.post_date_gmt | date_format }}
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
