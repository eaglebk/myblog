# Эффективная обработка данных из нескольких источников на Dart

> Как объединять асинхронные потоки данных (StreamGroup) в Dart и Flutter с сохранением контекста источников? Разбор практических паттернов и примеров кода.

- **URL:** https://eagle2000.space/dart/streams-groups/
- **Date:** 2024-05-11

- **Tags:** dart, dev

---
## Введение:
Сегодня я расскажу вам о том, как можно объединить несколько потоков данных Stream в Dart, сохраняя при этом их идентичность. Это полезно, если вы создаете систему реального времени, где данные поступают из разных источников и вам важно знать, откуда они пришли. Давайте начнем!

## Проблема
Представьте, что вы разрабатываете приложение для мониторинга различных сенсоров в умном доме. (Люблю эту тему 😁) 

У вас есть три потока данных: 
- температура (Temperature)
- влажность (Humidity)
- статус датчика движения (Motion)

Эти потоки поступают от разных сенсоров:

```dart
Stream<int> temperatureStream = getTemperatureStream();
Stream<int> humidityStream = getHumidityStream();
Stream<int> motionStream = getMotionStream();
```

На первый взгляд, вы можете подумать, что объединить эти потоки довольно просто. Используем класс [StreamGroup](https://api.flutter.dev/flutter/async/StreamGroup-class.html):

```dart
Stream<int> combinedStream = StreamGroup.merge([temperatureStream, humidityStream, motionStream]);

combinedStream.listen((event) {
  print(event);
});

```

И действительно, это работает. Но вот проблема: когда данные начинают поступать из этих объединенных потоков, вы видите что-то вроде этого:

```dart
> 25
> 60
> 1
```

## Решение
Чтобы решить эту проблему, предлагаю метод, который позволит не только объединить потоки, но и сохранить информацию об их источнике. Мы создадим два вспомогательных класса и метод для объединения потоков. [Увидел в статье Punnyarthabanerjee](https://medium.com/@punnyarthabanerjee/handling-multiple-streams-better-than-streamgroup-class-in-flutter-bb11e7982de2) подобный подход и решил адаптировать для себя

Шаг 1: Класс StreamIdentity

Этот класс будет хранить идентификатор потока и сам поток:

```dart
class StreamIdentity {
  final String identifier;
  final Stream stream;

  StreamIdentity(this.identifier, this.stream);
}
```

Шаг 2: Класс MergedStreamResponse

Этот класс будет содержать данные `data` и информацию о том, из какого потока они пришли `StreamIdentity`, а также временную метку `timestamp`:

```dart
class MergedStreamResponse {
  final StreamIdentity streamIdentity;
  final dynamic data;
  final DateTime timestamp;

  MergedStreamResponse(this.streamIdentity, this.data) : timestamp = DateTime.now();
}
```

Шаг 3: Метод mergeStream

Этот метод объединит все потоки и будет добавлять данные в контроллер вместе с их идентификатором:

```dart
Stream<MergedStreamResponse> mergeStream(List<StreamIdentity> streams) {
  final StreamController<MergedStreamResponse> controller = StreamController();

  for (var identity in streams) {
    identity.stream.listen((event) {
      controller.add(MergedStreamResponse(identity, event));
    });
  }

  return controller.stream;
}
```

## Применение

Теперь мы можем передать наши потоки в метод `mergeStream` и обрабатывать данные с учетом их источника:

```dart
void main() {
  Stream<int> temperatureStream = getTemperatureStream();
  Stream<int> humidityStream = getHumidityStream();
  Stream<int> motionStream = getMotionStream();

  List<StreamIdentity> streams = [
    StreamIdentity('temperature', temperatureStream),
    StreamIdentity('humidity', humidityStream),
    StreamIdentity('motion', motionStream),
  ];

  Stream<MergedStreamResponse> combinedStream = mergeStream(streams);

  combinedStream.listen((response) {
    print('Data from ${response.streamIdentity.identifier} at ${response.timestamp}: ${response.data}');
  });
}

```

Теперь данные выводятся с указанием их источника и времени получения:

```shell
Data from temperature at 2024-05-10 12:00:00: 25
Data from humidity at 2024-05-10 12:00:01: 60
Data from motion at 2024-05-10 12:00:02: 1
```

## Пример реализации (полный код)

```dart

class StreamIdentity {
  final String identifier;
  final Stream stream;

  StreamIdentity(this.identifier, this.stream);
}

class MergedStreamResponse {
  final StreamIdentity streamIdentity;
  final dynamic data;
  final DateTime timestamp;

  MergedStreamResponse(this.streamIdentity, this.data) : timestamp = DateTime.now();
}


Stream<MergedStreamResponse> mergeStream(List<StreamIdentity> streams) {
  final StreamController<MergedStreamResponse> controller = StreamController();

  for (var identity in streams) {
    identity.stream.listen((event) {
      controller.add(MergedStreamResponse(identity, event));
    });
  }

  return controller.stream;
}

void main() {
  Stream<double> temperatureSensor = getTemperatureStream();
  Stream<double> humiditySensor = getHumidityStream();
  Stream<int> motionSensor = getMotionStream();

  List<StreamIdentity> sensors = [
    StreamIdentity('temperature', temperatureSensor),
    StreamIdentity('humidity', humiditySensor),
    StreamIdentity('motion', motionSensor),
  ];

  Stream<MergedStreamResponse> combinedStream = mergeStream(sensors);

  combinedStream.listen((response) {
    print('Data from ${response.streamIdentity.identifier} at ${response.timestamp}: ${response.data}');
  });
}

Stream<double> getTemperatureStream() async* {
  yield* Stream.periodic(Duration(seconds: 1), (count) => 20.0 + count);
}

Stream<double> getHumidityStream() async* {
  yield* Stream.periodic(Duration(seconds: 2), (count) => 50.0 + count * 0.5);
}

Stream<int> getMotionStream() async* {
  yield* Stream.periodic(Duration(seconds: 3), (count) => count % 2);
}
```

В этом примере данные от датчика температуры, влажностности и датчика движения объединяются и выводятся с указанием их источника и времени получения. Это позволяет легко отслеживать и анализировать данные от каждого сенсора, что значительно упрощает принятие решений.

## Заключение
Надеюсь, эта статья была полезной и вы узнали что-то новое о работе с потоками в Dart. Если у вас есть вопросы или комментарии, обращайтесь. Стараюсь писать кратко и по делу. Спасибо за внимание и спасибо [автору статьи на медиум](https://medium.com/@punnyarthabanerjee/handling-multiple-streams-better-than-streamgroup-class-in-flutter-bb11e7982de2) за оригинальную идею!


--- 


### 📝 Тест по асинхронным потокам и StreamGroup в Dart


**1. Какой класс из пакета async позволяет объединять несколько Stream в единый поток?**

- [ ] StreamController
- [ ] StreamGroup
- [ ] FutureGroup
- [ ] StreamZip

**2. Чем отличаются одиночные потоки (Single-subscription) от Broadcast Stream в Dart?**

- [ ] Single-subscription допускает только одного слушателя, а Broadcast — нескольких
- [ ] Single-subscription работает только асинхронно
- [ ] Broadcast Stream нельзя отменить
- [ ] Ничем не отличаются

