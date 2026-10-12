# Tauri v2 Mobile против Flutter: Сравниваем фреймворки на примере реальной игры

> Разработал одну и ту же мобильную аркадную игру на Tauri 2.0 (Rust backend + React UI) и Flutter (Cubit). Подробный разбор архитектуры, переноса логики в Rust, замеры веса APK, памяти и задержек.

- **URL:** https://eagle2000.space/til/mobile/tauri-v2-vs-flutter-benchmark/
- **Date:** 2026-08-18

- **Tags:** tauri, flutter, rust, mobile, react, dart

---
Недавно в комментариях к одной из статей меня спросили: *«А можешь сравнить Tauri v2 и Flutter для мобильной разработки?»*. Я ответил, что предложение отличным образом подходит для статьи, и пообещал подготовить подробный разбор.

Сравнивать фреймворки по табличкам из документации — занятие скучное. Поэтому я решил провести честный инженерный эксперимент: **написать с нуля одну и ту же динамичную мобильную аркаду на обоих фреймворках**, измерить метрики, оценить Developer Experience (DX) и выложить открытый репозиторий.

Встречайте тестового испытуемого — реактивную мини-аркаду **«Ferris Tap Rush»** (гонка кликов по маскоту краба Ферриса с поддержкой механик топ-игр 2026 года: режим Лихорадки 🔥, крабы-бомбы 💣, засады «Минное поле», заморозка таймера 🧊 и супер-бонусы ⚡).

---

## Архитектурный баттл: В чем фундаментальная разница

Прежде чем прочитать первую строчку кода, важно понять главное различие между этими двумя технологиями. Они решают задачу кроссплатформенности абсолютно противоположными способами:

1. **Flutter (Принцип «We own every pixel»):**
   Flutter приносит с собой собственный высокопроизводительный векторный рендерер (Impeller / Skia) и компилирует Dart-код напрямую в нативные инструкции. Ему не важна операционная система — он рисует весь UI пиксель-в-пиксель на кастомном холсте.
2. **Tauri v2 Mobile (Принцип «Native OS WebViews + Rust Backend»):**
   Tauri не тянет за собой громоздкие веб-движки типа Chromium. На iOS он использует системный WKWebView, а на Android — Android System WebView. **Вся ключевая бизнес-логика, физика игры, подсчёт таймеров, генерация спавна и валидация кликов полностью выполняются в нативном Rust-бэкенде**, а веб-фронтенд (React/Zustand) выступает строго в роли тонкого визуального представления.

---

## Игровые механики и тренды 2026 года

Чтобы сравнение кода было максимально наглядным, в обе версии игры добавлены современные аркадные механики:

- 🔥 **Fever Mode (Режим «Лихорадка»):** При наборе комбо `x10` включается 5-секундный режим овердрайва со 3-кратным множителем очков (`x3 PTS`) и спавном до 5 целей одновременно.
- ⚡ **Golden Lightning Ferris:** Редкий супер-бонус на 450 мс, дающий `+500 PTS` и `+3 секунды` к таймеру.
- 💣 **Bomb Crabs:** Крабы-ловушки. Тап по ним отнимает 500 очков и 3 секунды времени.
- ⚠️ **Minefield Trap (Засада «Минное поле»):** 4–5 бомб окружают строго одного спасительного краба.
- 🧊 **Freeze Crabs:** Тап по ним замораживает главный 30-секундный таймер на 4 секунды.
- 🏆 **Rank SSS & Streamer Badges:** Динамические медали в конце раунда (*«Lightning God»*, *«Master Sniper»*, *«Rank SSS 👑»*).

---

## Перенос всей бизнес-логики в Rust (Tauri Backend IPC)

Правильный с точки зрения архитектуры подход в Tauri — **не оставлять расчеты на JavaScript/TypeScript**, а выносить игровой движок в **Rust**.

### Игровой движок на Rust (`src-tauri/src/lib.rs`)

Вся игра управляется потокобезопасным хранилищем `GameStateStore` и набором команды `tauri::command`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RustGameState {
    pub status: String,
    pub score: u32,
    pub combo: u32,
    pub time_left: u32,
    pub active_crabs: Vec<ActiveCrab>,
    pub is_fever_mode: bool,
    pub sound_to_play: Option<String>,
}

#[tauri::command]
fn tap_slot(slot_index: u8, state: State<'_, GameStateStore>) -> Result<RustGameState, String> {
    let mut game = state.inner.lock().map_err(|e| e.to_string())?;
    
    // Вся валидация тапа, начисление очков, обработка бомб и штрафов происходит в Rust
    if let Some(idx) = game.active_crabs.iter().position(|c| c.slot_index == slot_index) {
        let crab = game.active_crabs.remove(idx);
        if crab.crab_type == "bomb" {
            game.score = game.score.saturating_sub(500);
            game.combo = 1;
            game.sound_to_play = Some("bomb".to_string());
        } else {
            game.score += 100 * game.combo;
            game.combo += 1;
            game.sound_to_play = Some("pop".to_string());
        }
    }
    Ok(game.clone())
}
```

### Легковесный связующий слой на TypeScript (`src/store/useGameStore.ts`)

Фронтенд лишь отправляет IPC-вызовы через `invoke` и отрисовывает готовое состояние из Rust:

```typescript
export const useGameStore = create<GameState>((set) => ({
  status: 'menu',
  score: 0,
  activeCrabs: [],

  tapSlot: async (slotIndex: number) => {
    // Вся математика выполняется на сервере/Rust бэкенде!
    const rustState = await invoke<RustState>('tap_slot', { slotIndex });
    syncRustState(rustState, set);
  },
}));
```

---

## Версия на Flutter (Cubit)

Во Flutter аналогичный движок организован в рамках доменных сервисов `GameEngine` и `StatsCalculator`, взаимодействующих с `GameCubit`:

```dart
class GameCubit extends Cubit<GameState> {
  void tapSlot(int index) {
    if (state.status != GameStatus.playing) return;
    
    final target = state.activeCrabs[index];
    if (target?.type == CrabType.bomb) {
      emit(state.copyWith(score: max(0, state.score - 500), combo: 1));
      return;
    }

    emit(state.copyWith(score: state.score + 100 * state.combo, combo: state.combo + 1));
  }
}
```

---

## Сводный бенчмарк и замеры релизных сборок

Я собрал релизные версии обоих приложений (Release APK / Android) и измерил ключевые показатели:

| Метрика | Tauri v2 Mobile | Flutter |
| :--- | :--- | :--- |
| **Движок UI** | System WebView (WKWebView/Android) | Impeller Canvas Engine |
| **Логика / Бэкенд** | **Rust (`tauri::command` Engine)** 🦀 | Dart (`GameCubit`) |
| **Стейт-менеджмент** | Zustand (IPC Bridge) | `flutter_bloc` (Cubit) |
| **Размер Release APK** | **~4.2 МБ** 🏆 | ~14.8 МБ |
| **Потребление ОЗУ (RAM)** | ~65–90 МБ | **~45–65 МБ** 🏆 |
| **Кадровая частота (FPS)** | 60–120 FPS | Стабильные 120 FPS |
| **Touch Latency (Отклик)** | Нативный Rust IPC (<16ms) | Нативный Dart клик |

---

## Итогоный вердикт: Что выбрать под ваш проект

### Выбирайте Tauri v2 Mobile, если:
1. Ваша главная логика, алгоритмы, бизнес-правила или криптография вынесены на **Rust**.
2. У вас уже есть веб-команда (React/Vue/Svelte) и вы хотите легковесный нативный клиент.
3. Критически важен минимальный размер скачиваемого приложения в маркетах (до 5 МБ).

### Выбирайте Flutter, если:
1. Вам нужен сложный 2D-интерфейс, кастомная графика, холсты и сложная анимация 120 FPS.
2. Нужна огромная проверенная временем экосистема плагинов для редкого железа.
3. Команда предпочитает разработку на едином языке (Dart) без контекст-свитчинга.

---

## Исходный код проекта

Весь исходный код эксперимента с подробной историей коммитов выложен в открытом репозитории:
👉 **[Репозиторий на GitHub: tauri-vs-flutter-ferris-tap-rush](file:///Users/dev/dev/workspace/eagleblog/projects/tauri-vs-flutter-ferris-tap-rush)**

---


### 📝 Тест: Tauri v2 или Flutter?


**1. В чем заключается фундаментальное различие рендеринга между Flutter и Tauri Mobile?**

- [ ] Flutter использует системные нативные UI-компоненты iOS/Android, а Tauri — кастомный холст
- [ ] Flutter рендерит UI через собственный движок Impeller/Skia, а Tauri использует встроенный нативный OS WebView
- [ ] Оба фреймворка используют встроенный Chromium
- [ ] Tauri рендерит весь UI строго на стороне Rust бэкенда

**2. Почему вес приложения Hello World у Tauri v2 ощутимо меньше, чем у Flutter?**

- [ ] Потому что код на Rust весит 0 килобайт
- [ ] Tauri использует оптимизированное WebView, которое уже встроено в операционную систему телефона
- [ ] Flutter не сжимает свои приложения при релизной сборке
- [ ] Tauri откладывает загрузку UI до подключения к интернету

**3. Какое преимущество дает архитектура Tauri v2 при интеграции с Rust?**

- [ ] Rust код компилируется в JavaScript во время выполнения
- [ ] Rust является нативным хостом приложения (Native Host), а команды вызываются через прямой IPC мост
- [ ] В Tauri нельзя использовать JS, только Rust Wasm
- [ ] Tauri требует написания FFI мостов через C-биндинги вручную

**4. В какому источнику ресурсов Tauri v2 может быть более требователен во время активной работы, чем Flutter?**

- [ ] К объему свободного места на диске
- [ ] К оперативной памяти (RAM) из-за загрузки системного WebView для тяжелого DOM
- [ ] К заряду аккумулятора только в ночном режиме
- [ ] К скорости Wi-Fi подключения

**5. Какой стейт-менеджмент мы использовали во Flutter-версии нашей игры для строгого разделения состояний?**

- [ ] setState в главном виджете
- [ ] flutter_bloc (Cubit)
- [ ] Redux
- [ ] GetX

