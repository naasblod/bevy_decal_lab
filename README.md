lab for contact projective decals, from Alexander Sannikovs talk on the rendering techniques of Path of Exile

Uses the difference between the depth of the fragment in the mesh (quad) and the depth for the fragment in the depth buffer as a depth texture for the parallax mapping technique.

The depth test is disabled so that decals that intersect with other geometry can be smoothly faded instead of culled. This solves the problem of quad meshes on top of non flat surfaces in a nice way in my opinion.

This lab uses one step approximation parallax mapping, I think Alexander talked about parallax occlusion mapping in his talk ("A technique similar to parallax occlusion mapping"), maybe that needs some research. I have a feeling (literally only a feeling) that POM might eliminate the windowing issue that occurs when the quad is above the surface.

todos:
* try this with a moving camera and hope nothing breaks.
* figure out if parallax occlusion mapping would be better.

Collaborated with NiseVoid

uv checker map from https://github.com/Arahnoid/UVChecker-map
