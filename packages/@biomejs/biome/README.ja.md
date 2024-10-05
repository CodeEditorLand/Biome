<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
        <img alt="Biome - Toolchain of the web" src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
    </picture>
</p>

<div align="center">

[![Discord chat][discord-badge]][discord-url] [![CI on main][ci-badge]][ci-url]
[![npm version][npm-badge]][npm-url]
[![VSCode version][vscode-badge]][vscode-url]
[![Open VSX version][open-vsx-badge]][open-vsx-url]

[discord-badge]:
	https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=green
[discord-url]: https://biomejs.dev/chat
[ci-badge]:
	https://github.com/biomejs/biome/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/biomejs/biome/actions/workflows/main.yml
[npm-badge]:
	https://badgen.net/npm/v/@biomejs/biome?icon=npm&color=green&label=%40biomejs%2Fbiome
[npm-url]: https://www.npmjs.com/package/@biomejs/biome/v/latest
[vscode-badge]:
	https://badgen.net/vs-marketplace/v/biomejs.biome?label=vscode&icon=visualstudio&color=green
[vscode-url]: https://marketplace.visualstudio.com/items?itemName=biomejs.biome
[open-vsx-badge]:
	https://badgen.net/open-vsx/version/biomejs/biome?label=open-vsx&color=green
[open-vsx-url]: https://open-vsx.org/extension/biomejs/biome

</div>

<!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/biomejs/biome/tree/main/packages/%40biomejs/biome -->
<div align="center">

[हिन्दी](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.hi.md)
|
[English](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.md)
|
[繁體中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-TW.md)
|
[简体中文](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.zh-CN.md)
| 日本語 |
[Português do Brasil](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.pt-br.md)
|
[한글](https://github.com/biomejs/biome/blob/main/packages/%40biomejs/biome/README.kr.md)

</div>

**Biome** はWebプロジェクトのための高性能なツールチェーンであり、プロジェクトの
健全性を維持するための開発者ツールの提供を目的としています。

**Biome は _JavaScript_, _TypeScript_, _JSX_ そして _JSON_ 向け
の[高速なFormatter](./benchmark#formatting)**であ
り、**[_Prettier_ との互換性は97%](https://console.algora.io/challenges/prettier)**
を達成しています。

**Biome は _JavaScript_, _TypeScript_, _JSX_ のため
の[高性能なLinter](https://github.com/biomejs/biome/tree/main/benchmark#linting)**
であり、ESLint, typescript-eslint,
[その他のソース](https://github.com/biomejs/biome/discussions/3)から
**[270以上のルール](https://biomejs.dev/linter/rules/)**を提供しています。Biome
は**詳細で文脈に沿った結果を出力**するため、コードを改善し、より良いプログラマに
なるための手助けをします！

**Biome** は最初か
ら[**エディタ内で対話的に**](https://biomejs.dev/ja/guides/integrate-in-editor/)使
用できるように設計されています。あなたがコードを書いているときに、形の崩れたコー
ドを format と lint することができます。

### インストール

```shell
npm install --save-dev --save-exact @biomejs/biome
```

### 使い方

```shell
# ファイルをformatする
npx @biomejs/biome format --write ./src

# ファイルをlintする
npx @biomejs/biome lint ./src

# format、lintなどを実行し、安全な提案を適用する
npx @biomejs/biome check --write ./src

# CI環境では、すべてのファイルを対象にformatやlintをチェックする
npx @biomejs/biome ci ./src
```

Biome をインストールせずに試したい場合は、WebAssembly にコンパイルされ
た[オンラインのプレイグラウンド](https://biomejs.dev/playground/)を利用してくだ
さい。

## ドキュメント

Biome についてもっと知るために[ホームページ][biomejs]をチェックするか、Biome を
使い始めるために[はじめる][getting-started]に進んでください。

## Biome をもっと詳しく

**Biome** は理にかなったデフォルト設定を持ち、設定を必要としません。

**Biome** はモダンなウェブ開発における[全ての主要な言語][language-support]をサ
ポートすることを目指しています。

**Biome** は動作するために Node.js を必要としません。

**Biome** はソーステキストの完全な表現力とエラー回復能力を持つ洗練されたParserに
よって、優れたLSPサポートを提供します。

**Biome** は以前は別々のツールで提供されていた機能を統合します。共通基盤を構築す
ることで、コードの処理、エラーの表示、並列処理、キャッシュ、設定について統一的な
体験を提供します。

興味のある方は、[プロジェクトの理念][biome-philosophy]もご覧ください。

**Biome** は
[MIT ライセンス](https://github.com/biomejs/biome/tree/main/LICENSE-MIT)または
[Apache 2.0 ライセンス](https://github.com/biomejs/biome/tree/main/LICENSE-APACHE)で
あ
り、[コントリビューター行動規範](https://github.com/biomejs/biome/tree/main/CODE_OF_CONDUCT.md)の
下で管理されています。

## スポンサー

### ゴールドスポンサー

### ブロンズスポンサー

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://www.kanamekey.com" target="_blank"><img src="https://images.opencollective.com/kaname/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80"></a>
      </td>
    </tr>
  </tbody>
</table>

[biomejs]: https://biomejs.dev/ja/
[biome-philosophy]: https://biomejs.dev/ja/internals/philosophy/
[language-support]: https://biomejs.dev/ja/internals/language-support/
[getting-started]: https://biomejs.dev/ja/guides/getting-started/
