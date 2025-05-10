// eslint.config.js
import js from '@eslint/js'
import tseslint from '@typescript-eslint/eslint-plugin'
import tsParser from '@typescript-eslint/parser'
import react from 'eslint-plugin-react'
import reactHooks from 'eslint-plugin-react-hooks'
import tailwindcss from 'eslint-plugin-tailwindcss'
import prettier from 'eslint-config-prettier'

export default [
  // 🧠 ESLint base rules (JS globals, recommended rules)
  js.configs.recommended,

  // 🧠 TypeScript + React flat config
  {
    files: ['**/*.{ts,tsx}'], // Match all TS and TSX files
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: { jsx: true },
        project: './tsconfig.json', // enable type-aware linting
      },
    },
    plugins: {
      '@typescript-eslint': tseslint,
      react,
      'react-hooks': reactHooks,
      tailwindcss,
    },
    rules: {
      // ✅ TypeScript recommended rules
      ...tseslint.configs.recommended.rules,

      // ✅ React recommended rules
      ...react.configs.recommended.rules,

      // ✅ Tailwind plugin
      ...tailwindcss.configs.recommended.rules,

      // 🛠 Custom tweaks
      'react/react-in-jsx-scope': 'off', // not needed in React 17+
      'react/prop-types': 'off', // not used in TS
      'tailwindcss/no-custom-classname': 'off', // allow custom classNames if needed
      'tailwindcss/classnames-order': 'warn',

      // 🧪 Additional strictness
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { argsIgnorePattern: '^_' },
      ],
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      '@typescript-eslint/no-explicit-any': 'warn',
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
    },
  },

  // 🧼 Prettier: disables rules that conflict with formatting
  prettier,
]
