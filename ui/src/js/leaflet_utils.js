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

    map.__blue_airfield_icon = L.divIcon({
      html: "<img src='/images/airfield_fixed.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_airfield_icon = L.divIcon({
      html: "<img src='/images/airfield_fixed.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_farp_icon = L.divIcon({
      html: "<img src='/images/airfield_farp.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_farp_icon = L.divIcon({
      html: "<img src='/images/airfield_farp.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_airstart_icon = L.divIcon({
      html: "<img src='/images/airfield_airstart.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_airstart_icon = L.divIcon({
      html: "<img src='/images/airfield_airstart.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_target_strike_icon = L.divIcon({
      html: "<img src='/images/target_strike.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_target_strike_icon = L.divIcon({
      html: "<img src='/images/target_strike.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_target_ship_icon = L.divIcon({
      html: "<img src='/images/target_ship.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_target_ship_icon = L.divIcon({
      html: "<img src='/images/target_ship.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_plane_icon = L.divIcon({
      html: "<img src='/images/plane.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_plane_icon = L.divIcon({
      html: "<img src='/images/plane.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__blue_ship_icon = L.divIcon({
      html: "<img src='/images/airfield_ship.svg'/>",
      className: "svg_blue marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
      tooltipAnchor: [16, 0],
    });

    map.__red_ship_icon = L.divIcon({
      html: "<img src='/images/airfield_ship.svg'/>",
      className: "svg_red marker_no_bg",
      iconSize: [32, 32],
      iconAnchor: [16, 16],
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

    var layerControl = L.control.layers(null, overlays).addTo(map);
  }

  // update markers:
  map.__airfield_group.clearLayers();
  map.__target_group.clearLayers();
  map.__squadron_group.clearLayers();

  markers.forEach((m) => {
    if (m.class == "dce_lib::db_airbases::FixedAirBase" && m.side != "neutral") {
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

    if (m.class == "dce_lib::db_airbases::AirStartBase") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_airstart_icon
            : map.__red_airstart_icon,
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

    if (m.class == "dce_lib::db_airbases::FarpBase") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_farp_icon
            : map.__red_farp_icon,
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

    if (m.class == "dce_lib::oob_air::OobAir") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_plane_icon
            : map.__red_plane_icon,
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
    if (m.class == "dce_lib::targets::strike::Strike") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_target_strike_icon
            : map.__red_target_strike_icon,
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
    if (m.class == "dce_lib::targets::anti_ship::AntiShipStrike") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_target_ship_icon
            : map.__red_target_ship_icon,
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

    if (m.class == "dce_lib::db_airbases::ShipBase") {
      L.marker([m.lat, m.lon], {
        icon:
          m.side.toLowerCase() == "blue"
            ? map.__blue_ship_icon
            : map.__red_ship_icon,
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

    if (m.class == "dce_lib::targets::cap::CAP") {
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
    if (m.class == "dce_lib::targets::refueling::Refueling") {
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

    if (m.class == "dce_lib::targets::awacs::AWACS") {
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
