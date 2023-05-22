function draw_map(div_id, markers) {
  var first_draw = true;
  var map = null;
  if (window["map_" + div_id]) {
    map = window["map_" + div_id];
    first_draw = false;
  } else {
    map = L.map(div_id).setView([51.505, -0.09], 13);
    window["map_" + div_id] = map;

    L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution:
        '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(map);

    map.__airfield_group = new L.featureGroup([]);
    map.__target_group = new L.featureGroup([]);
    map.__squadron_group = new L.featureGroup([]);

    map.__blue_airfield_icon = L.icon({
      iconUrl: "/images/airfield_blue.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__red_airfield_icon = L.icon({
      iconUrl: "/images/airfield_red.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__red_target_icon = L.icon({
      iconUrl: "/images/target_red.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_target_icon = L.icon({
      iconUrl: "/images/target_blue.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_plane_icon = L.icon({
      iconUrl: "/images/plane_blue.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__red_plane_icon = L.icon({
      iconUrl: "/images/plane_red.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_ship_icon = L.icon({
      iconUrl: "/images/ship_blue.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    map.__red_ship_icon = L.icon({
      iconUrl: "/images/ship_red.png",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      popupAnchor: [0, -16],
      tooltipAnchor: [16, 0],
    });

    // display airfields
    map.__airfield_group.addTo(map);
    map.__target_group.addTo(map);
    map.__squadron_group.addTo(map);

    var overlays = {
      Airfields: map.__airfield_group,
      Targets: map.__target_group,
      Squadrons: map.__squadron_group,
    };

    // if (last_coords !== null) {
    //   map.setView(last_coords, last_zoom);
    // } else {
    //   // reset zoom:
    //   map.fitBounds(airfield_group.getBounds());
    // }

    var layerControl = L.control.layers(null, overlays).addTo(map);
  }


  // update markers:
  map.__airfield_group.clearLayers();
  map.__target_group.clearLayers();
  map.__squadron_group.clearLayers();

  markers.forEach((m) => {
    if (m.class == "FixedAirBase") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_airfield_icon
            : map.__red_airfield_icon,
      })
        .addTo(map.__airfield_group)
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
        icon: m.side.toLowerCase() == "blue" ? map.__blue_plane_icon : map.__red_plane_icon,
      })
        .addTo(map.__squadron_group)
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
          m.side.toLowerCase() == "blue" ? map.__blue_target_icon : map.__red_target_icon,
      })
        .addTo(map.__target_group)
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
        icon: m.side.toLowerCase() == "blue" ? map.__blue_ship_icon : map.__red_ship_icon,
      })
        .addTo(map.__airfield_group)
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
        .addTo(map.__target_group)
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
      ).addTo(map.__target_group);
    }
    if (m.class == "TargetRefuel") {
      L.polyline(
        [
          [m.lat, m.lon],
          [m.lat2, m.lon2],
        ],
        { color: "blue" }
      )
        .addTo(map.__target_group)
        .bindTooltip(m.name)
        .on("click", function (e) {
          fetch("https://testprotocol.example/", {
            method: "POST",
            body: JSON.stringify(m),
          });
        });
    }
  });

  if (first_draw) {
    map.fitBounds(map.__airfield_group.getBounds());
  }

}