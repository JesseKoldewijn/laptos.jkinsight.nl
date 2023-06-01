/** @type {import("prettier").Config} */
module.exports = {
    useTabs: false,
    tabWidth: 4,
    printWidth: 100,
    endOfLine: "lf",
    plugins: [require.resolve("prettier-plugin-rust")],
};
