module.exports = {
    env: {
        browser: true,
        es6: true,
        node: true,
    },
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint'],
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/eslint-recommended',
        'plugin:@typescript-eslint/recommended',
        'prettier',
        'prettier/@typescript-eslint',
    ],
    rules: {
        // allow interop with js
        '@typescript-eslint/no-explicit-any': 'off',
        '@typescript-eslint/ban-ts-ignore': 'warn',

        // allow types to be declared anywhere in module
        '@typescript-eslint/no-use-before-define': 'off',

        // allow implicit void return
        '@typescript-eslint/explicit-function-return-type': 'off',

        // allow unwrapping
        '@typescript-eslint/no-non-null-assertion': 'off',

        // unused vars is just asking for bugs
        '@typescript-eslint/no-unused-vars': [
            'warn', // TODO: make this error once library has stablized
            {
                varsIgnorePattern: '_.*',
                argsIgnorePattern: '_.*',
            },
        ],
    },
};
