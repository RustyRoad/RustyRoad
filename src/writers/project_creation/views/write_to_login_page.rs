use crate::writers::{controller_writer, write_to_controllers_mod, write_to_file};
use crate::Project;
use std::io::Error;

pub fn write_to_login_page(project: Project) -> Result<(), Error> {
    let contents = r#"
{% extends 'base.html.tera' %}
{% block title %}Login Page{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}
<div class='flex min-h-full items-center justify-center px-4 py-12 sm:px-6 lg:px-8'>
  <div class='w-full max-w-sm space-y-10'>
    <div>
      <img class='mx-auto h-10 w-auto' src='https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600'
        alt='Your Company'>
      <h2 class='mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900 dark:text-white dark:text-opacity-75
      dark:tracking-tight dark:font-bold'>
        Sign in to your account
      </h2>
    </div>
    <!-- Error message section -->
    {% if error %}
    <div class='text-center text-red-500'>
      {{ error }}
    </div>
    {% endif %}
    <form class='space-y-6' action='login' method='POST'>
      <div class='relative -space-y-px rounded-md shadow-sm'>
        <div class='pointer-events-none absolute inset-0 z-10 rounded-md ring-1 ring-inset ring-gray-300'></div>
        <div>
          <label for='username' class='sr-only'>Username</label>
          <input id='username' name='username' type='text' autocomplete='username' required
            class='relative block w-full rounded-t-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6'
            placeholder='Email address'>
        </div>
        <div>
          <label for='password' class='sr-only'>Password</label>
          <input id='password' name='password' type='password' autocomplete='current-password' required
            class='relative block w-full rounded-b-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6'
            placeholder='Password'>
        </div>
      </div>

      <div class='flex items-center justify-between'>
        <div class='flex items-center'>
          <input id='remember-me' name='remember-me' type='checkbox'
            class='h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600'>
          <label for='remember-me' class='ml-3 block text-sm leading-6 text-gray-900 dark:text-white dark:text-opacity-75
          dark:font-bold'>
            Remember me
          </label>
        </div>

        <div class='text-sm leading-6'>
          <a href='#' class='font-semibold text-indigo-600 hover:text-indigo-500'>Forgot password?</a>
        </div>
      </div>

      <div>
        <button type='submit'
          class='flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600'>Sign
          in</button>
      </div>
    </form>

    <p class='text-center text-sm leading-6 text-gray-500 dark:text-white dark:text-opacity-75'>
      Not a member?
      <a href='#' class='font-semibold text-indigo-600 hover:text-indigo-500 dark:text-indigo-400'>
        Start a 14-day free trial
      </a>
    </p>
  </div>
</div>
{% endblock content %}
"#.to_string();

    write_to_file(&project.login_page_html, contents.as_bytes())
        .unwrap_or_else(|why| panic!("Couldn't write to {}: {}", &project.login_page_html, why));

    controller_writer::write_to_initial_get_controller(project.login_controller.clone())
        .unwrap_or_else(|why| {
            panic!(
                "Couldn't write to the: {}: {}",
                &project.login_controller, why
            )
        });

    controller_writer::write_to_initial_post_controller_authentication(
        project.login_controller.clone(),
    )
    .unwrap_or_else(|why| {
        panic!(
            "Couldn't write to the: {}: {}",
            &project.login_controller, why
        )
    });

    write_to_controllers_mod(&project.controllers_module, "login".to_string()).unwrap_or_else(
        |why| {
            panic!(
                "Couldn't write to the: {}: {}",
                &project.login_controller, why
            )
        },
    );
    Ok(())
}
