module.exports = {
  extends: [ 'standard', 'plugin:vue/essential' ],
  plugins: ['align-assignments' ],
  env:     { browser: true, node: true, mocha: true },
  rules:   {
    'array-bracket-spacing':               ['warn', 'always'],
    'comma-dangle':                        ['warn', 'always-multiline'],
    'key-spacing':                         ['warn', { "align": 'value' }],
    'no-unused-vars':                      ['warn', { vars: 'all' }],
    'no-multi-spaces':                     'off',
    'padded-blocks':                       ['warn', 'never'],
    'align-assignments/align-assignments': ['warn', { 'requiresOnly': false } ],
    'vue/multi-word-component-names':      'off',
  },
}
