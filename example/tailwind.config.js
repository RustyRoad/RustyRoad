module.exports = {
        darkMode: 'media',
        content: ['./templates/**/*.{html.tera,js}'],
        theme: {
            extend: {
            },
        },
        plugins: [
            require('@tailwindcss/forms'),
        ],
        };