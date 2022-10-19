# Создание плагина Vst3 и Clap с помощью NIH-plug.

[`NIH-plug`](https://nih-plug.robbertvanderhelm.nl/nih_plug/) - Это незавершенный API-независимый фреймворк аудио плагинов VST3 и CLAP, написанный на Rust.
> Незавершенный!!!. Некоторые функции или переменные могут измениться, функции будут принимать другое количество аргументов или переменные изменят тип. (Во время написания статьи уже происходили изменения).    

Нам будет необходимо, не только подключить библиотеку в cargo.toml, но настроить [систему сборки по инструкции.](https://github.com/robbert-vdh/nih-plug/tree/master/nih_plug_xtask). 

Если всё сделано правильно, то создать плагин можно будет командой терминала.

```rust 
cargo xtask bundle plugin-rs --release
```
> Компиляция проекта `plugin-rs`

## Настройка нового проекта. 

1. Создать новый проект `plugin` как библиотеку.
2. Скопировать cargo.toml (зависимости и пространство имён).
3. Скопировать папку xtask(система сборки) .cargo(полная команда компиляции).

### Упаковщик в формат плагинов.
(Это необходимо, пока Cargo не поддерживает запуск бинарники из зависимостей напрямую.)

В проекте должен быть настроен упаковщик в формат плагинов, xtask.

NIH-plug поставляется с упаковщиком, который создает для вас пакеты плагинов на основе экспортируемых форматов плагинов, а также операционной системы и архитектуры, для которой вы компилируете. 

Поддерживает: Vst3, Clap, Standalone 
---
### Подробная инструкция: 

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

# Создание базового плагина:

> [Документация nih-plug](https://nih-plug.robbertvanderhelm.nl/nih_plug/)

Для создания базового плагина, мы должный создать общую структуру для всего плагина. Для неё мы должный реализовать минимум 3 трейта `Plugin` + `Default` и трейт формата плагина `Vst3` или `Clap`. Данную структуру мы должны передать макросу `nih_export_clap!()`,`nih_export_vst3!()`, который и создаст нам плагин в нужном формате.

1. В папке src/lib.rs я создал несколько модулей в которых реализовал необходимые трейты. 
```rust
//Плагин
mod plugin;

// Формат плагина Vst3 и Clap
mod vst3;
mod clap;
```

2. Создал структуру App в модуле plugin, которая будет общей всего плагина. основную реализацию `Plugin` + `Default`. 
```rust
#[derive(Default)]
pub struct App {}

impl Plugin for App {
    const NAME: &'static str = "Test Plugin";
    const VENDOR: &'static str = "Prameon";
    const URL: &'static str = "https://www.youtube.com/c/prameon/";
    const EMAIL: &'static str = "_";
    const VERSION: &'static str = "0.0.1";
    /*Реализация по умолчанию*/
    }
```

Константы `NAME`, `VENDOR`, `URL`, `EMAIL`, `VERSION` - отвечают за информацию. Имя плагина, производитель, ссылка на сайт, почта, и версия плагина.

3. В модулях `vst3`, `clap` реализовал соответствующему формату трейт и макрос. 
```rust
use nih_plug::prelude::*;
use crate::plugin::App;

//Vst3 (+Plugin)
impl Vst3Plugin for App {
    const VST3_CLASS_ID: [u8; 16] = *b"Synth Sine Plug ";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth|Tools";
}
nih_export_vst3!(App);
```
```rust
use nih_plug::prelude::*;
use crate::plugin::App;

//Clap (+Plugin)
impl ClapPlugin for App {
    const CLAP_ID: &'static str = "com.prameon.synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Описание нашего плагина");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("https://www.youtube.com/c/prameon/");
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://www.youtube.com/c/prameon/");
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Stereo];
    const CLAP_POLY_MODULATION_CONFIG: Option<PolyModulationConfig> = None;
}
nih_export_clap!(App);
```

Реализация сводится к тому что необходимо заполнить константы. Это просто информация о плагине. 

## Результат.
Мы создали плагин который ничего не будет делать. Он будет просто отображаться в плагинах и этого должно быть достаточно, чтобы просто запустить плагин. 

> Чтобы правильно заполнить константы достаточно почитать документацию.
Использовать авто-заполнение трейта или посмотреть пример.

# Пропустить звук сквозь плагин.

На данный момент, я хочу создать плагин который пропускает через себя звуковой сигнал. Другими словами плагин, будет принимать сигнал из входного канала и передавать в выходной. Нам стоит убедится, что каналов больше нуля.`DEFAULT_INPUT_CHANNELS`, `DEFAULT_OUTPUT_CHANNELS` - создаст нужное количество входных и выходных каналов в буфере. `Buffer` передается в качестве аргумента в метод `fn process()`.

```rust
    //Проверка: По умолчанию имеют стерео входы и выходы 
    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;
```

Метод `fn process()`, это то место где происходит вся обработка сигнала, создание нового (подмена сигнала на новый). Получение миди-событий, сайдчейна, или в нашем случае буфера. 

Давайте рассмотрим на буфер более подробно. Этот тип содержит вектор с массивом f32, иными словами канал который хранит семпл. С помощью метода .iter_samples() мы сможем получить каждый канал, а у него и каждый семпл в нем. В итоге мы получим указатель на семпл выходного канала, при этом который уже содержит семпл из входного канала. В нашем случае, мы добились первоначальной цели. 
> т.е. мы можем получить значения через указатель и вернуть в туже переменную, как-то изменить или присвоить новое значение.
```rust
    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, _context: &mut impl ProcessContext) -> ProcessStatus {
        //!!! Каждый канал. (L, R). - channel
        for channel in buffer.iter_samples() {
            // !!!Каждый семпл буфера.
            for sample in channel{
                //Семпл
                *sample = *sample;
                // * - это указатель
            }
        }
        ProcessStatus::Normal
    }
```

# Громкости.
Наш плагин пропускает через себя сигнал, далее мы можем создать эффект который будет изменять уровень сингала Gain. Этот эффект очень прост, нужно представить что сигнал (*sample) который приходит в плагин - это 1.0, и уменьшение и увеличение этого числа это и есть наш эффект. 
```rust
*sample = *sample * 0,90; //уменьшил сигнал на 10%
*sample = *sample * 1,50; //увеличил сигнал на 50%
```

# Параметры: 
Чтобы управлять нашим эффектом мы создадим параметр. Параметр будет отображаться средствами хоста.

1. В папке src/lib.rs я создал модуль в котором реализовал необходимый трейт.
```rust
//Параметры 
mod parameters;
```

2. Новая структура Parameters, будет хранить все параметры в плагине. 
```rust
use nih_plug::prelude::*;

#[derive(Params)]
pub struct Parameters {
    #[id = "master_gain"]
    pub master_gain: FloatParam,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            //1
            master_gain: FloatParam::new(
                "Master Gain",
                -10.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 6.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(3.0))
            .with_step_size(0.01)
            .with_unit(" dB"),
        }
    }
}
```
Мы использовали тип FloatParam, который принимает имя параметра, его значение, и диапазон. а также дополнительные методы со сглаживанием, размером шага изменения параметра, и имя формата измерений.

3. В модуле plugin подключил созданные параметры через Arc, и заполнил fn params(), так как этого требовал трейт Plugin.

```rust
use std::sync::Arc;
use crate::parameters::Parameters;

pub struct App {
    //Parameters
    params: Arc<Parameters>,
}

impl Plugin for App {
    //----
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone() as Arc<dyn Params>
    }
```

4. В методе process, мы будем использовать созданный параметр для взаимодействия как эффект gain. Мы буде получать при каждой итерации значение из параметра и преобразовывать их в децибелы.  
```rust 
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        //!!! Каждый канал. (L, R). - channel
        for channel in buffer.iter_samples() {

            // !!!Каждый семпл буфера.
            for sample in channel{

                // Громкость параметр
                let master_gain = self.params.gain.smoothed.next();
                //effect gain
                *sample *= util::db_to_gain(master_gain); 
                
            }
        }
        ProcessStatus::Normal
    }
```

# Bypass.
1. По аналоги создадим еще один эффект Bypass, создает обходной путь чтобы получить неизмененный сигнал. имеет два состояния активный и неактивный. 
```rust
for sample in channel{
    if /*Параметр*/ {
        *sample = *sample; 
    } else {
        let master_gain = self.params.gain.smoothed.next();
        *sample *= util::db_to_gain(master_gain);
    }
}
```

2. Теперь создадим параметр, который будет делать тоже самое.
```rust
use nih_plug::prelude::*;

#[derive(Params)]
pub struct Parameters {
    // 1 Master Gain
    // 2 Bypass
    #[id = "bypass"]
    pub bypass: BoolParam,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            // 1 Master Gain
            
            // 2 Bypass
            bypass: BoolParam::new("Bypass", false),
        }
    }
}
```
Мы использовали тип BoolParam, который прижимает два аргумента имя параметра и его состояние (Не активный). 

3. В методе process, мы будем использовать созданный параметр для взаимодействия. Теперь стоит заменить комментарий параметр на self.params.bypass.value(), чтобы получить значение параметра.

