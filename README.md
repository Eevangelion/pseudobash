# Pseudobash

**`Pseudobash`** — это легковесная псевдо-оболочка, написанная на Rust, предназначенная для Linux-систем. Проект сочетает в себе мощь Rust с простотой и удобством традиционных Unix-оболочек.

## Особенности

- **Высокая производительность** благодаря Rust
- **Стандартные команды** (`cat`, `echo`, `wc`, `pwd`, `exit`, `grep`, `cd`, `ls`)
- **Поддержка внешних команд** через `PATH`
- **Поддержка seq** **`;` и pipe `|`**
- **Минимальное количество зависимостей**: `std`, `anyhow`
- **Работа с окружением**
- **Документируемая архитектура** в `docs`

## Требования

- **Операционная система**: Linux (Ubuntu, Debian, CentOS, etc.)
- **Rust**: версия 1.70.0 или выше
- **Cargo**: система сборки Rust

## Полная установка

### Шаг 1: Установка Rust (если не установлен)

Установите Rust с официального сайта: [https://www.rust-lang.org/](https://www.rust-lang.org/ "Официальный сайт")

### Шаг 2: Склонируйте репозиторий

```bash
git clone https://github.com/Dx-by-Dy/pseudobash.git
```

### Шаг 3: Перейдите в скачаный репозиторий и соберите все исполняемые файлы

```bash
cd ./pseudobash
```

Соберите все дополнительные программы:

```bash
cd utils && find . -name "Cargo.toml" -exec dirname {} \; | xargs -I {} sh -c 'cd {} && cargo build --profile bin --target-dir ../'; cd ../
```

Соберите `pseudobash`:

```bash
cargo build --profile bin --target-dir .
```

### Шаг 4: Запустите `pseudobash`:

```bash
./bin/pseudobash
```

Вы увидите приглашение ввода:

```bash
Welcome to Pseudobash v2.2.2!

>>>
```

## Тестирование

### Запустите тестирование:

```bash
cargo test -r
```

Вы должны увидеть, что все тесты пройдены успешно

## Пример использования

```bash
>>> cat ./Cargo.toml
[package]
name = "pseudobash"
version = "2.2.2"
edition = "2024"

[dependencies]
anyhow = "1.0.99"
>>> 
```

```bash
>>> cat ./Cargo.toml | wc
7 14 99
>>>  
```

```bash
>>> pwd | wc
1 1 22
>>>
```

```bash
>>> x=ec y=ho 
>>> $x$y 100
100
>>> 
```

```bash
>>> echo 100; echo 200 | cat
100
200
>>> 
```

```bash
>>> grep -A 2 "grep" ./utils/grep/src/main.rs
    grep::{
        args::Args,
        grep::{Grep, matcher::GrepMatcher},
    },
};
        Ok(mut grep) => grep.run(),
        Err(e) => {
            eprintln!("{}", e);
>>>
```

## Планы развития

* **Перенаправление ввода/вывода** (`>`, `<`, `>>`)
* **Работа с терминалом в raw-режиме**
* **Конфигурационный файл** (`~/.pseudobashrc`)
* **Поддержка скриптов**
