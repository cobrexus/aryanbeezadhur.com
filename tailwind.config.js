const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.html'],
    theme: {
        extend: {},
        fontFamily: {
            body: ['Michroma', ...defaultTheme.fontFamily.mono]
        }
    },
    plugins: []
}
