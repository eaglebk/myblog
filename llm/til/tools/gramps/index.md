# Обзор Gramps: ПО для генеалогических исследований и древа

> Как исследовать родословную и построить семейное древо с помощью open-source приложения Gramps? Полный разбор возможностей и экспорта данных.

- **URL:** https://eagle2000.space/til/tools/gramps/
- **Date:** 2024-04-20

- **Tags:** Семейное древо, OpenSource

---

<p>Изучая очередные проекты с открытым исходным кодом заметил интересный проект который предлагает пользователю
создать генеалогическое древо и провести генеалогическое исследование.</p>
<p>Как обычно у приложения есть вики-сайт с <a href="https://gramps-project.org/wiki/index.php/Ru:Gramps_5.2_%D0%92%D0%B8%D0%BA%D0%B8_%D0%A0%D1%83%D0%BA%D0%BE%D0%B2%D0%BE%D0%B4%D1%81%D1%82%D0%B2%D0%BE"  target="_blank" rel="noreferrer">документацией</a></p>
<p>Интерфейс чем-то похож на <a href="https://calibre-ebook.com"  target="_blank" rel="noreferrer">calibre</a> для работы со своими электронными книгами.</p>
<p><a href="https://gramps-project.org"  target="_blank" rel="noreferrer">https://gramps-project.org</a></p>

<figure>
      <img class="my-0 rounded-md" src="https://i.imgur.com/NybXGrQ.png" alt="Gramps image site screenshoot" />
  
  
  </figure>

<h2 class="relative group">Впечатления от приложения за 5 минут использования
    <div id="впечатления-от-приложения-за-5-минут-использования" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%bf%d0%b5%d1%87%d0%b0%d1%82%d0%bb%d0%b5%d0%bd%d0%b8%d1%8f-%d0%be%d1%82-%d0%bf%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d1%8f-%d0%b7%d0%b0-5-%d0%bc%d0%b8%d0%bd%d1%83%d1%82-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Запустил, создал новый проект. В левой части окна отобразилась справка (но кто же ее будет читать).
Мне не сильно приглянулся интерфейс, крайне скудная цветовая гамма и малопонятные иконки у кнопок.
Заметно, что приложение предназначено для настоящих исследователей, которых не напугают никакие преграды и интерфейсы из 2000-x годов (GTK+/GNOME).</p>
<p>Я загрузил тестовую базу данных и немного погулял по приложению чтобы изучить его возможности - <a href="https://github.com/gramps-project/gramps/tree/master/example/gramps"  target="_blank" rel="noreferrer">ссылка на тестовую базу</a></p>
<p>На русском языке к сожалению такую не обнаружил. Возможно плохо искал.</p>

<h2 class="relative group">Код
    <div id="код" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Покопался в исходниках и посмотрел как в целом устроен проект.
Приложение написано на python - <a href="https://github.com/gramps-project/gramps"  target="_blank" rel="noreferrer">репозиторий</a></p>
<p>Можно скачивать и устанавливать различные плагины для расширения функционала.
Обнаружил в исходниках даже cli. Дальше не стал изучать, что с ним можно сделать, но скорее всего используется для автоматизации рутиных действий.</p>
<p>Репозиторий хорошо структурирован. Содержит .github/workflows для автоматической сборки проекта под различные архитектуры.
Разработчики содержат код и репозиторий в порядке, молодцы!</p>

<h2 class="relative group">Вывод
    <div id="вывод" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d0%b2%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Проект большой, требует достаточно времени чтобы читать документацию и изучать все вкладки, окна приложения.</p>
<p>Что не понравилось:</p>
<ul>
<li>Не хватает наглядности</li>
<li>Нет возможности загрузить шаблоны для отчетов с представлением результата</li>
<li>Результат генерации, будь то pdf документ либо сайт сразу не открывается, никаких окон уведомлений нет</li>
<li>Большой функционал не обернутый в красивую обложку (этим страдает множество свободных проектов)</li>
<li>Нет понимания как интегрировать это приложение с другими инструментами</li>
<li>Нет облачного хранилища и возможности поделиться резульатом работы чтобы твои родственники могли дополнить информацию</li>
</ul>
<p>Какие есть плюсы и возможности:</p>
<ul>
<li>Можно строить связи</li>
<li>Можно строить древо в различных представлениях. Оно будет интерактивным, достаточно удобным</li>
<li>Использовать множество фильтров</li>
<li>Добавлять геометки</li>
<li>Строить отчеты</li>
<li>Добавлять заметки</li>
<li>Добавлять ссылки</li>
</ul>

<figure>
      <img class="my-0 rounded-md" src="https://i.imgur.com/2y7NoKE.png" alt="Gramps familty tree screenshoot" />
  
  <figcaption>Скриншот приложения gramps</figcaption>
  </figure>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M506.3 417l-213.3-364c-16.33-28-57.54-28-73.98 0l-213.2 364C-10.59 444.9 9.849 480 42.74 480h426.6C502.1 480 522.6 445 506.3 417zM232 168c0-13.25 10.75-24 24-24S280 154.8 280 168v128c0 13.25-10.75 24-23.1 24S232 309.3 232 296V168zM256 416c-17.36 0-31.44-14.08-31.44-31.44c0-17.36 14.07-31.44 31.44-31.44s31.44 14.08 31.44 31.44C287.4 401.9 273.4 416 256 416z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    >Ставь лайк если интересны проекты с открытым исходным кодом. (Кнопка Like находится под заголовком поста)</span>
</div>


