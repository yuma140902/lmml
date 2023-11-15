# LMML

LMML (**L**MML **M**usic **M**acro **L**anguage) は MML の方言です。

## LMMLの文法

```
<lmml>       := <operator>*
<operator>   := <note>
              | <rest>
              | <set-octave>
              | <set-length>
              | <set-volume>
              | <set-tempo>
              | <inc-octave>
              | <dec-octave>
<note>       := <note-char> <modifier>? <length>?
<note-char>  := 'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B'
              | 'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b'
<modifier>   := '+' | '-'
<rest>       := 'R' <length>? | 'r' <length>?
<length>     := <number> <dot>?
<dot>        := '.'
<set-ocatve> := 'O' <number> | 'o' <number>
<set-length> := 'L' <length> | 'l' <length>
<set-volume> := 'V' <number> | 'v' <number>
<set-tempo>  := 'T' <number> | 't' <number>
<inc-octave> := '>'
<dec-octave> := '<'
<number>     := <digit>+
<digit>      := '0' | '1' | '2' | '3' | '4'
              | '5' | '6' | '7' | '8' | '9'
```

基本的に大文字小文字を区別しません。また、空白や改行は無視されます。

`;`から始まる行はコメントとして無視されます。

## LMMLの意味論

LMMLにはいくつかのコマンドが存在します。それらのコマンドを並べたものがLMMLのプログラムです。

### 音符コマンド

`C`～`B`の文字はそれぞれドからシの音を表します。`+`をつけると半音上がり、`-`をつけると半音下がります。

音符コマンドの後には音の長さを表す数字をつけることができます。四分音符なら4、八分音符なら8のように指定します。
数字が大きいほど音の長さは短いことに注意してください。

数字が省略された場合は、直近の`L`コマンドの値が使用されます。`L`コマンドが存在しないときは4が使用されます。

さらに、数字の後に`.`をつけることができます。`.`は付点音符を表します。

#### 例

- `C4` - ドの四分音符
- `C+8` - ド#の八分音符
- `C4.` - ドの付点四分音符
- `C+4.` - ド#の付点四分音符

### 休符コマンド

`R`は休符を表します。音符コマンドと同様に長さを表す数字と`.`をつけることができます。

### `L`コマンド

音符の長さをセットします。音符コマンドの後に数字をつけなかった場合はこのコマンドの値が使用されます。詳細は音符コマンドの節を参照してください。

### `O`コマンド

オクターブの値をセットします。オクターブの初期値は4で、値が大きいほど音が高くなります。オクターブ4のドは440Hzです。

### `>`コマンド

オクターブの値を1増やします。

### `<`コマンド

オクターブの値を1減らします。

### `V`コマンド

音の大きさをセットします。具体的な仕様は未定ですが、初期値が10で値が大きいほど音が大きくなります。

### `T`コマンド

テンポをセットします。値は1分間に四分音符が鳴る回数を表します。初期値は120です。

## `lmml`クレート

LMMLのASTやタイムラインを表すデータ構造を含むクレートです。

## `lmml-parser`クレート

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

## `lmml-cli`クレート

LMMLを演奏したり他の形式に変換するためのコマンドラインツールです。

### インストール方法

```sh
cargo install lmml-cli
```

### 使用方法

```sh
lmml --help
```
