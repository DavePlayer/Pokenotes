/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        default: "#14A098",
        "default-light": "#14A098",
        "default-lightest": "#51FFF5",
        "default-dark": "#184049",
        "default-darkest": "#0F292F",
      },
    },
  },
  plugins: [],
};
