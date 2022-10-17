/* # Компиляция плагина Vst3 и Clap.

    Компиляция проекта `plugin`(команда терминала):
    ```
    cargo xtask bundle plugin --release
    ```
*/

/* # Упаковщик в формат плагинов.

    В проекте должен быть настроен упаковщик в формат плагинов, xtask.

    NIH-plug поставляется с упаковщиком, который создает для вас пакеты плагинов 
    на основе экспортируемых форматов плагинов, а также операционной системы 
    и архитектуры, для которой вы компилируете. 
    Ознакомьтесь с (readme)[https://github.com/robbert-vdh/nih-plug/tree/master/nih_plug_xtask]
    для получения инструкций по его использованию в вашем собственном проекте.

*/

/* # Создание базового плагина:

    Мы должный создать реализацию плагина Plugin + Default, 
    и формата плагина Vst3 и Clap. Предать макросу.  
    `nih_export_clap!(_)`,`nih_export_vst3!(_)`

    > (Документация nih-plug)[https://nih-plug.robbertvanderhelm.nl/nih_plug/]
*/

//Плагин
mod plugin;

// Формат плагина Vst3 и Clap
mod vst3;
mod clap;

//Параметры 
mod parameters;

//Графический интерфейс
//mod editor;