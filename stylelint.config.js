/** @type {import('stylelint').Config} */
export default {
  extends: ['@rhapsodic/stylelint-config'],
  rules: {
    'plugin/no-low-performance-animation-properties': [
      true,
      {
        ignoreProperties: ['color', 'background-color', 'background', 'border-color'],
      },
    ],
  },
};
