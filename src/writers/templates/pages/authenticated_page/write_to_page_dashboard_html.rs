use std::fs::File;
use std::io::Write;
use std::path::Path;
use eyre::Error;

pub fn write_to_page_dashboard_html() -> Result<(), Error> {
    let contents = r#"
{% extends 'layouts/authenticated_page/authenticated_page.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}

{% block authenticated_content %}
{{ super() }}

<!-- Center the entire content -->
<div class="flex flex-col items-center justify-center min-h-screen  dark:border-slate-500">

    <!-- Create Page Button -->
    <div class="mb-4">
        <a href="/create_page" class="text-center dark:text-gray-800 text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900">
            Create Page
        </a>
    </div>


{% if error %}
        <div class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900">
            {{ error }}
        </div>
    {% endif %}

    <!-- List of Pages -->
    <div class="flex flex-col items-start">
        {% for page in pages %}
        <div class="flex flex-row gap-2 justify-between w-full">
            {% if page.id %}
            <!-- label the page -->
            <a href="/page/{{ page.id }}/details" class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900 mb-2">
                {% if page.title !="" %}Page Title: {{page.title}}{% else %}Page Id: {{page.id}}{% endif %}
            </a>
            <a href="/page/{% if page.slug %}{{ page.slug }}{% else %}{{ page.id }}{% endif %}"
            class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900 mb-2">
                View page
            </a>

            {% else %}
            <div class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900">
                You have no pages yet.
            </div>
            {% endif %}
        </div>
        {% endfor %}
    </div>
</div>
{% endblock authenticated_content %}
    "#;
    let path = Path::new("src/writers/templates/pages/authenticated_page/page_dashboard.html.tera");
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    println!("Finished creating the page_dashboard.html.tera file.");
    Ok(())
}
