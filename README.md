# LMML

[![](https://img.shields.io/crates/v/lmml?color=blue)](https://crates.io/crates/lmml)

LMML (**L**MML **M**usic **M**acro **L**anguage) はMMLの方言です。

## 演奏例

https://github.com/yuma140902/lmml/assets/23431077/dfad8777-ade5-4591-8804-a3968a2e14ea

```
; Bad Apple!! feat. nomico

t320 l4

@0 v15
<ab>cd e.r8ag e.r8<a.r8> edc<b>
<ab>cd e.r8dc <bab>c <bagb>
<ab>cd e.r8ag e.r8<a.r8> edc<b>
<ab>cd e.r8dc <b.r8>c.r8 d.r8e.r8

@1 v10
<ab>cd e.r8ag e.r8<a.r8> edc<b>
<ab>cd e.r8dc <bab>c <bagb>
<ab>cd e.r8ag e.r8<a.r8> edc<b>
<ab>cd e.r8dc <b.r8>c.r8 d.r8e.r8

@3 v25
gaed e.r8de gaed e.r8
dedc <bga.r8 gab>c de<a.r8>
egga ede.r8 dega ede.r8
dedc <bga.r8 gab>c de<a.r8>

@4 v30
egga ede.r8 dega ede.r8
dedc <bga.r8 gab>c de<a.r8>
egga ede.r8 dega ede.r8
ab>c<b age.r8 dedc <bga.r8>

@0 v15
egga ede.r8 dega ede.r8
dedc <bga.r8 gab>c de<a.r8>
egga ede.r8 dega ede.r8
dedc <bga.r8 gab>c de<a.r8>
egga ede.r8 dega ede.r8
dedc <bga.r8 gab>c de<a.r8>
egga ede.r8 dega ede.r8
ab>c<b age.r8 dedc <bga.r8
```

## LMML言語の文法

```
<lmml>       := <command>*
<command>    := <note-cmd>
              | <chord>
              | <n-cmd>
              | <rest-cmd>
              | <set-octave>
              | <set-length>
              | <set-volume>
              | <set-tempo>
              | <set-wave>
              | <inc-octave>
              | <dec-octave>
<note-cmd>   := <note-char> <modifier>? <number>? <dot>?
<note-char>  := 'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B'
              | 'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b'
<modifier>   := '+' | '-'
<rest-cmd>   := 'R' <number>? <dot>? | 'r' <number>? <dot>?
<chord>      := '[' <note-char>+ ']' <number>? <dot>?
<dot>        := '.'
<n-cmd>      := 'N' <number> | 'n' <number>
<set-ocatve> := 'O' <number> | 'o' <number>
<set-length> := 'L' <number> <dot>? | 'l' <number> <dot>?
<set-volume> := 'V' <number> | 'v' <number>
<set-tempo>  := 'T' <number> | 't' <number>
<set-wave>   := '@' <number>
<inc-octave> := '>'
<dec-octave> := '<'
<number>     := <digit>+
<digit>      := '0' | '1' | '2' | '3' | '4'
              | '5' | '6' | '7' | '8' | '9'
```

基本的に大文字小文字を区別しません。また、空白や改行は無視されます。

`;`から始まる行はコメントとして無視されます。

## LMMLチュートリアル

LMMLにはいくつかのコマンドが存在します。それらのコマンドを並べたものがLMMLのプログラムです。

### 音符コマンド

`C`～`B`の文字はそれぞれドからシの音を表します。`+`をつけると半音上がり、`-`をつけると半音下がります。

音符コマンドの後には音の長さを表す数字をつけることができます。四分音符なら4、八分音符なら8のように指定します。
数字が大きいほど音の長さは短いことに注意してください。

数字が省略された場合は、その時点で最後に実行された`L`コマンドの値が使用されます。`L`コマンドが存在しないときは4が使用されます。

さらに、数字の後に`.`をつけることができます。`.`は付点音符を表します。

#### 例

- `C4` - ドの四分音符
- `C+8` - ド#の八分音符
- `C4.` - ドの付点四分音符
- `C+4.` - ド#の付点四分音符

### 休符コマンド

`R`は休符を表します。音符コマンドと同様に長さを表す数字と`.`をつけることができます。

### 和音

`[`と`]`で音符を囲むことにより和音を表すことができます。音符はルート音を先頭に、低い順に書いてください

#### 例

- `[ceg]` - C Major
- `[ace]` - Am
- `[ga+df]` - Gm7

### `L`コマンド

音符の長さをセットします。音符コマンドの後に数字をつけなかった場合はこのコマンドの値が使用されます。詳細は下の「LMML言語の細かい仕様」を参照してください。

### `O`コマンド

オクターブの値をセットします。オクターブの初期値は4で、値が大きいほど音が高くなります。オクターブ4のAは440Hzです。

### `>`コマンド

オクターブの値を1増やします。

### `<`コマンド

オクターブの値を1減らします。

### `N`コマンド

MIDIのノート番号(0～127)によって音符を指定します。

※LMMLはMIDIに依存していませんが、内部的にMIDI互換のフォーマットで音符を扱っています。

### `V`コマンド

音の大きさをセットします。初期値は20で、大きいほど音が大きくなります。100が0dB、0が-∞ dBに対応します。

### `T`コマンド

テンポをセットします。値は1分間に四分音符が鳴る回数を表します。初期値は120です。

### `@`コマンド

0～4の数字で波形を設定します。デフォルト値は0です。

| 数字 | 波形       |
|------|------------|
| 0    | ノコギリ波 |
| 1    | 矩形波     |
| 2    | パルス波   |
| 3    | 三角波     |
| 4    | 正弦波     |

## LMML言語の細かい仕様

### 長さの指定について

音符コマンド、休符コマンド、`L`コマンドで数字や`.`を省略した場合の挙動は表のようになります。予想する挙動が人によって異なると思われる部分は太字で示してあります。

| 最後の`L`コマンドの数字 | `.`有無 | 音符・休符コマンドの数字 | `.`有無 | 長さ            |
|-------------------------|---------|--------------------------|---------|-----------------|
| `L`コマンド無し         | N/A     | 省略                     | 無      | 四分音符        |
| `L`コマンド無し         | N/A     | 省略                     | 有      | 付点四分音符    |
| `L`コマンド無し         | N/A     | n                        | 無      | n分音符         |
| `L`コマンド無し         | N/A     | n                        | 有      | 付点n分音符     |
| m                       | 無      | 省略                     | 無      | m分音符         |
| m                       | 無      | 省略                     | 有      | 付点m分音符     |
| m                       | 無      | n                        | 無      | n分音符         |
| m                       | 無      | n                        | 有      | 付点n分音符     |
| m                       | **有**  | 省略                     | **無**  | **付点m分音符** |
| m                       | 有      | 省略                     | 有      | 付点m分音符     |
| m                       | **有**  | n                        | **無**  | **n分音符**     |
| m                       | 有      | n                        | 有      | 付点n分音符     |

### 音の高さについて

音符コマンドを周波数に変換する処理は2段階で行われます。

まず以下のようにしてノート番号と呼ばれる内部形式に変換します。$`n`$は音符コマンドの種類、$`m`$はシャープ・フラットなどの指定、$`o`$は現在のオクターブです。

```
base = if n == C then 0
       if n == D then 2
       if n == E then 4
       if n == F then 5
       if n == G then 7
       if n == A then 9
       if n == B then 11

modifier = if m == '+' then  1
           if m == '-' then -1
           else              0

N = base + modifier + ((o + 1) * 12)
```

次にノート番号$`N`$を周波数$`f`$ [Hz]に変換します。

```math
f = 440 \times 2 ^ { \frac{N - 69}{12} }
```

### 音の減衰について

1つの音は鳴った瞬間に最も音量が大きく、徐々に小さくなっていきます。具体的には`V`コマンドで指定した音量を$`v_0`$、音の長さを$`T`$、音が鳴り始めてからの経過時間を$`t`$とすると、$`t`$における音量$`v(t)`$は
```math
v(t) = \frac{v_0 (T-t)}{T}
```
です。

減衰の仕方を変える方法は現在のところありません。

### 数値の限界について

#### オクターブの値

上限・下限ともにありませんが、-1から9程度の範囲で使用することを想定しています。
なお、`O`コマンドでは非負の値しか指定できないため、負のオクターブを指定したい場合は`<`コマンドを使用してください。

#### ボリュームの値

下限は0、上限はありません。ただし100が0dBに対応するため、それより大きい値を指定すると音割れが発生する場合があります。0のときは音が全く出ません。

#### `N`コマンドの値

下限は0、上限はありません。ただし0から127の範囲の値を想定しています。

#### テンポ

下限は1、上限はありません。

## LMML実装の細かい仕様

### 精度について

波形合成は44.1kHz、32bit-floatで行っています。

## 各クレート

### `lmml`クレート

LMMLのASTやタイムラインのデータ構造と波形合成処理を含むクレートです。

### `lmml-parser`クレート

LMMLのパーサーです。

```rust
use lmml::{LmmlAst, LmmlCommand, NoteChar, NoteModifier};
use lmml_parser::parse_lmml;

pub fn main() {
    let ast = parse_lmml("t80 c+ d e8. r8").unwrap();

    assert_eq!(ast, LmmlAst(vec![
        LmmlCommand::SetTempo(80),
        LmmlCommand::Note {
            note: NoteChar::C,
            modifier: NoteModifier::Sharp,
            length: None,
            is_tied: false,
        },
        LmmlCommand::Note {
            note: NoteChar::D,
            modifier: NoteModifier::Natural,
            length: None,
            is_tied: false,
        },
        LmmlCommand::Note {
            note: NoteChar::E,
            modifier: NoteModifier::Natural,
            length: Some(8),
            is_tied: true,
        },
        Lmmlcommand::Rest { 
            length: Some(8) 
            is_tied: false,
        },
    ]));
}
```

### `lmml-cli`クレート

LMMLを対話的に演奏したり他の形式に変換するためのコマンドラインツールです。

#### インストール方法

```sh
cargo install lmml-cli
```

#### 使用方法

ファイルを演奏する。

```sh
lmml load ファイル
```

対話モードを起動する。

```sh
lmml repl
```
