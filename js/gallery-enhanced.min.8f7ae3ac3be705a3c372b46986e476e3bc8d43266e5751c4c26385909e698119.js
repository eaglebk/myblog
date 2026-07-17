(function(){function e(){const i=document.querySelector(".gallery");if(!i)return;let t=[],n=0;const e=document.createElement("div");e.className="gallery-lightbox",e.innerHTML=`
      <button class="lightbox-close" aria-label="Закрыть">&times;</button>
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
    `,document.body.appendChild(e);const o=e.querySelector(".lightbox-image"),x=e.querySelector(".lightbox-title"),f=e.querySelector(".camera-val"),w=e.querySelector(".lens-val"),C=e.querySelector(".params-val"),_=e.querySelector(".location-val"),j=e.querySelector(".date-val"),u=e.querySelector(".lightbox-music-bar"),b=e.querySelector(".music-track-name"),v=e.querySelector(".lightbox-close"),g=e.querySelector(".lightbox-prev"),p=e.querySelector(".lightbox-next"),a=Array.from(i.querySelectorAll("img"));t=[...a];function m(){t=a.filter(e=>!e.classList.contains("filtered-out"))}function s(s){s<0&&(s=t.length-1),s>=t.length&&(s=0),n=s;const i=t[n];if(!i)return;o.style.opacity="0",setTimeout(()=>{o.src=i.src,o.alt=i.alt||"Фотография";const l=i.getAttribute("data-title")||i.alt||"Фото",d=i.getAttribute("data-camera")||"—",h=i.getAttribute("data-lens")||"—",t=i.getAttribute("data-aperture"),n=i.getAttribute("data-shutter"),s=i.getAttribute("data-iso"),m=i.getAttribute("data-date")||"—",a=i.getAttribute("data-music"),r=i.getAttribute("data-story");let p="—";if(t||n||s){const e=[];t&&e.push(t),n&&e.push(n),s&&e.push(`ISO ${s}`),p=e.join(" • ")}x.textContent=l,f.textContent=d,w.textContent=h,_.textContent=location,j.textContent=m;const c=e.querySelector(".lightbox-story-box"),g=e.querySelector(".story-text");r?(g.textContent=r,c.style.display="block"):c.style.display="none",a?(b.textContent=a,u.style.display="flex"):u.style.display="none",o.onload=()=>{o.style.opacity="1"}},150),d(s+1),d(s-1)}function d(e){e<0&&(e=t.length-1),e>=t.length&&(e=0);const n=t[e];if(n&&n.src){const e=new Image;e.src=n.src}}function y(n){m();const o=t.indexOf(n);o!==-1&&(s(o),e.classList.add("is-open"),document.body.style.overflow="hidden")}function r(){e.classList.remove("is-open"),document.body.style.overflow="",setTimeout(()=>{o.src=""},350)}i.addEventListener("click",function(e){const t=e.target.closest("img");t&&a.includes(t)&&y(t)}),v.addEventListener("click",r),g.addEventListener("click",()=>s(n-1)),p.addEventListener("click",()=>s(n+1)),e.addEventListener("click",function(t){(t.target===e||t.target.classList.contains("lightbox-container"))&&r()}),document.addEventListener("keydown",function(t){if(!e.classList.contains("is-open"))return;t.key==="Escape"?r():t.key==="ArrowLeft"?s(n-1):t.key==="ArrowRight"&&s(n+1)});let l=0,c=0;e.addEventListener("touchstart",function(e){l=e.changedTouches[0].screenX},{passive:!0}),e.addEventListener("touchend",function(e){c=e.changedTouches[0].screenX,O()},{passive:!0});function O(){const e=c-l,t=60;e>t?s(n-1):e<-t&&s(n+1)}const h=document.querySelectorAll(".gallery-filters .filter-btn");h.forEach(e=>{e.addEventListener("click",function(){h.forEach(e=>e.classList.remove("is-active")),this.classList.add("is-active");const e=this.getAttribute("data-filter");if(a.forEach(t=>{const n=t.getAttribute("data-category");e==="all"||n===e?(t.classList.remove("filtered-out"),t.style.display=""):(t.classList.add("filtered-out"),t.style.display="none")}),m(),window.Packery){const e=window.Packery.data(i);e&&(e.reloadItems(),e.layout())}})})}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",e):e()})()