# Loadout Rules

Not all loadouts are suitable for all targets and day/night operations - for example LGBs can't be used in adverse weather (can't see the target!)

Here are some rules that can be auto applied by sensible naming in the mission editor (place these in the loadout name):

| Loadout String | Target Class | Overrides                                                                   | Attributes              |
| -------------- | ------------ | --------------------------------------------------------------------------- | ----------------------- |
| " day "        | Strike       | night=false<br>adverse=false                                                |                         |
| " lgb "        | Strike       | adverse=false<br>weapon_type="Guided Bombs"<br>capability=2<br>             | precise                 |
| " cbu "        | Strike       |                                                                             | soft<br>parked_aircraft |
| " jdam "       | Strike       | adverse=true<br>weapon_type="Guided Bombs"<br>capability=2<br>sortie_rate=3 | precise                 |
| " rockets "    | Strike       | adverse=false<br>weapon_type="Rockets"                                      | soft<br>parked_aircraft |
| " saturation " | Strike       | capability=10<br>sortie_rate=1                                              | saturation              |

Multiple rules can be provided, and will be applied in the order listed above