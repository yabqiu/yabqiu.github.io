---
title: "{{ post.title }}"
date: {{ post.post_date_gmt | date_format }}
url: /{{ post.name }}
featured: false
draft: true
toc: false
# menu: main
usePageBundles: true
featureImage: "{{ post.feature_image }}" {# Sets featured image on blog post #}
thumbnail: "{{ post.feature_image }}" {# Sets thumbnail image appearing inside card on homepage. #}
codeMaxLines: 50
categories:{% for cat in post.categories %}
  - {{ cat }}{% endfor %}
tags: {% for tag in post.tags %}
  - {{ tag }}{% endfor %}
comment: true
# additional
wpPostId: {{ post.id }} {# post id in Wordpress #}
views: {{ post.views }}
lastModified: {{ post.last_modified_gmt | date_format }}
---

{{ post.content }}
