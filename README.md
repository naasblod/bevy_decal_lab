# Contact Projective Decals
from Alexander Sannikovs talk on the rendering techniques of Path of Exile

now there's a library:
https://github.com/naasblod/bevy_contact_projective_decals/tree/main

<img width="856" alt="Image of a bunch of decals being projected on top of a bunch of boxes and stuff" src="https://github.com/naasblod/bevy_decal_lab/assets/51246882/85b17493-2428-41a1-9b54-83bf192fdc0a">

### Description
Uses the difference between the depth of the fragment in the mesh (quad) and the depth for the fragment in the depth buffer as a depth texture for the parallax mapping technique.

The depth test is disabled so that decals that intersect with other geometry can be smoothly faded instead of culled. This solves the problem of quad meshes on top of non flat surfaces in a nice way in my opinion.

This lab uses one step approximation parallax mapping, I think Alexander talked about parallax occlusion mapping in his talk ("A technique similar to parallax occlusion mapping"), maybe that needs some research. I have a feeling (literally only a feeling) that POM might eliminate the windowing issue that occurs when the quad is above the surface.

In it's current state I think it's usable for fixed angle cameras since the effect will be deterministic, if you were to change the view vector the parts of the decal that are offset by geometry will shift (probably a skill issue on my part though). Further work is needed for being able to handle really steep angles when looking at the quad.

### todos:
* figure out if parallax occlusion mapping would be better.
* figure out if it's possible to not have the world offset shift when rotating the camera

Super big thanks to NiseVoid and Griffin

uv checker map from https://github.com/Arahnoid/UVChecker-map
