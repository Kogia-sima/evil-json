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
                        time:   [1.4850 ms 1.4881 ms 1.4916 ms]
                        thrpt:  [319.86 MiB/s 320.62 MiB/s 321.29 MiB/s]
citm_catalog.json/simd-json
                        time:   [1.2758 ms 1.2790 ms 1.2828 ms]
                        thrpt:  [371.50 MiB/s 372.59 MiB/s 373.55 MiB/s]
citm_catalog.json/simd-json-derive
                        time:   [554.66 us 556.49 us 558.58 us]
                        thrpt:  [854.17 MiB/s 857.37 MiB/s 860.21 MiB/s]
citm_catalog.json/evil-json
                        time:   [366.31 us 367.94 us 369.93 us]
                        thrpt:  [1.2595 GiB/s 1.2664 GiB/s 1.2720 GiB/s]

twitter.json/serde-json
                        time:   [1.0518 ms 1.0556 ms 1.0602 ms]
                        thrpt:  [428.86 MiB/s 430.72 MiB/s 432.29 MiB/s]
twitter.json/simd-json
                        time:   [907.32 us 910.34 us 913.72 us]
                        thrpt:  [497.62 MiB/s 499.47 MiB/s 501.13 MiB/s]
twitter.json/simd-json-derive
                        time:   [846.42 us 849.35 us 852.67 us]
                        thrpt:  [534.29 MiB/s 536.38 MiB/s 538.24 MiB/s]
twitter.json/evil-json
                        time:   [404.72 us 407.50 us 410.75 us]
                        thrpt:  [1.0810 GiB/s 1.0896 GiB/s 1.0971 GiB/s]
```

## Author

👤 **Ryohei Machida**

* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 📝 License

This project is distributed under the [MIT-0](./LICENSE) License.
