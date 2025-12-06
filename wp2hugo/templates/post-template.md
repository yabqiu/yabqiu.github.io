---
title: {{ post.title }}
url: /{{ post.name }}/
date: {{ post.post_date_gmt | date_format }}
featured: false
draft: false
toc: false
# menu: main
usePageBundles: true{% if post.feature_image %}
thumbnail: "../images/logos/{{ post.feature_image }}"{% endif %}
categories:{% for cat in post.categories %}
  - {{ cat }}{% endfor %}
tags: {% for tag in post.tags %}
  - {{ tag }}{% endfor %}
comment: true
# additional
wpPostId: {{ post.id }} {# post id in Wordpress #}
wpStatus: {{ post.status }}
views: {{ post.views }}
lastmod: {{ post.last_modified_gmt | date_format }}
---

{{ post_content }}
