module.exports = {
        darkMode: 'media',
        content: ['./src/views/**/*.{html.tera,js}'],
        theme: {
            extend: {
            },
        },
        plugins: [
            require('@tailwindcss/forms'),
        ],
        };