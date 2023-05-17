# すべかぜ見聞暗号生成/解読ツール
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FNSK-1010%2Fkembun-cipher.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FNSK-1010%2Fkembun-cipher?ref=badge_shield)

すべかぜの表裏枠である見聞の1番間奏の暗号を生成/解読するツール

## 使い方
```sh
cargo run -- [オプション] [引数]
```

## オプション
|ショート|ロング|引数|
|---|---|---|
|`-e`|`--encode`|平文|
|`-d`|`--decode`|解読用テキストファイルパス|

## 解読用テキストファイルについて
このような構造になっています。
```
生成時の1回目のレールフェンスのレール数
生成時の2回目のレールフェンスのレール数
暗号文(スペース区切り)
```


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FNSK-1010%2Fkembun-cipher.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FNSK-1010%2Fkembun-cipher?ref=badge_large)