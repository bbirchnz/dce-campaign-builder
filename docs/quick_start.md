## Quick start

Prerequisites: install [DCE Manager](https://forum.dcs.world/topic/162712-dce-campaigns). Once installed, open it and on the update tab install Scriptsmod.

### Create a new DCS Mission.

This example mission can be found [here](/dce-lib/test_resources/base_mission.miz).

1. Set Airbases you want as red and blue by clicking on them and chosing the coalition.
1. Create Air groups to represent each squadron.
    1. Group name = Squadron name
    1. For each task you want the squadron to do, create a unit in this group and name like:
        `<SQN NAME>_<TASK>_<Loadout name>`

        You can set liveries here as well, each unit can have a different livery.
    1. Set `LATE ACTIVATION`
    1. Leave only the initial waypoint as air start. Place it near the desired starting base. DCE Campaign Builder will snap it to the nearest airbase.

        ![squadron example](/docs/squadron_example_cap.png)

1. Create Ground Groups. If you want them as an assignable strike target they must be named `STRIKE_<Desired Target Name>`

    Individual units within the group can be named as desired - their names will show up in the briefing.

    Don't set them as late activation, but you may want to select `HIDDEN ON..` as desired.

    ![vehicle example](/docs/strike_vehicle_group_example.png)

1. Place ship groups. All ship groups will be created as targets.

1. Carrier Groups are just ship groups with the Carrier unit named `CV_<Carrier name>` 

    Create a big long path for the carrier to travel. DCE will give you in-mission F10 radio commands
    to order the carrier into the wind if required.

    ![carrier example](/docs/carrier_example.png)

1. Place zones to mark CAP, AWACS and Refueling orbits, and airstart locations.

    `<SIDE>_<TASK>_<axis 0-360>_<radius, km>`

    See image for examples:

    ![zone example](/docs/zones_example.png)

1. Static targets on existing map objects can be defined with zones:

    `<SIDE>_STATICSTRIKE_<Target group name>_<Target item name>`

    ![zone static example](/docs/zone_static_targets.png)

1. And finally, static targets can be placed, and will be grouped as targets:

    `STATICSTRIKE_<Target group name>_<Target item name>`

    ![static object example](/docs/static_target_example.png)

### DCE Campaign Editor

1. Open the editor and click the `new` button in the top left - select the dcs mission you've just created

    ![create new](/docs/dcecb_new.png)

1. Click on items on the map, or explore the menus on the left to adjust settings for your campaign. You can set dawn/dusk times, startup times, loadout ranges, cruise altitudes and much more. You can mark targets as inactive and set priorities. 

    Default values are designed to be sensible and a good starting point for a working campaign. 

    The triggers and actions are one of the most amazing features of DCE, and this UI will be improved to make them more accessible:

    ![triggers](/docs/dcecb_triggers.png)

1. Finally, click generate to build a zip file with all required files for use with `DCE Manager`

    ![generate zip](/docs/dcecb_generate.png)

1. Open in `DCE Manager`. Make sure that DCE Manager has Scriptsmod installed (update tab) - DCE Campaign Builder does not package all these scripts with it.

    You will likely see a warning after clicking install campaign about missing files - this is due to the above missing scripts and is not a problem. 

1. Hit start campaign and follow the instructions.

    ![DCE Manager start](/docs/dce_manager_start.png)

    You're good to go! Open DCS, go to campaigns and enjoy!

