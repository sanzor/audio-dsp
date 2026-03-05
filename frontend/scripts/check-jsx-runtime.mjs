import { createRequire } from 'node:module'

const require = createRequire(import.meta.url)

function assertResolvable(specifier) {
  try {
    require.resolve(specifier)
  } catch {
    console.error(
      [
        `TypeScript JSX runtime requires \`${specifier}\`, but it could not be resolved.`,
        '',
        'Fix:',
        '- Ensure dependencies are installed for this package (e.g. `pnpm install` from `./frontend`).',
        '- Ensure `react` is installed at v17+ (it must provide `react/jsx-runtime`).',
        '',
        'Tip:',
        '- If you recently changed React versions, delete `node_modules` and reinstall.',
      ].join('\n'),
    )
    process.exitCode = 1
  }
}

assertResolvable('react/jsx-runtime')
assertResolvable('react/jsx-dev-runtime')

