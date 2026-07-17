(function(){function e(){const l=document.querySelector(".gallery");if(!l)return;let t=[],n=0,c=null,i=!1;const e=document.createElement("div");e.className="gallery-lightbox",e.innerHTML=`
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
    `,document.body.appendChild(e);const o=e.querySelector(".lightbox-image"),z=e.querySelector(".lightbox-title"),y=e.querySelector(".camera-val"),F=e.querySelector(".lens-val"),M=e.querySelector(".params-val"),S=e.querySelector(".location-val"),k=e.querySelector(".date-val"),f=e.querySelector(".lightbox-music-bar"),E=e.querySelector(".music-track-name"),x=e.querySelector(".lightbox-close"),w=e.querySelector(".lightbox-prev"),_=e.querySelector(".lightbox-next"),r=e.querySelector(".btn-slideshow"),a=e.querySelector(".btn-toggle-info"),p=e.querySelectorAll(".swatch"),d=Array.from(l.querySelectorAll("img"));t=[...d];function g(){t=d.filter(e=>!e.classList.contains("filtered-out"))}function O(){if(i)return;i=!0,r.textContent="⏸️",r.setAttribute("title","Приостановить слайд-шоу"),c=setInterval(()=>{s(n+1,!0)},3500)}function u(){if(!i)return;i=!1,r.textContent="▶️",r.setAttribute("title","Запустить слайд-шоу"),c&&(clearInterval(c),c=null)}function C(){i?u():O()}function s(s,i=!1){i||u(),s<0&&(s=t.length-1),s>=t.length&&(s=0),n=s;const a=t[n];if(!a)return;o.style.opacity="0",setTimeout(()=>{o.src=a.src,o.alt=a.alt||"Фотография";const p=a.getAttribute("data-title")||a.alt||"Фото",u=a.getAttribute("data-camera")||"—",d=a.getAttribute("data-lens")||"—",t=a.getAttribute("data-aperture"),s=a.getAttribute("data-shutter"),n=a.getAttribute("data-iso"),h=a.getAttribute("data-location")||"—",m=a.getAttribute("data-date")||"—",r=a.getAttribute("data-music"),i=a.getAttribute("data-story");let c="—";if(t||s||n){const e=[];t&&e.push(t),s&&e.push(s),n&&e.push(`ISO ${n}`),c=e.join(" • ")}z.textContent=p,y.textContent=u,F.textContent=d,M.textContent=c,S.textContent=h,k.textContent=m;const l=e.querySelector(".lightbox-story-box"),g=e.querySelector(".story-text");i?(g.textContent=i,l.style.display="block"):l.style.display="none",r?(E.textContent=r,f.style.display="flex"):f.style.display="none",o.onload=()=>{o.style.opacity="1"}},150),m(s+1),m(s-1)}function m(e){e<0&&(e=t.length-1),e>=t.length&&(e=0);const n=t[e];if(n&&n.src){const e=new Image;e.src=n.src}}function A(n){g();const o=t.indexOf(n);o!==-1&&(s(o),e.classList.add("is-open"),document.body.style.overflow="hidden")}function h(){u(),e.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{o.src=""},350)}l.addEventListener("click",function(e){const t=e.target.closest("img");t&&d.includes(t)&&A(t)}),x.addEventListener("click",h),w.addEventListener("click",()=>s(n-1)),_.addEventListener("click",()=>s(n+1)),r.addEventListener("click",e=>{e.stopPropagation(),C()}),a.addEventListener("click",t=>{t.stopPropagation(),e.classList.toggle("meta-hidden");const n=e.classList.contains("meta-hidden");n?(a.textContent="👁️‍🗨️",a.setAttribute("title","Показать описание")):(a.textContent="👁️",a.setAttribute("title","Скрыть описание"))}),p.forEach(t=>{t.addEventListener("click",n=>{n.stopPropagation(),p.forEach(e=>e.classList.remove("is-active")),t.classList.add("is-active");const s=t.getAttribute("data-color");e.style.setProperty("--lightbox-bg",s);const o=["#ffffff","#e5e7eb"].includes(s);o?e.classList.add("light-bg"):e.classList.remove("light-bg")})}),e.addEventListener("click",function(t){(t.target===e||t.target.classList.contains("lightbox-container"))&&h()}),document.addEventListener("keydown",function(t){if(!e.classList.contains("is-open"))return;t.key==="Escape"?h():t.key==="ArrowLeft"?s(n-1):t.key==="ArrowRight"&&s(n+1)});let v=0,b=0;e.addEventListener("touchstart",function(e){v=e.changedTouches[0].screenX},{passive:!0}),e.addEventListener("touchend",function(e){b=e.changedTouches[0].screenX,T()},{passive:!0});function T(){const e=b-v,t=60;e>t?s(n-1):e<-t&&s(n+1)}const j=document.querySelectorAll(".gallery-filters .filter-btn");j.forEach(e=>{e.addEventListener("click",function(){j.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(d.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),g(),window.Packery){const e=window.Packery.data(l);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",e):e()})()