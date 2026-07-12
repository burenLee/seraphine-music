export default {
  printWidth: 100,
  semi: false,
  singleQuote: true,
  trailingComma: 'none',
  bracketSameLine: true,

  plugins: ['prettier-plugin-tailwindcss', '@trivago/prettier-plugin-sort-imports'],
  importOrderSeparation: true, // import 分组之间不插入空行
  importOrderSortSpecifiers: true // 同一个 import 语句内部，对花括号里的导入项进行排序
}
