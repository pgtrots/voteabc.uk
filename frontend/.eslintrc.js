module.exports = {
    env: {
        browser: true,
        es6: true,
    },
    extends: 'eslint:recommended',
    globals: {
        Atomics: 'readonly',
        SharedArrayBuffer: 'readonly',
    },
    parserOptions: {
        ecmaVersion: 2019,
        sourceType: 'module',
    },
    plugins: ['svelte3'],
    overrides: [
        {
            files: ['**/*.svelte'],
            processor: 'svelte3/svelte3',
        },
    ],
    rules: {
        'prettier/prettier': 'error',
        'svelte3/lint-template': 2,
        indent: ['error', 4],
        'linebreak-style': ['error', 'unix'],
        quotes: ['error', 'single'],
        semi: ['error', 'always'],
    },
};
