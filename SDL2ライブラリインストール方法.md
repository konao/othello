# SDL2ライブラリインストール方法

SDL2を使用したコードのビルドには、SDL2のlibファイルが必要である．

## 方法

2つのライブラリ(SDL2, SDL2_Image, SDL2_ttf)のlibファイルを、Rust(rustup)の管理下のlibディレクトリへコピーする．

[コピー元]
+ `https://github.com/libsdl-org/SDL/releases/tag/release-2.30.3/SDL2-devel-2.30.3-VC.zip`
+ `https://github.com/libsdl-org/SDL_image/releases/SDL2_image-devel-2.8.2-VC.zip`
+ `https://github.com/libsdl-org/SDL_ttf/releases/SDL2_ttf-devel-2.22.0-VC.zip`

上記のzipを適当な場所に展開後、lib/x64ディレクトリにあるファイル群を以下にコピーする．

[コピー先]
`C:\Users\sekikawa\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`

コピー完了後、cargo buildでビルドできるはず
