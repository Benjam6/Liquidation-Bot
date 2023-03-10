/** @type {import('eslint').Linter.Config} */
module.exports = {
  parser: "@typescript-eslint/parser",
  parserOptions: { project: "./tsconfig.json" },
  settings: { "import/resolver": "typescript" },
  extends: [
    "eslint:recommended",
    "plugin:import/errors",
    "plugin:import/warnings",
    "plugin:node/recommended",
    "plugin:prettier/recommended",
    "plugin:eslint-comments/recommended",
    "plugin:@typescript-eslint/recommended",
  ],
  rules: {
    "no-console": "error",
    "node/no-missing-import": "off",
    "node/no-unpublished-import": "off",
    "@typescript-eslint/no-shadow": "error",
    "eslint-comments/no-unused-disable": "error",
    "@typescript-eslint/no-floating-promises": "error",
    "import/no-extraneous-dependencies": ["error", { devDependencies: true }],
    "@typescript-eslint/no-unused-vars": ["error", { ignoreRestSiblings: true }],
    "node/no-unsupported-features/es-syntax": ["error", { ignores: ["modules"] }],
  },
};
