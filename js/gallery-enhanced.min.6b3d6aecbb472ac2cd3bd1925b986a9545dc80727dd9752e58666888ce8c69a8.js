(function(){function e(){const i=document.querySelector(".gallery");if(!i)return;let t=[],n=0,M=null,A=!1;const e=document.createElement("div");e.className="gallery-lightbox",e.innerHTML=`
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
    `,document.body.appendChild(e);const o=e.querySelector(".lightbox-image"),f=e.querySelector(".lightbox-title"),p=e.querySelector(".camera-val"),y=e.querySelector(".lens-val"),x=e.querySelector(".params-val"),O=e.querySelector(".location-val"),w=e.querySelector(".date-val"),m=e.querySelector(".lightbox-music-bar"),_=e.querySelector(".music-track-name"),j=e.querySelector(".lightbox-close"),g=e.querySelector(".lightbox-prev"),v=e.querySelector(".lightbox-next"),S=e.querySelector(".btn-slideshow"),k=e.querySelector(".btn-toggle-info"),E=e.querySelectorAll(".swatch"),a=Array.from(i.querySelectorAll("img"));t=[...a];function h(){t=a.filter(e=>!e.classList.contains("filtered-out"))}function s(s){s<0&&(s=t.length-1),s>=t.length&&(s=0),n=s;const i=t[n];if(!i)return;o.style.opacity="0",setTimeout(()=>{o.src=i.src,o.alt=i.alt||"Фотография";const v=i.getAttribute("data-title")||i.alt||"Фото",u=i.getAttribute("data-camera")||"—",d=i.getAttribute("data-lens")||"—",t=i.getAttribute("data-aperture"),s=i.getAttribute("data-shutter"),n=i.getAttribute("data-iso"),h=i.getAttribute("data-location")||"—",g=i.getAttribute("data-date")||"—",r=i.getAttribute("data-music"),a=i.getAttribute("data-story");let c="—";if(t||s||n){const e=[];t&&e.push(t),s&&e.push(s),n&&e.push(`ISO ${n}`),c=e.join(" • ")}f.textContent=v,p.textContent=u,y.textContent=d,x.textContent=c,O.textContent=h,w.textContent=g;const l=e.querySelector(".lightbox-story-box"),b=e.querySelector(".story-text");a?(b.textContent=a,l.style.display="block"):l.style.display="none",r?(_.textContent=r,m.style.display="flex"):m.style.display="none",o.onload=()=>{o.style.opacity="1"}},150),d(s+1),d(s-1)}function d(e){e<0&&(e=t.length-1),e>=t.length&&(e=0);const n=t[e];if(n&&n.src){const e=new Image;e.src=n.src}}function C(n){h();const o=t.indexOf(n);o!==-1&&(s(o),e.classList.add("is-open"),document.body.style.overflow="hidden")}function r(){e.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{o.src=""},350)}i.addEventListener("click",function(e){const t=e.target.closest("img");t&&a.includes(t)&&C(t)}),j.addEventListener("click",r),g.addEventListener("click",()=>s(n-1)),v.addEventListener("click",()=>s(n+1)),e.addEventListener("click",function(t){(t.target===e||t.target.classList.contains("lightbox-container"))&&r()}),document.addEventListener("keydown",function(t){if(!e.classList.contains("is-open"))return;t.key==="Escape"?r():t.key==="ArrowLeft"?s(n-1):t.key==="ArrowRight"&&s(n+1)});let c=0,l=0;e.addEventListener("touchstart",function(e){c=e.changedTouches[0].screenX},{passive:!0}),e.addEventListener("touchend",function(e){l=e.changedTouches[0].screenX,b()},{passive:!0});function b(){const e=l-c,t=60;e>t?s(n-1):e<-t&&s(n+1)}const u=document.querySelectorAll(".gallery-filters .filter-btn");u.forEach(e=>{e.addEventListener("click",function(){u.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(a.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),h(),window.Packery){const e=window.Packery.data(i);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",e):e()})()