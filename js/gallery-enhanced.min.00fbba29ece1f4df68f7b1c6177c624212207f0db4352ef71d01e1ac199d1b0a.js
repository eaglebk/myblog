(function(){function e(){const u=document.querySelector(".gallery");if(!u)return;let t=[],n=0,l=null,r=!1,s=null;function h(){if(s&&clearTimeout(s),!e.classList.contains("is-open")||!e.classList.contains("meta-hidden")){e.classList.remove("controls-hidden");return}e.classList.remove("controls-hidden"),s=setTimeout(()=>{e.classList.contains("is-open")&&e.classList.contains("meta-hidden")&&e.classList.add("controls-hidden")},2500)}const e=document.createElement("div");e.className="gallery-lightbox",e.innerHTML=`
      <div class="lightbox-toolbar">
        <div class="toolbar-left">
          <button class="toolbar-btn btn-slideshow" aria-label="Запустить слайд-шоу" title="Запустить слайд-шоу">▶️</button>
          <button class="toolbar-btn btn-toggle-info" aria-label="Скрыть/показать описание" title="Скрыть описание">👁️</button>
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
        <div class="lightbox-color-bar">
          <div class="color-swatches">
            <button class="swatch" data-color="#ffffff" style="background-color: #ffffff;" title="Белый"></button>
            <button class="swatch" data-color="#e5e7eb" style="background-color: #e5e7eb;" title="Светло-серый"></button>
            <button class="swatch" data-color="#7f7f7f" style="background-color: #7f7f7f;" title="Серый"></button>
            <button class="swatch" data-color="#374151" style="background-color: #374151;" title="Тёмно-серый"></button>
            <button class="swatch is-active" data-color="#18181b" style="background-color: #18181b;" title="Угольный (по умолчанию)"></button>
            <button class="swatch" data-color="#000000" style="background-color: #000000;" title="Чёрный"></button>
          </div>
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
    `,document.body.appendChild(e);const a=e.querySelector(".lightbox-image"),N=e.querySelector(".lightbox-title"),z=e.querySelector(".camera-val"),T=e.querySelector(".lens-val"),F=e.querySelector(".params-val"),S=e.querySelector(".location-val"),A=e.querySelector(".date-val"),g=e.querySelector(".lightbox-music-bar"),E=e.querySelector(".music-track-name"),x=e.querySelector(".lightbox-close"),w=e.querySelector(".lightbox-prev"),O=e.querySelector(".lightbox-next"),c=e.querySelector(".btn-slideshow"),i=e.querySelector(".btn-toggle-info"),y=e.querySelectorAll(".swatch"),d=Array.from(u.querySelectorAll("img"));t=[...d];function v(){t=d.filter(e=>!e.classList.contains("filtered-out"))}function C(){if(r)return;r=!0,c.textContent="⏸️",c.setAttribute("title","Приостановить слайд-шоу"),l=setInterval(()=>{o(n+1,!0)},3500)}function f(){if(!r)return;r=!1,c.textContent="▶️",c.setAttribute("title","Запустить слайд-шоу"),l&&(clearInterval(l),l=null)}function k(){r?f():C()}function o(s,o=!1){o||f(),s<0&&(s=t.length-1),s>=t.length&&(s=0),n=s;const i=t[n];if(!i)return;a.style.opacity="0",setTimeout(()=>{a.src=i.src,a.alt=i.alt||"Фотография";const f=i.getAttribute("data-title")||i.alt||"Фото",u=i.getAttribute("data-camera")||"—",d=i.getAttribute("data-lens")||"—",t=i.getAttribute("data-aperture"),s=i.getAttribute("data-shutter"),n=i.getAttribute("data-iso"),h=i.getAttribute("data-location")||"—",m=i.getAttribute("data-date")||"—",r=i.getAttribute("data-music"),o=i.getAttribute("data-story");let c="—";if(t||s||n){const e=[];t&&e.push(t),s&&e.push(s),n&&e.push(`ISO ${n}`),c=e.join(" • ")}N.textContent=f,z.textContent=u,T.textContent=d,F.textContent=c,S.textContent=h,A.textContent=m;const l=e.querySelector(".lightbox-story-box"),p=e.querySelector(".story-text");o?(p.textContent=o,l.style.display="block"):l.style.display="none",r?(E.textContent=r,g.style.display="flex"):g.style.display="none",a.onload=()=>{a.style.opacity="1"}},150),b(s+1),b(s-1)}function b(e){e<0&&(e=t.length-1),e>=t.length&&(e=0);const n=t[e];if(n&&n.src){const e=new Image;e.src=n.src}}function M(n){v();const s=t.indexOf(n);s!==-1&&(o(s),e.classList.add("is-open"),document.body.style.overflow="hidden")}function m(){f(),s&&(clearTimeout(s),s=null),e.classList.remove("controls-hidden"),e.classList.remove("meta-hidden"),i.textContent="👁️",i.setAttribute("title","Скрыть описание"),e.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{a.src=""},350)}u.addEventListener("click",function(e){const t=e.target.closest("img");t&&d.includes(t)&&M(t)}),x.addEventListener("click",m),w.addEventListener("click",()=>o(n-1)),O.addEventListener("click",()=>o(n+1)),c.addEventListener("click",e=>{e.stopPropagation(),k()}),i.addEventListener("click",t=>{t.stopPropagation(),e.classList.toggle("meta-hidden");const n=e.classList.contains("meta-hidden");n?(i.textContent="👁️‍🗨️",i.setAttribute("title","Показать описание"),h()):(i.textContent="👁️",i.setAttribute("title","Скрыть описание"),s&&(clearTimeout(s),s=null),e.classList.remove("controls-hidden"))}),y.forEach(t=>{t.addEventListener("click",n=>{n.stopPropagation(),y.forEach(e=>e.classList.remove("is-active")),t.classList.add("is-active");const s=t.getAttribute("data-color");e.style.setProperty("--lightbox-bg",s);const o=["#ffffff","#e5e7eb"].includes(s);o?e.classList.add("light-bg"):e.classList.remove("light-bg")})}),e.addEventListener("click",function(t){(t.target===e||t.target.classList.contains("lightbox-container"))&&m()}),e.addEventListener("mousemove",h),e.addEventListener("touchstart",h,{passive:!0}),document.addEventListener("keydown",function(t){if(!e.classList.contains("is-open"))return;t.key==="Escape"?m():t.key==="ArrowLeft"?o(n-1):t.key==="ArrowRight"&&o(n+1)});let j=0,p=0;e.addEventListener("touchstart",function(e){j=e.changedTouches[0].screenX},{passive:!0}),e.addEventListener("touchend",function(e){p=e.changedTouches[0].screenX,D()},{passive:!0});function D(){const e=p-j,t=60;e>t?o(n-1):e<-t&&o(n+1)}const _=document.querySelectorAll(".gallery-filters .filter-btn");_.forEach(e=>{e.addEventListener("click",function(){_.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(d.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),v(),window.Packery){const e=window.Packery.data(u);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",e):e()})()