---
url: "/my-first-post/" # The URL of the blog post.
title: "My First Post" # Title of the blog post.
date: 1900-11-30T12:04:21-06:00 # Date of post creation.
description: "Article description." # Description used for search engine.
featureImage: "../images/logos/rust-logo.png" # Top image on post.
featured: true # Sets if post is a featured post, making appear on the home page side bar.
draft: false # Sets whether to render this page. Draft of true will not be rendered.
toc: false # Controls if a table of contents should be generated for first-level links automatically.
# menu: main
usePageBundles: true # Set to true to group assets like images in the same folder as this post.
thumbnail: "../images/logos/rust-logo.png" # Sets thumbnail image appearing inside card on homepage.
codeMaxLines: 10 # Override global value for how many lines within a code block before auto-collapsing.
codeLineNumbers: false # Override global value for showing of line numbers within code block.
categories:
  - Technology
tags:
  - Tag_name1
  - Tag_name2
# comment: false # Disable comment if false.
---

**Insert Lead paragraph here.**

```json
{
  "key": "value"
}
```

Post summary here
<br/>
换行 <!--more-->

The remaining content of the post

```go {linenos=inline hl_lines=[3,"6-8"]}
package main // Package declaration

import "fmt" 

func main() {
    for i := 0; i < 3; i++ {
        fmt.Println("Value of i:", i)
    }
}
```

The remaining content of the post

## bundle image
![python](python3.14-new-features-1.png)

{{< figure src="python3.14-new-features-1.png" width="300px" >}}

## image from static/images
![rust-logo](../../../../../images/logos/rust-logo.png)

{{< figure src="../images/logos/rust-logo.png" width="150px" >}}