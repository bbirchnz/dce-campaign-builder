function drawmap(div_id, markers) {
  var map = L.map(div_id).setView([51.505, -0.09], 13);

  L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution:
      '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
  }).addTo(map);

  var airfield_group = new L.featureGroup([]);

  markers.forEach((m) => {
    mark = L.marker([m.y, m.x])
      .addTo(airfield_group)
      .bindPopup(m.name)
      .on("click", function (e) {
        fetch("https://testprotocol.example/", {
          method: "POST",
          body: JSON.stringify(m),
        });
      });
  });
  // display airfields
  airfield_group.addTo(map);

  var overlays = {
    Airfields: airfield_group,
  };

  // reset zoom:
  map.fitBounds(airfield_group.getBounds());

  var layerControl = L.control.layers(null, overlays).addTo(map);
}
