# Магия кривых: Как убрать паразитные оттенки и сделать красивый тон

> Разбор работы с кривыми (Curves) в графических редакторах: как исправить нежелательный красный отлив на фотографии и настроить правильный баланс цветов.

- **URL:** https://eagle2000.space/photography/curves-color-toning/
- **Date:** 2026-07-31
- **Categories:** photography
- **Tags:** обработка, коррекция, советы

---

<p>Вы наверняка замечали при обработке фотографий, что иногда в кадр пробирается неприятный, паразитный оттенок. Чаще всего это <strong>красный отлив</strong>, который портит тон кожи (скинтон) или лишает пейзаж естественности.</p>
<p>Как с этим бороться? Простой ползунок баланса белого может испортить всю остальную цветовую гамму. На помощь приходит мощнейший инструмент тонирования — <strong>Кривые (Curves)</strong>.</p>
<p>В этой заметке я поделюсь простым и проверенным рецептом из старой группы ВКонтакте о том, как убрать паразитный красный цвет и придать кадру благородную тонировку с помощью поканальной коррекции.</p>
<hr>

<h2 class="relative group">🛠 Рецепт тонирования кривыми
    <div id="-рецепт-тонирования-кривыми" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d1%80%d0%b5%d1%86%d0%b5%d0%bf%d1%82-%d1%82%d0%be%d0%bd%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f-%d0%ba%d1%80%d0%b8%d0%b2%d1%8b%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Весь процесс строится на тонкой работе с отдельными цветовыми каналами (RGB). Нам не нужно двигать общую кривую яркости — вместо этого мы переключаемся на конкретные цвета.</p>
<p><figure><img
    class="my-0 rounded-md"
    loading="lazy"
    decoding="async"
    fetchpriority="auto"
    alt="Панель кривых в Photoshop"
    width="963"
    height="1024"
    src="/photography/curves-color-toning/curves_hu_ca3acaa79ef55233.png"
    srcset="/photography/curves-color-toning/curves_hu_ca3acaa79ef55233.png 800w, /photography/curves-color-toning/curves.png 1280w"
    sizes="(min-width: 768px) 50vw, 65vw"
    data-zoom-src="/photography/curves-color-toning/curves.png"></figure>
</p>

<h3 class="relative group">Шаг 1. Компенсация красного через Зелёный канал
    <div id="шаг-1-компенсация-красного-через-зелёный-канал" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-1-%d0%ba%d0%be%d0%bc%d0%bf%d0%b5%d0%bd%d1%81%d0%b0%d1%86%d0%b8%d1%8f-%d0%ba%d1%80%d0%b0%d1%81%d0%bd%d0%be%d0%b3%d0%be-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-%d0%b7%d0%b5%d0%bb%d1%91%d0%bd%d1%8b%d0%b9-%d0%ba%d0%b0%d0%bd%d0%b0%d0%bb" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Красный и зелёный являются дополнительными (комплиментарными) цветами в цветовом круге. Чтобы ослабить красный, мы можем немного усилить зелёный:</p>
<ol>
<li>Перейдите в <strong>Зелёный канал (Green)</strong>.</li>
<li>Поставьте точку в самом центре кривой (средние тона).</li>
<li><strong>Слегка приподнимите</strong> её вверх. Будьте аккуратны — нам нужно очень незначительное изменение, иначе снимок станет «болотным».</li>
</ol>

<h3 class="relative group">Шаг 2. Нейтрализация зелени и придание тона в Синем канале
    <div id="шаг-2-нейтрализация-зелени-и-придание-тона-в-синем-канале" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-2-%d0%bd%d0%b5%d0%b9%d1%82%d1%80%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d0%b7%d0%b5%d0%bb%d0%b5%d0%bd%d0%b8-%d0%b8-%d0%bf%d1%80%d0%b8%d0%b4%d0%b0%d0%bd%d0%b8%d0%b5-%d1%82%d0%be%d0%bd%d0%b0-%d0%b2-%d1%81%d0%b8%d0%bd%d0%b5%d0%bc-%d0%ba%d0%b0%d0%bd%d0%b0%d0%bb%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>После подъёма зелёного цвета картинка может приобрести зеленоватый отлив. Мы компенсируем это в синем канале:</p>
<ol>
<li>Перейдите в <strong>Синий канал (Blue)</strong>.</li>
<li>Сделайте <strong>слабую S-образную кривую</strong>:
<ul>
<li>Слегка потяните вниз точку в тенях (левая нижняя часть), чтобы добавить в тени тёплый жёлтый оттенок.</li>
<li>Слегка приподнимите точку в светах (правая верхняя часть), чтобы добавить прохладный синий оттенок.</li>
</ul>
</li>
<li>Это компенсирует избыток зелени и создаст гармоничный цветовой контраст между светлыми и тёмными участками.</li>
</ol>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="tip">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span></div>
        <div class="grow">
          Совет
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p><strong>Горячие клавиши:</strong> В Adobe Photoshop быстро вызвать панель кривых можно с помощью сочетания клавиш <code>Ctrl + M</code> (на Windows) или <code>Cmd + M</code> (на macOS).</p></div></div><hr>

<h2 class="relative group">💡 Почему это работает?
    <div id="-почему-это-работает" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-%d1%8d%d1%82%d0%be-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0%d0%b5%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Работа с кривыми напрямую влияет на структуру пикселей в разных тоновых диапазонах. В отличие от простых ползунков насыщенности, поканальные кривые позволяют:</p>
<ul>
<li>Убирать нежелательный оттенок только там, где он мешает (например, в средних тонах, не затрагивая глубокие тени).</li>
<li>Создавать кинематографичный контраст за счёт раздельного тонирования теней и светов.</li>
<li>Сохранять естественность полутонов.</li>
</ul>
<p>Попробуйте применить этот приём на любом кадре с избытком тёплого красного света, и вы увидите, как фотография мгновенно приобретает чистый, профессиональный вид!</p>

