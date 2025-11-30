use crate::writers::write_to_file;
use eyre::Error;

pub fn write_to_404_html(file_name: &str) -> Result<(), Error> {
    let contents = r#"
    {% extends 'base.html.tera' %}
{% block title %}404 Page Not Found{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}
<main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
    <div class="text-center">
      <p class="text-base font-semibold text-indigo-600">404</p>
      <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Page not found</h1>
      <p class="mt-6 text-base leading-7 text-gray-600">{{error | default(value="The page you're looking for does not exist.", boolean=true)}}</p>
      <div class="mt-10 flex items-center justify-center gap-x-6">
        <a href="{% if user %}/dashboard{% else %}/{% endif %}"
        class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Go back home</a>
      </div>
    </div>
  </main>
{% endblock content %}
    "#;

    write_to_file(file_name, contents.as_bytes())?;

    Ok(())
}
