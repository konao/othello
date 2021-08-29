// mod内で使う他のクレートがあるときはここに書く
// （各modのソース内には書かない）
// extern crate sdl2;
// extern crate rand;

pub mod board;

// （注）
// lib.rsを使うのに、cargo new --lib XXXでライブラリを生成する必要はない．
// 通常のアプリ同様、cargo new XXXでバイナリパッケージを生成し、
// 単にlib.rsファイルを追加するだけでよい
// （ただモジュール参照のルールが良く分かってないので、色々試さないとなかなかうまく参照してくれない・・）