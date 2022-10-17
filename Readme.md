
# Roadmap (✔ ✖)

- ✔ Создать базовый плагин. 
- ✔ Поддержка Форматов Vst3, Clap.   
- ✔ Создать параметры.
- ✔ Создать Gain (Мастер громкость).
- ✔ Создать Bypass.

# Настройка нового проекта. 

1. Создать новый проект `plugin-rs` как библиотеку.
2. Скопировать cargo.toml (зависимости и пространство имён).
3. Скопировать папку xtask(система сборки) .cargo(полная команда компиляции).

## Подробная инструкция: 

1. Создаем проект `name` плагина как библиотеку:
```
cargo new --lib name 
cd name
```

2. Подключаем `name\Cargo.toml`:
```toml
[dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }

[lib]
crate-type = ["cdylib"]

[workspace]
members = ["xtask"]
```

3. Создаем папку `name/.cargo/config`
```
[alias]
xtask = "run --package xtask --release --"
```

> Мы изменили команду для компиляции проекта, теперь вместо `cargo run --package xtask --release -- bundle plugin --release` мы будем использовать `cargo xtask bundle plugin --release`


4. Создаем 2-й проект в этом проекте для подключения [системы сборки](https://github.com/robbert-vdh/nih-plug/tree/master/nih_plug_xtask) 

```
cargo new --bin xtask
```

`name/xtask/src/main.rs`:
```rust
fn main() -> nih_plug_xtask::Result<()> {
    nih_plug_xtask::main()
}
```

`name/xtask\Cargo.toml`:
```toml
[dependencies]
nih_plug_xtask = { git = "https://github.com/robbert-vdh/nih-plug" }
```
---