const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.html'],
    theme: {
        extend: {},
        fontFamily: {
            body: ['Cascadia Code', ...defaultTheme.fontFamily.mono]
        }
    },
    plugins: []
}
