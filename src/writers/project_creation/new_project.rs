use crate::Project;

pub fn new(name: String) -> Project {
    let timestamp = chrono::offset::Local::now().format("%Y%m%d%H%M%S");

    let src_dir = format!("{name}/src");
    let rustyroad_toml = format!("{name}/rustyroad.toml");
    let cargo_toml = format!("{name}/Cargo.toml");
    let main_rs = format!("{src_dir}/main.rs");
    let package_json = format!("{name}/package.json");
    let readme = format!("{name}/README.md");
    let gitignore = format!("{name}/.gitignore");
    let templates = format!("{}/views", src_dir);
    let static_dir = format!("{}/static", name);
    let template_components = format!("{}/components", templates);
    let template_sections = format!("{}/sections", templates);
    let template_layouts = format!("{}/layouts", templates);
    let auth_template_layouts = format!("{}/authenticated", template_layouts);
    let template_pages = format!("{}/pages", templates);
    let static_css = format!("{}/css", static_dir);
    let static_js = format!("{}/js", static_dir);
    let index_js = format!("{}/index.js", static_js);
    let static_images = format!("{}/images", static_dir);
    let config = format!("{}/config", name);
    let config_env = format!("{}/environments", config);
    let config_dev_env = format!("{}/dev.env", config_env);
    let config_prod_env = format!("{}/prod.env", config_env);
    let config_test_env = format!("{}/test.env", config_env);
    let config_default_env = format!("{}/default.env", config_env);
    let db = format!("{}/database", config);
    let config_dev_db = format!("{}/dev.db", db);
    let config_prod_db = format!("{}/prod.db", db);
    let config_test_db = format!("{}/test.db", db);
    let controllers = format!("{}/controllers", src_dir);
    let controllers_module = format!("{}/mod.rs", controllers);
    let controllers = format!("{}/controllers", src_dir);
    let models = format!("{}/models", src_dir);
    let models_module = format!("{}/mod.rs", models);
    let migrations = format!("{}/migrations", db);
    let seeders = format!("{}/seeders", name);
    let tests = format!("{}/tests", name);
    let config_initializers = format!("{}/initializers", config);
    let config_initializers_assets = format!("{}/assets.rs", config_initializers);
    let config_initializers_db = format!("{}/initialize_db.sql", config_initializers);
    let config_initializers_default = format!("{}/default.rs", config_initializers);
    let config_initializers_middleware = format!("{}/middleware.rs", config_initializers);
    let config_initializers_controllers = format!("{}/controllers.rs", config_initializers);
    let index_html = format!("{}/index.html.tera", template_pages);
    let base_html = format!("{}/base.html.tera", templates);
    let tailwind_css = format!("{}/tailwind.css", src_dir);
    let tailwind_config = format!("{}/tailwind.config.js", name);
    let postcss_config = format!("{}/postcss.config.js", name);
    let not_found_controller = format!("{}/not_found.rs", controllers);
    let not_found_html = format!("{}/404.html.tera", template_pages);
    let server_error_html = format!("{}/500.html.tera", template_pages);
    let favicon_ico = format!("{}/favicon.ico", static_images);
    let robots_txt = format!("{}/robots.txt", static_dir);
    let login_page_html = format!("{}/login.html.tera", template_pages);
    let signup_page_html = format!("{}/signup.html.tera", template_pages);
    let reset_password_page_html = format!("{}/reset_password.html.tera", template_pages);
    let forgot_password_page_html = format!("{}/forgot_password.html.tera", template_pages);
    let dashboard_page_html = format!("{}/dashboard.html.tera", template_pages);
    let authenticated_layout = format!("{}/authenticated.html.tera", auth_template_layouts);
    let user_controller_directory = format!("{}/user", controllers);
    let user_controller = format!("{}/user.rs", user_controller_directory);
    let user_controller_module = format!("{}/mod.rs", user_controller_directory);
    let user_model = format!("{}/user.rs", models);
    let initial_migration_directory = format!("{}/{}_user", migrations, timestamp);
    let initial_migration_up = format!("{}/up.sql", initial_migration_directory);
    let initial_migration_down = format!("{}/down.sql", initial_migration_directory);
    let user_test = format!("{}/user.rs", tests);
    let index_controller = format!("{}/index.rs", controllers);
    let login_controller = format!("{}/login.rs", controllers);
    let signup_controller = format!("{}/signup.rs", controllers);
    let reset_password_controller = format!("{}/reset_password.rs", controllers);
    let forgot_password_controller = format!("{}/forgot_password.rs", controllers);
    let dashboard_controller = format!("{}/dashboard.rs", controllers);
    let navbar_component = format!("{}/navbar.html.tera", template_components);
    let header_section = format!("{}/header.html.tera", template_sections);

    Project {
        name,
        src_dir,
        rustyroad_toml,
        cargo_toml,
        main_rs,
        package_json,
        readme,
        gitignore,
        templates,
        static_dir,
        template_components,
        template_sections,
        template_layouts,
        auth_template_layouts,
        template_pages,
        static_css,
        static_js,
        index_js,
        static_images,
        config,
        config_env,
        config_dev_env,
        config_prod_env,
        config_test_env,
        config_default_env,
        db,
        config_dev_db,
        config_prod_db,
        config_test_db,
        controllers,
        controllers_module,
        models,
        models_module,
        migrations,
        seeders,
        tests,
        config_initializers,
        config_initializers_assets,
        config_initializers_db,
        config_initializers_default,
        config_initializers_middleware,
        config_initializers_controllers,
        index_html,
        base_html,
        tailwind_css,
        tailwind_config,
        postcss_config,
        not_found_controller,
        not_found_html,
        server_error_html,
        favicon_ico,
        robots_txt,
        login_page_html,
        signup_page_html,
        reset_password_page_html,
        forgot_password_page_html,
        dashboard_page_html,
        user_controller_directory,
        user_controller,
        user_controller_module,
        user_model,
        initial_migration_directory,
        initial_migration_up,
        initial_migration_down,
        user_test,
        index_controller,
        login_controller,
        signup_controller,
        reset_password_controller,
        forgot_password_controller,
        dashboard_controller,
        navbar_component,
        header_section,
    }
}
