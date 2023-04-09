use crate::generators::create_file;
use crate::Project;
use std::io::Error;

pub fn create_files(project: &Project) -> Result<(), Error> {
    let files = vec![
        &project.rustyroad_toml,
        &project.rocket_toml,
        &project.cargo_toml,
        &project.main_rs,
        &project.package_json,
        &project.readme,
        &project.gitignore,
        &project.config_dev_env,
        &project.config_prod_env,
        &project.config_test_env,
        &project.config_default_env,
        &project.config_dev_db,
        &project.config_prod_db,
        &project.config_test_db,
        &project.routes_module,
        &project.models_module,
        &project.user_controller_module,
        &project.index_html,
        &project.base_html,
        &project.tailwind_css,
        &project.tailwind_config,
        &project.postcss_config,
        &project.not_found_html,
        &project.server_error_html,
        &project.favicon_ico,
        &project.robots_txt,
        &project.login_page_html,
        &project.signup_page_html,
        &project.reset_password_page_html,
        &project.forgot_password_page_html,
        &project.dashboard_page_html,
        &project.user_controller,
        &project.user_model,
        &project.initial_migration_up,
        &project.initial_migration_down,
        &project.user_test,
        &project.user_route,
        &project.index_route,
        &project.login_route,
        &project.signup_route,
        &project.reset_password_route,
        &project.forgot_password_route,
        &project.dashboard_route,
        &project.index_js,
        &project.navbar_component,
        &project.header_section,
    ];

    for file in files {
        create_file(file)?;
    }

    Ok(())
}
