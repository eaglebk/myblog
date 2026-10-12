# Как отправить несколько файлов одним сообщением через Telegram Bot API

> Пошаговый гайд по отправке медиагруппы из нескольких файлов, фотографий и документов одним альбомом или сообщением через Telegram Bot API с примерами кода.

- **URL:** http://localhost:1314/about-all/telegram-multiple-files/
- **Date:** 2025-02-08
- **Categories:** about all


---
В разработке приложений взаимодействующих с telegram bot api часто возникает необходимость отправки нескольких файлов одновременно. 
(Я использую этот механизм для отладки иногда)

Telegram API предоставляет для этого специальный метод `sendMediaGroup`, который позволяет отправлять до 10 файлов в одном сообщении. В этой статье разберемся, как использовать этот метод с помощью различных инструментов.

## Преимущества использования sendMediaGroup

- Все файлы группируются в одно сообщение
- Уменьшается количество запросов к API
- Возможность добавления подписей к каждому файлу
- Поддержка различных типов медиа (фото, документы, видео)

## Подготовка тестовых файлов

Для демонстрации работы метода создадим два тестовых файла:

```bash
echo "Test content 1" > test1.txt
echo "Test content 2" > test2.txt
```

## Отправка через cURL

Самый простой способ протестировать API – использовать cURL. Вот полный пример команды:

```bash
curl -X POST https://api.telegram.org/bot<ТВОЙ_TOKEN>/sendMediaGroup \
-F chat_id="ТВОЙ_CHAT_ID" \
-F "media=[{\"type\":\"document\",\"media\":\"attach://doc1\",\"caption\":\"Test files 1\",\"parse_mode\":\"HTML\"},{\"type\":\"document\",\"media\":\"attach://doc2\",\"caption\":\"Test files 2\"}]" \
-F doc1=@"test1.txt" \
-F doc2=@"test2.txt"
```

Разберем параметры запроса:
- `-X POST` - указывает метод запроса
- `-F` - добавляет form-data поле
- `media` - JSON-массив с описанием медиафайлов
- `attach://doc1` - указывает на привязку к form-data полю doc1

## Реализация на Dart

Для тех, кто разрабатывает на Dart/Flutter, вот пример реализации той же функциональности:

```dart
import 'package:http/http.dart' as http;
import 'package:http_parser/http_parser.dart';

Future<void> sendMediaGroup() async {
  final token = 'ТВОЙ_TOKEN';
  final chatId = 'ТВОЙ_CHAT_ID';
  final url = Uri.parse('https://api.telegram.org/bot$token/sendMediaGroup');

  final request = http.MultipartRequest('POST', url)
    ..fields['chat_id'] = chatId
    ..fields['media'] = '''
    [
      {"type": "document", "media": "attach://doc1", "caption": "Test files 1"},
      {"type": "document", "media": "attach://doc2", "caption": "Test files 2"}
    ]
    '''
    ..files.add(await http.MultipartFile.fromPath('doc1', 'test1.txt'))
    ..files.add(await http.MultipartFile.fromPath('doc2', 'test2.txt'));

  final response = await request.send();
  print(await response.stream.bytesToString());
}
```

## Важные замечания

1. Максимальное количество файлов в одной группе - 10
2. Все файлы должны быть одного типа (например, все документы или все фотографии)
3. Размер каждого файла не должен превышать лимиты Telegram
4. Поддерживаются следующие типы медиа: photo, video, audio, document

## Заключение

Метод `sendMediaGroup` - мощный инструмент для отправки нескольких файлов в одном сообщении. Он особенно полезен при разработке ботов, где требуется отправка связанных файлов или создание альбомов медиа-контента.

Спасибо что читаете! 👾

### Полезные ссылки

- [Официальная документация Telegram Bot API](https://core.telegram.org/bots/api#sendmediagroup)
- [Limits in Telegram Bot API](https://core.telegram.org/bots/api#senddocument)
- [Документация как создать и работать с чат-ботом Telegram](https://core.telegram.org/bots/tutorial)


