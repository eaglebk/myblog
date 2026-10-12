# Генерация статического контента и API документации в Flutter

> Гайд по утилите assets-for-api-docs и генерации статических графических ресурсов и примеров для документации Flutter API и пакетов Dart.

- **URL:** https://eagle2000.space/til/flutter/1711660895366-flutter/
- **Date:** 2024-03-28

- **Tags:** flutter

---

<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  Генерация статического контента для сайта docs flutter
</div>

<p>Static assets for embedding into docs.flutter.io <a href="https://github.com/flutter/assets-for-api-docs/tree/main"  target="_blank" rel="noreferrer">Assets for api docs repo</a></p>
<div class="github-card-wrapper">
    <a id="github-fcbb394e3ed48d810a57d3f142657671" target="_blank" href="https://github.com/flutter/assets-for-api-docs" class="cursor-pointer">
      <div
        class="w-full md:w-auto p-0 m-0 border border-neutral-200 dark:border-neutral-700 border rounded-md shadow-2xl"><div class="w-full nozoom">
            <img
              src="https://opengraph.githubassets.com/0/flutter/assets-for-api-docs"
              alt="GitHub Repository Thumbnail"
              class="nozoom mt-0 mb-0 w-full h-full object-cover">
          </div><div class="w-full md:w-auto pt-3 p-5">
          <div class="flex items-center">
            <span class="text-2xl text-neutral-800 dark:text-neutral me-2">
              <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512"><path fill="currentColor" d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/></svg>
</span>
            </span>
            <div
              id="github-fcbb394e3ed48d810a57d3f142657671-full_name"
              class="m-0 font-bold text-xl text-neutral-800 decoration-primary-500 hover:underline hover:underline-offset-2 dark:text-neutral">
              flutter/assets-for-api-docs
            </div>
          </div>

          <p id="github-fcbb394e3ed48d810a57d3f142657671-description" class="m-0 mt-2 text-md text-neutral-800 dark:text-neutral">
            Static assets for embedding into docs.flutter.io
          </p>

          <div class="m-0 mt-2 flex items-center">
            <span class="mr-1 inline-block h-3 w-3 rounded-full language-dot" data-language="Dart"></span>
            <div class="m-0 mr-5 text-md text-neutral-800 dark:text-neutral">
              Dart
            </div>

            <span class="text-md mr-1 text-neutral-800 dark:text-neutral">
              <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path fill="currentColor" d="M287.9 0C297.1 0 305.5 5.25 309.5 13.52L378.1 154.8L531.4 177.5C540.4 178.8 547.8 185.1 550.7 193.7C553.5 202.4 551.2 211.9 544.8 218.2L433.6 328.4L459.9 483.9C461.4 492.9 457.7 502.1 450.2 507.4C442.8 512.7 432.1 513.4 424.9 509.1L287.9 435.9L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.9 115.1 483.9L142.2 328.4L31.11 218.2C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C270.4 5.249 278.7 0 287.9 0L287.9 0zM287.9 78.95L235.4 187.2C231.9 194.3 225.1 199.3 217.3 200.5L98.98 217.9L184.9 303C190.4 308.5 192.9 316.4 191.6 324.1L171.4 443.7L276.6 387.5C283.7 383.7 292.2 383.7 299.2 387.5L404.4 443.7L384.2 324.1C382.9 316.4 385.5 308.5 391 303L476.9 217.9L358.6 200.5C350.7 199.3 343.9 194.3 340.5 187.2L287.9 78.95z"/></svg></span>
            </span>
            <div id="github-fcbb394e3ed48d810a57d3f142657671-stargazers" class="m-0 mr-5 text-md text-neutral-800 dark:text-neutral">
              122
            </div>

            <span class="text-md mr-1 text-neutral-800 dark:text-neutral">
              <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M80 104c13.3 0 24-10.7 24-24s-10.7-24-24-24S56 66.7 56 80s10.7 24 24 24zm80-24c0 32.8-19.7 61-48 73.3V192c0 17.7 14.3 32 32 32H304c17.7 0 32-14.3 32-32V153.3C307.7 141 288 112.8 288 80c0-44.2 35.8-80 80-80s80 35.8 80 80c0 32.8-19.7 61-48 73.3V192c0 53-43 96-96 96H256v70.7c28.3 12.3 48 40.5 48 73.3c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-32.8 19.7-61 48-73.3V288H144c-53 0-96-43-96-96V153.3C19.7 141 0 112.8 0 80C0 35.8 35.8 0 80 0s80 35.8 80 80zm208 24c13.3 0 24-10.7 24-24s-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24zM248 432c0-13.3-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24s24-10.7 24-24z"/></svg></span>
            </span>
            <div id="github-fcbb394e3ed48d810a57d3f142657671-forks" class="m-0 mr-5 text-md text-neutral-800 dark:text-neutral">
              108
            </div>
          </div>
        </div>
      </div>
      
      
      <script
        async
        type="text/javascript"
        src="/js/fetch-repo.min.dc5533c50cefd50405344b235937142271f26229fe39cbee27fd4960e8bb897a0beebfad77a1091ca91cd0d1fb14e70fc37cc114dd9674fb2c32e0ab512ec8a4.js"
        integrity="sha512-3FUzxQzv1QQFNEsjWTcUInHyYin&#43;OcvuJ/1JYOi7iXoL7r&#43;td6EJHKkc0NH7FOcPw3zBFN2WdPssMuCrUS7IpA=="
        data-repo-url="https://api.github.com/repos/flutter/assets-for-api-docs"
        data-repo-id="github-fcbb394e3ed48d810a57d3f142657671"></script>
    </a>
  </div>
<p>Много примеров, интересные решения для генерации анимаций, картинок. Стоит изучить этот репозиторий</p>

  
    
    
    <div class="width-patch"></div>
<div id="gallery-7089e6485f1a12313cd4ae56cea3e84f" class="gallery">
  
  <img src="https://github.com/flutter/assets-for-api-docs/blob/main/assets/flutter-test/goldens/widget.gif?raw=true" decoding="async" class="grid-w33" />
  <img src="https://github.com/flutter/assets-for-api-docs/blob/main/assets/cupertino/cupertino_active_colors.png?raw=true" decoding="async" class="grid-w33" />
  <img src="https://github.com/flutter/assets-for-api-docs/blob/main/assets/material/data_table.png?raw=true" decoding="async" class="grid-w33" />
</div>


