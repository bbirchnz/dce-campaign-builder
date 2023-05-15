function drawmap(div_id, markers) {
  var map = L.map(div_id).setView([51.505, -0.09], 13);

  L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution:
      '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
  }).addTo(map);

  var airfield_group = new L.featureGroup([]);
  var target_group = new L.featureGroup([]);

  var blue_airfield_icon = L.icon({
    iconUrl: "/images/airfield_blue.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  var red_airfield_icon = L.icon({
    iconUrl: "/images/airfield_red.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  var red_target_icon = L.icon({
    iconUrl: "/images/target_red.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  var blue_target_icon = L.icon({
    iconUrl: "/images/target_blue.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  markers.forEach((m) => {
    if (m.class == "FixedAirBase") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? blue_airfield_icon
            : red_airfield_icon,
      })
        .addTo(airfield_group)
        .bindTooltip(m.name)
        .on("click", function (e) {
          fetch("https://testprotocol.example/", {
            method: "POST",
            body: JSON.stringify(m),
          });
        });
    }
    if (m.class == "TargetStrike" || m.class == "TargetAntiShipStrike") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue" ? blue_target_icon : red_target_icon,
      })
        .addTo(target_group)
        .bindTooltip(m.name)
        .on("click", function (e) {
          fetch("https://testprotocol.example/", {
            method: "POST",
            body: JSON.stringify(m),
          });
        });
    }

    

    if (m.class == "TargetCAP") {
      L.corridor(
        [
          [m.lat, m.lon],
          [m.lat2, m.lon2],
        ],
        {
          color: m.side == "blue" ? "#0066ff44" : "#ff330044",
          corridor: m.radius, // meters
          className: "route-corridor",
        }
      )
        .addTo(target_group)
        .bindTooltip(m.name)
        .on("click", function (e) {
          fetch("https://testprotocol.example/", {
            method: "POST",
            body: JSON.stringify(m),
          });
        });
      console.log(JSON.stringify(m));
    }
    if (m.class == "TargetRefuel") {
      L.polyline(
        [
          [m.lat, m.lon],
          [m.lat2, m.lon2],
        ],
        { color: "blue" }
      )
        .addTo(target_group)
        .bindTooltip(m.name)
        .on("click", function (e) {
          fetch("https://testprotocol.example/", {
            method: "POST",
            body: JSON.stringify(m),
          });
        });
    }
  });
  // display airfields
  airfield_group.addTo(map);
  target_group.addTo(map);

  var overlays = {
    Airfields: airfield_group,
    Targets: target_group,
  };

  // reset zoom:
  map.fitBounds(airfield_group.getBounds());

  var layerControl = L.control.layers(null, overlays).addTo(map);
}
