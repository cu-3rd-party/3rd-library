module.exports = {
  // Translation file format, po is the most convenint
  format: "po",
  locales: ["ru", "en"],
  // Default locale
  sourceLocale: "ru",

  // Rules by which Lingui searches for and sums translation files
  catalogs: [
    {
      path: "<rootDir>/src/locales/{locale}/messages",
      include: ["src"],
    },
  ],
};
