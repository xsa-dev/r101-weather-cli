<div align="center">
  <h1><code>wf</code> (weather forecast)</h1>
  <div>
    <img src="https://img.shields.io/badge/status-going project-3E3E3E?style=float&color=3cbf50"/>
    &nbsp;
    <a href="https://stepik.org/a/184782">
        <img src="https://img.shields.io/badge/Курс-Hands on основы Rust-3E3E3E?style=float&color=e6963c"/>
    </a>
    <br />
    <br />
  </div>
</div>

`wf` - утилита, которая выводит прогноз погоды в терминал, используя [API Яндекс.Погоды](https://yandex.ru/dev/weather/doc/dg/concepts/forecast-info.html).

Запустить:

```bash
$ cargo run -q -- -f
```

> [!WARNING]
> Не забудьте переименовать `.env.example` в `.env` или, в случае кастомного названия файла, использовать:
>
> ```rust
> dotenvy::from_filename(".env.dev")?;
> ```

## Использование

```bash
$ cargo build --release

# теперь переместим нашу утилиту в какую-нибудь дирректорию,
# которая находится в $PATH
# (например, подойдет `/usr/local/bin`)
sudo cp ./target/release/wf /usr/local/bin

# перезапустим терминал - и можно запускать ;)
export YANDEX_WEATHER_API_KEY=<YOUR_KEY>
wf -f
```

## Задание

Добавьте к флагу `-f` количество дней, на которое будет даваться прогноз погоды.

```bash
cargo run -q -- -f 3
```

> [!NOTE]  
> Тут стоит сказать, что у "API Яндекс.Погоды" [тариф](https://yandex.ru/dev/weather/doc/dg/concepts/pricing.html) "Погода на вашем сайте" дает прогноз только на 2 (!) последующих периода. Но вы можете использовать тариф "Тестовый", который будет 30 первых дней бесплатным и давать прогноз уже на 7 дней.
>
> Как альтернативу можно использовать OpenWeatherMap (смотри `public-apis#weather` [GitHub repo](https://github.com/public-apis/public-apis#weather))

_(опционально)_ Улучшите вывод прогноза погоды, например, добавив другие поля или изменив цвет определенных элементов для улучшения восприятия.
