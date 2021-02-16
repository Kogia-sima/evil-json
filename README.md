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
                        time:   [1.6592 ms 1.6638 ms 1.6690 ms]
                        thrpt:  [285.87 MiB/s 286.77 MiB/s 287.55 MiB/s]
citm_catalog.json/simd-json
                        time:   [1.2783 ms 1.2811 ms 1.2843 ms]
                        thrpt:  [371.49 MiB/s 372.44 MiB/s 373.25 MiB/s]
citm_catalog.json/evil-json
                        time:   [672.48 us 674.80 us 677.61 us]
                        thrpt:  [704.13 MiB/s 707.06 MiB/s 709.50 MiB/s]

twitter.json/serde-json time:   [1.2504 ms 1.2548 ms 1.2597 ms]
                        thrpt:  [361.65 MiB/s 363.06 MiB/s 364.34 MiB/s]
twitter.json/simd-json  time:   [947.01 us 949.58 us 952.45 us]
                        thrpt:  [478.32 MiB/s 479.77 MiB/s 481.07 MiB/s]
twitter.json/evil-json  time:   [430.82 us 432.64 us 434.65 us]
                        thrpt:  [1.0236 GiB/s 1.0283 GiB/s 1.0327 GiB/s]
```

## Author

👤 **Ryohei Machida**

* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 📝 License

This project is distributed under the [MIT-0](./LICENSE) License.
