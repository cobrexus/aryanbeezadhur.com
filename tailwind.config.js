const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.rs', './index.html'],
    theme: {
        extend: {},
        fontFamily: {
            body: ['Instrument Sans', ...defaultTheme.fontFamily.mono]
        }
    },
    plugins: []
}
