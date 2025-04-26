
# Lighting Engine Mod Manager (WIP)

Lighting Engine Mod Manager (LEMM) is a small utility to help manage the creation and installation of mods made for the Dark Souls 2/3 Lighting Engine mod.

## Creating a mod

The app has a very convinient "Create" page for this purpose. In it, you are asked to go through 3 steps:

1. In the first step, you're asked to simply input the details of your mod into three text inputs.
    * First, the name of the mod, which is limited to 32 characters.
    * Second, the name of the author of the mod, currently limited to 16 characters.
    * Third, the version of the mod, limited to 5 characters ( major.minor.patch segmentation expected ).
2. In the second step, you're asked to simply add the files of the mod to the package. Keep in mind that textures that are supposed to go under `ds2le_atmosphere_presests/textures` should be added as Engine Textures via the dedicated button.
3. Finally, after going through step 1 and 2, in the third step you will package your mod into an archive.

## Installing and managing your mods

For this, you can go to either the Dark Souls 2 or 3 (planned) page, and then simply use the load order management buttons in there to add packages to the list, then apply them to your game as needed. You can also move each mod around in the load order.

## TODO

* [ ] Add `cfgpbr` alphabetical order loading support.
* [ ] Collect feedback & make improvements.
