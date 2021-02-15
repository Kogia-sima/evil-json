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
                        time:   [1.4925 ms 1.4964 ms 1.5008 ms]
                        thrpt:  [1.0719 GiB/s 1.0750 GiB/s 1.0778 GiB/s]
citm_catalog.json/simd-json
                        time:   [1.3359 ms 1.3400 ms 1.3448 ms]
                        thrpt:  [1.1962 GiB/s 1.2004 GiB/s 1.2041 GiB/s]
citm_catalog.json/evil-json
                        time:   [675.27 us 677.53 us 680.12 us]
                        thrpt:  [2.3651 GiB/s 2.3742 GiB/s 2.3821 GiB/s]

twitter.json/serde-json time:   [1.0604 ms 1.0661 ms 1.0726 ms]
                        thrpt:  [561.51 MiB/s 564.94 MiB/s 567.97 MiB/s]
twitter.json/simd-json  time:   [959.06 us 963.76 us 968.97 us]
                        thrpt:  [621.55 MiB/s 624.91 MiB/s 627.97 MiB/s]
twitter.json/evil-json  time:   [427.86 us 429.98 us 432.41 us]
                        thrpt:  [1.3602 GiB/s 1.3678 GiB/s 1.3746 GiB/s]
```

## Author

👤 **Ryohei Machida**

* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 📝 License

This project is distributed under the [MIT-0](./LICENSE) License.
