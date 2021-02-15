<div align="center">
  <h1>evil-json</h1>
  <p>Experimental JSON Serializer</p>
  <p>
    <img alt="Version" src="https://img.shields.io/badge/version-0.1.0--alpha.0-blue.svg?cacheSeconds=2592000" />
    <a href="https://github.com/Kogia-sima/evil-json/blob/master/LICENSE" target="_blank">
      <img alt="License: MIT-0" src="https://img.shields.io/badge/License-MIT--0-yellow.svg" />
    </a>
  </p>
</div>

:warning: This crate is just for experimentation. Do not use this in product.

## Performance

```console
citm_catalog.json/serde-json
                        time:   [1.7086 ms 1.7116 ms 1.7148 ms]
                        thrpt:  [960.57 MiB/s 962.37 MiB/s 964.05 MiB/s]
citm_catalog.json/simd-json
                        time:   [1.3119 ms 1.3164 ms 1.3216 ms]
                        thrpt:  [1.2172 GiB/s 1.2220 GiB/s 1.2262 GiB/s]
citm_catalog.json/evil-json
                        time:   [830.96 us 833.46 us 836.16 us]
                        thrpt:  [1.9238 GiB/s 1.9300 GiB/s 1.9358 GiB/s]

twitter.json/serde-json time:   [1.2553 ms 1.2600 ms 1.2655 ms]
                        thrpt:  [475.92 MiB/s 477.97 MiB/s 479.78 MiB/s]
twitter.json/simd-json  time:   [960.02 us 963.32 us 966.94 us]
                        thrpt:  [622.85 MiB/s 625.19 MiB/s 627.34 MiB/s]
twitter.json/evil-json  time:   [634.82 us 637.20 us 639.93 us]
                        thrpt:  [941.13 MiB/s 945.16 MiB/s 948.70 MiB/s]
```

## Author

👤 **Ryohei Machida**

* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 📝 License

This project is distributed under the [MIT-0](./LICENSE) License.
