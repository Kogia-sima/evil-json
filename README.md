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
                        time:   [1.6274 ms 1.6318 ms 1.6365 ms]
                        thrpt:  [291.54 MiB/s 292.40 MiB/s 293.18 MiB/s]
citm_catalog.json/simd-json
                        time:   [1.2735 ms 1.2778 ms 1.2830 ms]
                        thrpt:  [371.44 MiB/s 372.94 MiB/s 374.22 MiB/s]
citm_catalog.json/evil-json
                        time:   [373.71 us 374.81 us 376.17 us]
                        thrpt:  [1.2386 GiB/s 1.2431 GiB/s 1.2468 GiB/s]

twitter.json/serde-json
                        time:   [1.1918 ms 1.1955 ms 1.1998 ms]
                        thrpt:  [379.71 MiB/s 381.06 MiB/s 382.25 MiB/s]
twitter.json/simd-json
                        time:   [955.47 us 959.51 us 963.87 us]
                        thrpt:  [472.67 MiB/s 474.82 MiB/s 476.82 MiB/s]
twitter.json/evil-json
                        time:   [411.54 us 413.36 us 415.51 us]
                        thrpt:  [1.0707 GiB/s 1.0763 GiB/s 1.0811 GiB/s]
```

## Author

👤 **Ryohei Machida**

* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 📝 License

This project is distributed under the [MIT-0](./LICENSE) License.
