// @ts-nocheck
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['src/**/*.rs', 'index.html'],
  theme: {
    fontFamily: {
      sans: ['Source Code Pro', 'sans'],
    },
    extend: {
      width: {
        128: '32rem',
        172: '48rem',
        256: '64rem',
      },
    },
  },
  plugins: [],
};
