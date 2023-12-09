# Loadout Rules

Not all loadouts are suitable for all targets and day/night operations - for example LGBs can't be used in adverse weather (can't see the target!)

Here are some rules that can be auto applied by sensible naming in the mission editor (place these in the loadout name):

| Loadout String | Target Class | Overrides                                                                                                   | Attributes              |
| -------------- | ------------ | ----------------------------------------------------------------------------------------------------------- | ----------------------- |
| " day "        | Strike       | night=false<br>adverse=false                                                                                |                         |
| " lgb "        | Strike       | adverse=false<br>weapon_type="Guided Bombs"<br>capability=2<br>standoff=9000<br>expend=Auto                 | precise                 |
| " cbu "        | Strike       |                                                                                                             | soft<br>parked_aircraft |
| " jdam "       | Strike       | adverse=true<br>weapon_type="Guided Bombs"<br>capability=2<br>standoff=9000<br>sortie_rate=3<br>expend=Auto | precise                 |
| " rockets "    | Strike       | adverse=false<br>weapon_type="Rockets"                                                                      | soft<br>parked_aircraft |
| " saturation " | Strike       | capability=10<br>sortie_rate=1<br>firepower=4<br>range=1000000<br>standoff=150000<br>weapon_type=ASM        | saturation              |

Multiple rules can be provided, and will be applied in the order listed above