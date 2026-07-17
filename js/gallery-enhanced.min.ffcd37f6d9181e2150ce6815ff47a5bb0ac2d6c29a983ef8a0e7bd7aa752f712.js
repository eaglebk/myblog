(function(){function e(){const a=document.querySelector(".gallery");if(!a)return;let t=[],n=0,c=null,i=!1;const e=document.createElement("div");e.className="gallery-lightbox",e.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Запустить слайд-шоу" title="Запустить слайд-шоу">▶️</button>
          <button class="toolbar-btn btn-toggle-info" aria-label="Скрыть/показать описание" title="Скрыть описание">👁️</button>
        </div>
        <div class="toolbar-center">
          <div class="color-swatches">
            <button class="swatch" data-color="#ffffff" style="background-color: #ffffff;" title="Белый"></button>
            <button class="swatch" data-color="#e5e7eb" style="background-color: #e5e7eb;" title="Светло-серый"></button>
            <button class="swatch" data-color="#7f7f7f" style="background-color: #7f7f7f;" title="Серый"></button>
            <button class="swatch" data-color="#374151" style="background-color: #374151;" title="Тёмно-серый"></button>
            <button class="swatch is-active" data-color="#18181b" style="background-color: #18181b;" title="Угольный (по умолчанию)"></button>
            <button class="swatch" data-color="#000000" style="background-color: #000000;" title="Чёрный"></button>
          </div>
        </div>
        <div class="toolbar-right">
          <button class="lightbox-close" aria-label="Закрыть">&times;</button>
        </div>
      </div>
      <button class="lightbox-btn lightbox-prev" aria-label="Предыдущее фото">&#10094;</button>
      <button class="lightbox-btn lightbox-next" aria-label="Следующее фото">&#10095;</button>
      <div class="lightbox-container">
        <div class="lightbox-img-wrapper">
          <img class="lightbox-image" src="" alt="" />
        </div>
        <div class="lightbox-metadata-card">
          <div class="lightbox-title"></div>
          <div class="lightbox-story-box" style="display: none;">
            <span class="story-text"></span>
          </div>
          <div class="exif-grid">
            <div class="exif-item">
              <span class="exif-label">Камера</span>
              <span class="exif-value camera-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Объектив</span>
              <span class="exif-value lens-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Параметры</span>
              <span class="exif-value params-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Локация</span>
              <span class="exif-value location-val">-</span>
            </div>
            <div class="exif-item">
              <span class="exif-label">Дата</span>
              <span class="exif-value date-val">-</span>
            </div>
          </div>
          <div class="lightbox-music-bar">
            <span class="music-icon-pulse">🎧</span>
            <span class="music-text">
              <span class="music-label">В наушниках играло:</span>
              <span class="music-track-name">-</span>
            </span>
          </div>
        </div>
      </div>
    `,document.body.appendChild(e);const o=e.querySelector(".lightbox-image"),y=e.querySelector(".lightbox-title"),b=e.querySelector(".camera-val"),A=e.querySelector(".lens-val"),M=e.querySelector(".params-val"),S=e.querySelector(".location-val"),k=e.querySelector(".date-val"),p=e.querySelector(".lightbox-music-bar"),E=e.querySelector(".music-track-name"),x=e.querySelector(".lightbox-close"),w=e.querySelector(".lightbox-prev"),_=e.querySelector(".lightbox-next"),r=e.querySelector(".btn-slideshow"),F=e.querySelector(".btn-toggle-info"),z=e.querySelectorAll(".swatch"),l=Array.from(a.querySelectorAll("img"));t=[...l];function v(){t=l.filter(e=>!e.classList.contains("filtered-out"))}function O(){if(i)return;i=!0,r.textContent="⏸️",r.setAttribute("title","Приостановить слайд-шоу"),c=setInterval(()=>{s(n+1,!0)},3500)}function g(){if(!i)return;i=!1,r.textContent="▶️",r.setAttribute("title","Запустить слайд-шоу"),c&&(clearInterval(c),c=null)}function T(){i?g():O()}function s(s,i=!1){i||g(),s<0&&(s=t.length-1),s>=t.length&&(s=0),n=s;const a=t[n];if(!a)return;o.style.opacity="0",setTimeout(()=>{o.src=a.src,o.alt=a.alt||"Фотография";const f=a.getAttribute("data-title")||a.alt||"Фото",u=a.getAttribute("data-camera")||"—",d=a.getAttribute("data-lens")||"—",t=a.getAttribute("data-aperture"),s=a.getAttribute("data-shutter"),n=a.getAttribute("data-iso"),h=a.getAttribute("data-location")||"—",m=a.getAttribute("data-date")||"—",r=a.getAttribute("data-music"),i=a.getAttribute("data-story");let c="—";if(t||s||n){const e=[];t&&e.push(t),s&&e.push(s),n&&e.push(`ISO ${n}`),c=e.join(" • ")}y.textContent=f,b.textContent=u,A.textContent=d,M.textContent=c,S.textContent=h,k.textContent=m;const l=e.querySelector(".lightbox-story-box"),g=e.querySelector(".story-text");i?(g.textContent=i,l.style.display="block"):l.style.display="none",r?(E.textContent=r,p.style.display="flex"):p.style.display="none",o.onload=()=>{o.style.opacity="1"}},150),f(s+1),f(s-1)}function f(e){e<0&&(e=t.length-1),e>=t.length&&(e=0);const n=t[e];if(n&&n.src){const e=new Image;e.src=n.src}}function j(n){v();const o=t.indexOf(n);o!==-1&&(s(o),e.classList.add("is-open"),document.body.style.overflow="hidden")}function d(){e.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{o.src=""},350)}a.addEventListener("click",function(e){const t=e.target.closest("img");t&&l.includes(t)&&j(t)}),x.addEventListener("click",d),w.addEventListener("click",()=>s(n-1)),_.addEventListener("click",()=>s(n+1)),e.addEventListener("click",function(t){(t.target===e||t.target.classList.contains("lightbox-container"))&&d()}),document.addEventListener("keydown",function(t){if(!e.classList.contains("is-open"))return;t.key==="Escape"?d():t.key==="ArrowLeft"?s(n-1):t.key==="ArrowRight"&&s(n+1)});let m=0,h=0;e.addEventListener("touchstart",function(e){m=e.changedTouches[0].screenX},{passive:!0}),e.addEventListener("touchend",function(e){h=e.changedTouches[0].screenX,C()},{passive:!0});function C(){const e=h-m,t=60;e>t?s(n-1):e<-t&&s(n+1)}const u=document.querySelectorAll(".gallery-filters .filter-btn");u.forEach(e=>{e.addEventListener("click",function(){u.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(l.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),v(),window.Packery){const e=window.Packery.data(a);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",e):e()})()