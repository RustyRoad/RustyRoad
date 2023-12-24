use eyre::Error;
use color_eyre::Result;
use crate::generators::create_file;
use crate::writers::write_to_file;

/// # Name: write_to_page_dashboard_html
/// This function writes to the page_dashboard.html.tera file.
/// The page_dashboard.html.tera file is the main listing page for all pages in the website.
/// From this page, the user can create a new page, view a page, or edit a page.
/// # Arguments:
/// * None
/// # Returns:
/// * Result<(), Error>
/// # Example:
/// ```
/// use rustyroad::writers::write_to_page_dashboard_html;
/// use eyre::Error;
///
/// write_to_page_dashboard_html();
/// ```
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

    println!("Creating the page.html.tera file...");
    // add the new View/Template: PageDashboard.html.tera to the views/pages directory
    let page_dashboard_path = format!("./src/views/layouts/authenticated_page/page/page_dashboard.html.tera");

    create_file(&page_dashboard_path).expect("Error creating the page_dashboard.html.tera file");
    println!("Writing the page.html.tera file...");
    write_to_file(&page_dashboard_path, contents.as_bytes()).expect("Error writing to the page_dashboard.html.tera file");
    Ok(())
}