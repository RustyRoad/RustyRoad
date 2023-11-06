use eyre::Error;
use color_eyre::Result;
use crate::generators::create_file;
use crate::writers::write_to_file;

pub fn write_to_get_all_pages_html() -> Result<(), Error> {
    let contents = r#"{% extends 'layouts/authenticated_page/authenticated_page.html.tera' %}
    {% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}
    
    {% block authenticated_content %}
    {{ super() }}
    
    <!-- Center the entire content -->
    <div class="flex flex-col items-center justify-center min-h-screen">
    
        <!-- Create Page Button -->
        <div class="mb-4">
            <a href="/page" class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900">
                Create Page
            </a>
        </div>
    
        <!-- List of Pages -->
        <div class="flex flex-col items-center">
            {% for page in pages %}
             <div class="flex flex-row gap-2">
                <!-- label the page -->
                <div class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900 mb-2">
                    {{page.id}}
                </div>
                <a href="/page/{{page.id}}" class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900 mb-2">
                    View page
                </a>
                <!-- edit page option -->
                <a href="/page/{{page.id}}/edit" class="text-center text-gray-700 bg-gray-50 hover:bg-gray-100 rounded-md p-2 text-sm leading-6 font-semibold dark:hover:text-gray-100 dark:hover:bg-slate-900 mb-2">
                    Edit Page
                </a>
             </div>
            {% endfor %}
        </div>
    
    </div>
    {% endblock authenticated_content %}"#;

    println!("Creating the page.html.tera file...");
    create_file("src/views/pages/pages.html.tera").expect("Error creating the pages.html.tera file");
    println!("Writing the page.html.tera file...");
    write_to_file("src/views/pages/pages.html.tera", contents.as_bytes())
        .expect("Error writing the pages.html.tera file");

    Ok(())
}