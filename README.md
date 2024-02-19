# cpuex2-simulator
Simulator for CPUEX-Group2 computer
```
Usage: simulator [OPTIONS]

Options:
  -b, --bin <BIN>            Name of the input binary file [default: main.bin]
  -s, --sld <SLD>            Name of sld file for raytracing [default: ./sld/contest.sld]
  -v, --verbose <VERBOSE>    Verbose mode. If this flag is not set, the simulator won't print anything in each cycle. If this flag is set to 1, the simulator will print the information about only pipeline in each cycle. If this flag is set to 2, the simulator will print the information about pipeline and registers in each cycle, and save history of registers and pc
  -t, --test-fpu <TEST_FPU>  Operation name for test of FPU (fadd, fsub, fmul, fdiv, fsqrt, flt, fcvtsw, fcvtws, or all)
  -d, --disassemble          Disassemble mode. If this flag is set, the simulator will print the disassembled instructions. The output file name is the same as the input binary file name, but the extension is changed to ".dasm"
  -h, --help                 Print help
  -V, --version              Print version
```

## 詳細
Rust により実装した、2班のサイクルレベルのシミュレータです。コアと同様に、IF、IF2、ID、EX、MEM, WB の6つのステージからなります。また、FPU やキャッシュのエミュレートも実機と同様のアルゴリズムで行っています。ただし、FPU での演算におけるストールや、キャッシュミス時のストールについては、シミュレーションの必要はないと判断し省略しています。
なお実行時間予測については命令レベルのシミュレータにより行っています。

### 動作環境
Cargo を用いてビルド・実行をすることが前提になっています。WSL2 で動作確認をしています。なお用いた Rust のバージョンおよび Cargo のバージョンは
```sh
$ rustc --version
rustc 1.70.0 (90c541806 2023-05-31)
$ cargo --version
cargo 1.70.0 (ec8a8a0ca 2023-04-25)
```
です。

### 実行方法
実行時のオプションの説明は次のようになります(`make help` でも確認できます)。
```
Simulator for CPUEX-Group2 computer

Usage: simulator [OPTIONS]

Options:
  -b, --bin <BIN>            Name of the input binary file [default: main.bin]
  -s, --sld <SLD>            Name of sld file for raytracing [default: ./sld/contest.sld]
  -v, --verbose <VERBOSE>    Verbose mode. If this flag is not set, the simulator won't print anything in each cycle. If this flag is set to 1, the simulator will print the information about only pipeline in each cycle. If this flag is set to 2, the simulator will print the information about pipeline and registers in each cycle, and save history of registers and pc
  -t, --test-fpu <TEST_FPU>  Operation name for test of FPU (fadd, fsub, fmul, fdiv, fsqrt, flt, fcvtsw, fcvtws, or all)
  -d, --disassemble          Disassemble mode. If this flag is set, the simulator will print the disassembled instructions. The output file name is the same as the input binary file name, but the extension is changed to ".dasm"
  -h, --help                 Print help
  -V, --version              Print version
```
例えば、`minrt.bin` というバイナリファイルと `./sld/contest.sld` というファイルを入力として与え、かつ逆アセンブルの結果を得たいときは、次のように実行します。
```sh
$ cargo run --release -- --bin minrt.bin -s ./sld/contest.sld --disassemble
```
`cargo` コマンドの `--release` オプションは必須ではないですが、256x256 のレイトレをシミュレーションする際などにはある程度時間がかかることが予想されるので、このオプションを付けることを推奨します。また、主にDDR2メモリをシミュレートする都合上、実行時のメモリ消費量は大きくなります。そのため、実行時には
```sh
$ ulimit -s unlimited
```
などとして使用できるスタックサイズを増やしておかないと、スタックオーバーフローが発生する可能性があります。

#### プログラムからの出力
プログラムからの出力先のファイル名として、入力のバイナリファイルの名前の `.bin` を `.ppm` に置き換えたものが使われます。`outchar` や `outint` 命令が実行されるたびに、対応するレジスタの値がこのファイルに出力されていきます。

#### `--verbose` オプションについて　
`--verbose` オプションは、シミュレーションの途中経過を毎サイクルで表示するためのオプションです。次のような値を指定することができます。
- `1` を指定する場合、パイプラインの状態(各ステージの命令やPC)が表示されます。
- `2` を指定する場合、パイプラインの状態とレジスタの状態が表示されます。また、PCやレジスタの値の履歴が保存され、実行終了後にタイムラインのような形式で表示されます。

なお、`--verbose` を指定するかどうかによらず、10000000 サイクルごとに標準エラー出力にサイクル数、出力バイト数、PC、SPの値が表示されます。

#### `--test-fpu` オプションについて
`--test-fpu` オプションは、FPU のエミュレータと実機との比較を行うためのものです。これをセットして実行すると、シミュレーションは実行されず、FPU のエミュレータと実機との比較だけを行います。所定の場所に実機での結果を記述したファイルを置く必要があります。

#### 逆アセンブルについて
逆アセンブルでは、PCの値とそのPCにおける命令バイナリをアセンブリコード風に変換したものを各行に書いたファイルが生成されます。このファイルの名前は、入力のバイナリファイル名に対して拡張子を `.dasm` に変更したものになります。なお、元のアセンブリコードでラベルが使われている部分は、シミュレーション時にラベル名を知ることができないので、代わりにジャンプのオフセット等を表示するようにしています。

### 統計情報
シミュレーションの実行が終了するとさまざまな統計情報が標準出力に出力されます。それらは以下の通りです。
- 実行命令数
- シミュレーションの実行時間
- シミュレーションの実行速度(MIPS)
- PCおよびレジスタの値の履歴
- メモリアクセス数
- キャッシュヒット数
- キャッシュヒット率
- 命令メモリへのアクセス数
- プログラムからの出力
- 各命令の実行数(降順にソート)
- 各PCの到達回数(降順にソート)

なおこれらの一部は実行時のオプションによっては出力されないことがあります。

### 各ソースファイルの説明
`src` ディレクトリにある各ファイルの説明です。
- `main.rs` はエントリーポイントになります。コマンドライン引数をパースして、`core` に必要な引数を渡してシミュレータを実行します。
- `core.rs` はコア本体です。以下で述べるようないくつかのモジュールを使って、命令の実行、メモリの読み書き、キャッシュのシミュレーション、統計情報の収集などを行います。
- `decoder.rs` は命令のデコーダーです。なお、このシミュレータでは実行前にあらかじめすべての命令をデコードしておくことで高速化を図っています。
- `fpu_emulator.rs` は FPU のエミュレータです。FPU の命令を実行するために使います。また、FPU が要件を満たしているかをチェックするためのテストコードも含まれています。
- `fpu_tester.rs` は、FPUの実機との比較を行うためのテストコードです。
- `instruction_memory.rs` は命令メモリのエミュレータです。命令の読み込みおよび読み出しを行います。
- `instruction.rs` は命令のエミュレータです。命令の実行を行います。
- `memory.rs` はメモリのエミュレータです。メモリの読み書きを行います。メモリのサイズはDDR2メモリのサイズに合わせてありますが、容易に変更することができます。
- `pseudo_lru_cache.rs` は擬似LRUセットアソシアティブキャッシュのエミュレータです。キャッシュのシミュレーションを行います。キャッシュサイズやウェイ数、ラインサイズなどを容易に変更することができるようになっています。
- `cache.rs` はLRUセットアソシアティブキャッシュのエミュレータです。実際に用いているのは疑似LRUなので、このモジュールは使っていません。
- `register.rs` はレジスタのエミュレータです。レジスタの読み書きを行います。
- `sld_loader.rs` はレイトレのための sld ファイルを読み込むためのモジュールです。
- `types.rs` はシミュレータ全体で使う型を定義しています。
- `utils.rs` はシミュレータ全体で使うユーティリティ関数を定義しています。

### 使用した外部クレート
- rand = 0.8.4 (FPUのテスト時の乱数生成)
- float_next_after = 0.1.1 (FPUテスト)
- clap = 4.4.8 (コマンドライン引数のパース)
- linked-hash-map = 0.5.6 (LRUキャッシュの実装)
- fxhash = 0.2.1 (高速なハッシュマップ)
