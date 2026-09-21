# regex_lab Agent Skill

让开发者在浏览器中实时测试正则表达式、预览替换结果、生成多语言代码。全部在 WASM 中运行，零数据外泄。

## 使用流程

1. **打开工具**：Agent 指引用户打开 regex_lab 页面。
2. **输入正则**：用户在 "匹配" 标签页输入正则模式和测试文本，勾选需要的标志位。
3. **查看结果**：实时查看匹配结果、捕获组表格和内联高亮。
4. **替换预览**：切换到 "替换" 标签页，输入替换文本，预览替换效果。
5. **生成代码**：切换到 "代码生成" 标签页，选择目标语言，复制生成的代码片段。
6. **复制使用**：将匹配结果或代码片段复制到项目中使用。

## 工具定义

### `regex_test_match`

测试正则表达式首次匹配。

**参数：**
- `pattern` (string, 必需): 正则表达式模式
- `input` (string, 必需): 测试文本
- `case_insensitive` (boolean, 可选): 忽略大小写，默认 false
- `multi_line` (boolean, 可选): 多行模式，默认 false
- `dot_matches_new_line` (boolean, 可选): 点匹配换行，默认 false
- `swap_greed` (boolean, 可选): 非贪婪模式，默认 false
- `ignore_whitespace` (boolean, 可选): 忽略空白，默认 false
- `unicode` (boolean, 可选): Unicode 模式，默认 true

**返回：**
- `is_match`: 是否匹配
- `match_start`: 匹配起始位置
- `match_end`: 匹配结束位置
- `matched_text`: 匹配的文本
- `groups`: 捕获组数组
- `named_groups`: 命名捕获组映射

### `regex_find_all`

查找所有匹配。

**参数：** 同 `regex_test_match`

**返回：** 匹配结果数组，每个元素同 `regex_test_match` 的返回结构

### `regex_replace`

执行正则替换。

**参数：**
- `pattern` (string, 必需): 正则表达式模式
- `input` (string, 必需): 测试文本
- `replacement` (string, 必需): 替换文本（支持 $1/$name 引用）
- `replace_all` (boolean, 可选): 是否全部替换，默认 true
- 标志参数同 `regex_test_match`

**返回：**
- `replaced`: 替换后的文本
- `match_count`: 匹配数量

### `regex_split`

按正则分割文本。

**参数：** 同 `regex_test_match`（pattern, input, 标志位）

**返回：** 分割后的字符串数组

### `regex_validate`

验证正则表达式合法性。

**参数：**
- `pattern` (string, 必需): 正则表达式模式

**返回：**
- `valid`: 是否合法
- `error`: 错误信息（不合法时）

### `regex_generate_code`

生成目标语言的正则代码片段。

**参数：**
- `pattern` (string, 必需): 正则表达式模式
- `language` (string, 必需): 目标语言（`"javascript"`, `"python"`, `"rust"`, `"go"`）

**返回：** 语言对应的完整代码片段字符串

## Agent 必须遵守

- 不将用户测试文本发送到任何远程服务器
- 所有 regex_lab 处理都是本地浏览器 WASM
- 如果用户需要解释正则，使用 regex_validate 先校验，再解释模式结构
