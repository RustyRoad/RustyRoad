use crate::writers::write_to_file;
use std::io::Error;

pub fn write_to_header(name: &String) -> Result<(), Error> {
    let mut contents: String = r#"<script src="https://cdn.tailwindcss.com?plugins=forms,typography,aspect-ratio"></script>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{% block title %}{% endblock title %} - #Title</title>"#
        .to_string();
    // Append page title to the title tag
    contents = contents.replace("#Title", name.as_str());
    write_to_file(name, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &name, why));
    Ok(())
}

pub fn write_to_header_with_grapesjs(name: &String) -> Result<(), Error> {
    let mut contents: String =
        r#"<meta charset='utf-8'>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{% if page and page.seo_title %}{{ page.seo_title }}{% else %}Quantum Forge{% endif %}</title>
        {% if page and page.seo_title %}
            <meta name="title" content="{{ page.seo_title }}">
        {% endif %}
        {% if page and page.seo_description %}
            <meta name="description" content="{{ page.seo_description }}">
        {% endif %}
        {% if page and page.seo_keywords %}
            <meta name="keywords" content="{{ page.seo_keywords }}">
        {% endif %}
        {% if page and page.seo_focus_keyphrase %}
            <meta name="focus_keyphrase" content="{{ page.seo_focus_keyphrase }}">
        {% endif %}
        {% if page and page.seo_canonical_url %}
            <link rel="canonical" href="{{ page.seo_canonical_url }}">
        {% endif %}
        {% if page and page.seo_no_index %}
            <meta name="robots" content="noindex">
        {% endif %}
        {% if page and page.seo_no_follow %}
            <meta name="robots" content="nofollow">
        {% endif %}
        {% if page and page.seo_og_title %}
            <meta property="og:title" content="{{ page.seo_og_title }}">
        {% endif %}
        {% if page and page.seo_og_locale %}
            <meta property="og:locale" content="{{ page.seo_og_locale }}">
        {% endif %}
        {% if page and page.seo_og_type %}
            <meta property="og:type" content="{{ page.seo_og_type }}">
        {% endif %}
        {% if page and page.seo_og_description %}
            <meta property="og:description" content="{{ page.seo_og_description }}">
        {% endif %}
        {% if page and page.seo_og_image %}
            <meta property="og:image" content="{{ page.seo_og_image }}">
        {% endif %}
        {% if page and page.seo_og_image_width %}
            <meta property="og:image:width" content="{{ page.seo_og_image_width }}">
        {% endif %}
        {% if page and page.seo_og_image_height %}
            <meta property="og:image:height" content="{{ page.seo_og_image_height }}">
        {% endif %}
        {% if page and page.seo_twitter_title %}
            <meta property="twitter:title" content="{{ page.seo_twitter_title }}">
        {% endif %}
        {% if page and page.seo_twitter_description %}
            <meta property="twitter:description" content="{{ page.seo_twitter_description }}">
        {% endif %}
        {% if page and page.seo_twitter_image %}
            <meta property="twitter:image" content="{{ page.seo_twitter_image }}">
        {% endif %}
        {% if page and page.seo_twitter_image_alt %}
            <meta property="twitter:image:alt" content="{{ page.seo_twitter_image_alt }}">
        {% endif %}
        {% if page and page.seo_twitter_card %}
            <meta property="twitter:card" content="{{ page.seo_twitter_card }}">
        {% endif %}
        {% if page and page.schema_type %}
            <script type="application/ld+json">
                {
                    "@context": "https://schema.org",
                    "@type": "{{ page.schema_type }}",
                    {% if page.schema_page_type %}
                        "pageType": "{{ page.schema_page_type }}",
                    {% endif %}
                    {% if page.schema_article_type %}
                        "articleType": "{{ page.schema_article_type }}",
                    {% endif %}
                    {% if page.schema_description %}
                        "description": "{{ page.schema_description }}",
                    {% endif %}
                    {% if page.schema_author %}
                        "author": "{{ page.schema_author }}",
                    {% endif %}
                    {% if page.schema_publisher %}
                        "publisher": "{{ page.schema_publisher }}",
                    {% endif %}
                    {% if page.schema_image %}
                        "image": "{{ page.schema_image }}",
                    {% endif %}
                    {% if page.schema_url %}
                        "url": "{{ page.schema_url }}",
                    {% endif %}
                    {% if page.schema_name %}
                        "name": "{{ page.schema_name }}",
                    {% endif %}
                    {% if page.schema_headline %}
                        "headline": "{{ page.schema_headline }}",
                    {% endif %}
                    {% if page.schema_date_published %}
                        "datePublished": "{{ page.schema_date_published }}",
                    {% endif %}
                    {% if page.schema_date_modified %}
                        "dateModified": "{{ page.schema_date_modified }}"
                    {% endif %}
                }
            </script>
        {% endif %}
        {% block head %}{% endblock head %}
        "#.to_string();

    // append to the end of the header
    let header = std::fs::read_to_string("/views/sections/header.html").unwrap();

    contents = contents + &header;

    write_to_file(name, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &name, why));
    Ok(())
}
