function drawmap(div_id, markers) {
  last_coords = null;
  last_zoom = null;

  if (window["map_" + div_id] && window["map_" + div_id].remove) {
    last_coords = window["map_" + div_id].getCenter();
    last_zoom = window["map_" + div_id].getZoom();
    window["map_" + div_id].off();
    window["map_" + div_id].remove();
  }

  var map = L.map(div_id).setView([51.505, -0.09], 13);
  window["map_" + div_id] = map;

  L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution:
      '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
  }).addTo(map);

  var airfield_group = new L.featureGroup([]);
  var target_group = new L.featureGroup([]);
  var squadron_group = new L.featureGroup([]);

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

  var blue_plane_icon = L.icon({
    iconUrl: "/images/plane_blue.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  var red_plane_icon = L.icon({
    iconUrl: "/images/plane_red.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  var blue_ship_icon = L.icon({
    iconUrl: "/images/ship_blue.png",
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
    tooltipAnchor: [16, 0],
  });

  var red_ship_icon = L.icon({
    iconUrl: "/images/ship_red.png",
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

    if (m.class == "Squadron") {
      L.marker([m.lat, m.lon], {
        icon: m.side.toLowerCase() == "blue" ? blue_plane_icon : red_plane_icon,
      })
        .addTo(squadron_group)
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
        .bindTooltip(m.name, { permanent: false, opacity: 1.0 })
        .on("click", function (e) {
          fetch("https://testprotocol.example/", {
            method: "POST",
            body: JSON.stringify(m),
          });
        });
    }

    if (m.class == "ShipAirBase") {
      L.marker([m.lat, m.lon], {
        icon: m.side.toLowerCase() == "blue" ? blue_ship_icon : red_ship_icon,
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
      L.polyline(
        [
          [m.lat, m.lon],
          [m.lat2, m.lon2],
        ],
        { color: m.side == "blue" ? "#0066dd44" : "#dd330044" }
      ).addTo(target_group);
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
  squadron_group.addTo(map);

  var overlays = {
    Airfields: airfield_group,
    Targets: target_group,
    Squadrons: squadron_group,
  };

  if (last_coords !== null) {
    map.setView(last_coords, last_zoom);
  } else {
    // reset zoom:
    map.fitBounds(airfield_group.getBounds());
  }

  var layerControl = L.control.layers(null, overlays).addTo(map);
}
