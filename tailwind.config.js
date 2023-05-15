/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{html,rs}", "./index.html"],
    theme: {
        extend: {
            colors: {
                primary: "rgb(251, 113, 133)",
                secondary: "rgb(254, 240, 138)",
                success: "rgb(16, 185, 129)",
                danger: "rgb(69, 10, 10)",
                light: {
                    100: "rgb(216, 222, 225)", // Dark-end
                    200: "rgb(221, 227, 230)",
                    300: "rgb(226, 232, 235)",
                    400: "rgb(231, 237, 240)", // Default mid-point
                    500: "rgb(236, 242, 245)",
                    600: "rgb(241, 247, 255)",
                    700: "rgb(246, 252, 260)", // Light-end
                },
                dark: {
                    100: "rgb(21, 35, 55)",
                    200: "rgb(31, 45, 65)",
                    300: "rgb(41, 55, 75)",
                    400: "rgb(51, 65, 85)", // Default mid-point
                    500: "rgb(61, 75, 95)",
                    600: "rgb(71, 85, 105)",
                    700: "rgb(81, 95, 115)",
                },
            },
        },
    },
    plugins: [],
};
