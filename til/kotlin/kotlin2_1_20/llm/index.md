# Обзор Kotlin 2.1.20: Главные обновления и фичи языка

> Подробный разбор ключевых улучшений компилятора K2, исправлений багов и новых возможностей языка Kotlin версии 2.1.20 для Android и Backend разработки.

- **URL:** http://localhost:1314/til/kotlin/kotlin2_1_20/
- **Date:** 2025-03-26

- **Tags:** Kotlin

---
На днях вышел очередной релиз Kotlin 2.1.20, который принес массу интересных обновлений. Давайте разберемся, что нового появилось в этой версии.

---

#### **1. Основные изменения**
**🔧 K2-компилятор и KAPT**  
- **KAPT по умолчанию на K2**: Новый плагин kapt по умолчанию – вроде бы мелочь, но для тех, кто возится с аннотациями и препроцессингом, это может быть существенно. Раньше приходилось возиться с кучей конфигураций, теперь обещают больше стабильности.

  *Откат на K1*: Добавьте в `gradle.properties`:  
  ```properties
  kapt.use.k2=false
  ```



**⚡ Kotlin/Native: Производительность**  
- **Инлайн-оптимизация** (экспериментально):  
Инлайн-оптимизации в Kotlin/Native – вещь, которую оценят только настоящие перфекционисты. Миллисекунды? Да. Принципиально? Абсолютно.

Эта оптимизация превосходит стандартный инлайнер LLVM и ускоряет выполнение программ.

  *Как включить:*

  ```bash
  -Xbinary=preCodegenInlineThreshold=40
  ```  
  - Значение 40 — оптимальный порог для баланса между оптимизацией и временем компиляции.
  - Можно экспериментировать с другими значениями (чем выше, тем агрессивнее оптимизация).

  *Результаты*: +9.5% к скорости выполнения (по тестам JetBrains).

**🌐 WebAssembly**  
- **Кастомные форматтеры по умолчанию** в dev-сборках.
Дефолтные кастомные форматтеры в Wasm – это уже интересно. Web-разработчики получают немного больше контроля без лишних плясок с бубном.

```kotlin
// build.gradle.kts
kotlin {
    wasmJs {
        // ...

        compilerOptions {
            freeCompilerArgs.add("-Xwasm-debugger-custom-formatters")
        }
    }
}
```

- **DWARF-отладка** (для Wasm VM):  
  ```bash
  -Xwasm-generate-dwarf
  ```  
  *Что даёт*: инспекция переменных, пошаговая отладка.

---

#### **2. Экспериментальные возможности**
**🛠 Поддержка Lombok @SuperBuilder и улучшения @Builder**  

- **Kotlin Lombok plugin** теперь поддерживает аннотацию @SuperBuilder, упрощая создание билдеров для иерархий классов. Раньше при наследовании приходилось вручную описывать логику для суперклассов — теперь это делается автоматически.

**Что изменилось:**

1. @SuperBuilder:
   - Автоматически наследует поля родительского класса.
   - Позволяет инициализировать поля суперкласса при создании объекта.
   
```kotlin
    @SuperBuilder
    public class Parent {
        private String parentField;
    }

    @SuperBuilder
    public class Child extends Parent {
        private String childField;
    }

    // Использование в Kotlin:
    val child = Child.builder()
        .parentField("value")
        .childField("anotherValue")
        .build()
```

2. **@Builder на конструкторах:**

- Теперь работает с конструкторами, предлагая гибкие варианты создания объектов.

```kotlin
public class User {
    @Builder
    public User(String name, int age) { ... }
}
```

**📦 Новый DSL для Multiplatform**  
С Gradle 8.7 старый Application plugin несовместим с Kotlin Multiplatform. Появился новый экспериментальный DSL с блоком executable {}:

```kotlin
kotlin {
    jvm {
        @OptIn(ExperimentalKotlinGradlePluginApi::class)
        binaries {
            executable {
                mainClass.set("foo.MainKt")
            }
            
            // Можно настроить несколько точек входа
            executable(KotlinCompilation.MAIN_COMPILATION_NAME, "another") {
                mainClass.set("foo.MainAnotherKt")
            }
        }
    }
}
```

**Важно:** Требуется `@OptIn(ExperimentalKotlinGradlePluginApi::class)`


**📤 Кастомные публикации в Gradle**  
Добавление своих вариантов публикации для JVM и Multiplatform (экспериментально):  
Возможность добавлять кастомные варианты публикации. Звучит скучно? Для кого-то – да. Для того, кто хочет максимально тонко настроить сборку – почти как панацея.
```kotlin
plugins {
    kotlin("jvm") // или multiplatform
}

kotlin {
    @OptIn(ExperimentalKotlinGradlePluginApi::class)
    publishing {
        adhocSoftwareComponent {
            // Конфигурация кастомных вариантов
        }
    }
}
```

**⏱️ Трекинг времени в stdlib**  
Новые классы `Clock` и `Instant` (аналоги из `kotlinx-datetime`):  
```kotlin
@OptIn(ExperimentalTime::class)
fun main() {
    val now = Clock.System.now()
    val past = Instant.parse("2023-01-01T00:00:00Z")
    println("Прошло времени: ${now - past}")
}
```
*Конвертеры*:  
- `.toJavaInstant()` → `java.time.Instant`  
- `.toJSDate()` → JS `Date` (с потерей точности).

---

#### **3. Kotlin/Wasm: Миграция на Provider API**  
Раньше:  
```kotlin
the<NodeJsExtension>().version = "2.0.0"
```  
Теперь:  
```kotlin
the<NodeJsEnvSpec>().version.set("2.0.0")
```  
*Зачем*: Ленивые вычисления и улучшенная интеграция с задачами Gradle.

**🚨 Заменили устаревшее**  

| Устарело                | Замена                              |
|-------------------------|-------------------------------------------|
| `wasmJsRun`             | `wasmJsBrowserDevelopmentRun`             |
| `wasmJsBrowserRun`      | `wasmJsBrowserDevelopmentRun`             |
| `wasmJsNodeRun`         | `wasmJsNodeDevelopmentRun`                |
| `wasmJsBrowserWebpack`  | `wasmJsBrowserProductionWebpack или wasmJsBrowserDistribution` |
| `jsRun`                 | `jsBrowserDevelopmentRun`                 |
| `jsBrowserRun`          | `jsBrowserDevelopmentRun`                 |
| `jsNodeRun`             | `jsNodeDevelopmentRun`                    |
| `jsBrowserWebpack`      | `jsBrowserProductionWebpack или jsBrowserDistribution`       |




---

#### **4. Стандартная библиотека**  
- **Атомарные типы**: Для конкурентных операций.  
- **UUID**: Улучшенная генерация и сериализация.  
- **Трекинг времени**: Точное измерение интервалов (см. раздел 2).

---

#### **5. Compose: Отладка и оптимизации**  
- **Исходная информация включена по умолчанию** для всех платформ.  
- **Поддержка default-аргументов в open-функциях**:  
  ```kotlin
  open class MyComponent {
      @Composable
      open fun Content(text: String = "default") { ... }
  }
  ```  
  *Важно*: Требуется Kotlin 2.1.20+ для корректной работы.  
- **Финальные функции могут быть restartable**: Убрано ограничение для `final override`.  
- **Удаление ComposableSingletons из публичного API**: Устранение утечек в inline-функциях.

---

#### **6. Gradle**  
- **Поддержка версий**: 7.6.3 – 8.11 (с предупреждениями в новых версиях).  
- **Изолированные проекты** (pre-Alpha):  
  - Включите через системное свойство.  
  - JS/Wasm пока не поддерживаются.  
  - *Отключение для Multiplatform*:  
    ```properties
    kotlin.kmp.isolated-projects.support=disable
    ```  
- **Кастомные публикации**: Экспериментальный API (см. раздел 2).

---

#### **7. Установка**  
- **IntelliJ IDEA 2023.3+ и Android Studio Iguana**: Плагин встроен.  
- **Обновление**: Пропишите в `build.gradle`:  
  ```kotlin
  plugins {
      kotlin("jvm") version "2.1.20"
  }
  ```

---

#### **8. Breaking Changes**  
- **Устарело**:  
  - `withJava()` → Java-сорс-сеты создаются автоматически.  
  - `kotlin-android-extensions` → Полная блокировка (ошибка конфигурации).  
- **Удалено**: Свойство `kotlin.incremental.classpath.snapshot.enabled`.

---

### **Итоги**  
Kotlin 2.1.20 фокусируется на **производительности** (Native, K2), **упрощении конфигурации** (Multiplatform DSL) и **отладке** (Wasm DWARF, Compose). Экспериментальные фичи вроде `@SuperBuilder` и нового DSL дают заглянуть в будущее языка.  

🔗 https://github.com/JetBrains/kotlin/releases/tag/v2.1.20

**Внимание:** Все экспериментальные фичи требуют явного `@OptIn`!

P.S. Если заметили баги с K2 — не паникуйте, всегда можно откатиться на K1 через `gradle.properties`.  
**Тестируйте и программируйте в удовольствие!** 🚀






--- 


### 📝 Тест по обновлению компилятора Kotlin 2.1.20


**1. Какая главная задача нового компилятора K2 в Kotlin 2.1.20?**

- [ ] Удаление поддержки Java
- [ ] Существенное ускорение компиляции и усовершенствованный анализ типов
- [ ] Переход исключительно на JavaScript
- [ ] Автоматическое написание юнит-тестов

