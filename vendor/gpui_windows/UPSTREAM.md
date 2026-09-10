# gpui_windows provenance

This directory is a minimal, repo-owned copy of `crates/gpui_windows` from
[`zeronsh/zui`](https://github.com/zeronsh/zui) commit
`07fd941ad72e7edc812fed317aab66adb69fa8cc`. The upstream crate is Apache-2.0;
`LICENSE-APACHE` is preserved verbatim.

The copy exists because that exact revision's GPUI CPU scene layout contains
`EdgeFadeParams` in `Quad` and `PolychromeSprite`, while its DirectX HLSL omits
the field. In `PolychromeSprite` this also shifts `AtlasTile`, so the shader
reads invalid atlas metadata.

Zeron-local changes are intentionally limited to:

- a standalone manifest whose GPUI-family dependencies retain the exact same
  zui URL and revision;
- matching DirectX HLSL declarations and the same squared, four-edge fade used
  by the pinned Metal and WGPU shaders;
- focused CPU/HLSL layout and shader-use regression tests.

The Windows renderer still deliberately ignores `BackdropBlur` paint
operations. This patch does not claim or emulate backdrop blur; Windows keeps
its existing opaque/transparent composition behavior until a real DirectX
implementation is designed and tested.
