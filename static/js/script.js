document.addEventListener('DOMContentLoaded',function() {
    document.querySelector('select[name="path"]').onchange=changeEventHandler;
},false);

function changeEventHandler(event) {
  url = event.target.value
  if(url) {
      $.getJSON("data/slides.json", function(json) {
      id = json.find(x => x.url == url).id
      //info = document.getElementById("slide_info");
      //info.style.display = 'block'
      //description = document.getElementById("slide_description");
      //description.innerHTML = json[id - 1].description;
      screenshot = document.getElementById("slide_screenshot");
      screenshot.setAttribute('src', json[id - 1].screenshot);
      //pres_title = document.getElementById("slide_title");
      //pres_title.innerHTML = json[id - 1].title;
    });
  }
}
