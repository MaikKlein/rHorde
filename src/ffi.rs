use std::libc::*;
#[link_args = "-lHorde3D"]
#[fixed_stack_segment]
extern "C" {
 pub fn h3dInit() -> bool;
 pub fn h3dGetVersionString() -> *c_char;
 pub fn h3dCheckExtension(extension_name: *c_char)->bool;
 pub fn h3dGetError() -> bool;
 pub fn h3dRelease();
 pub fn h3dRender(camera_node: H3DNode);
 pub fn h3dFinalizeFrame();
 pub fn h3dClear();
 pub fn h3dGetOption(param: H3DOptions) -> c_float;
 pub fn h3dSetOption(param: H3DOptions, value: c_float)->bool;
 pub fn h3dGetStat(param: H3DStats, reset: bool) -> c_float;
}

pub type H3DRes = c_int;
pub type H3DNode = c_int;
pub static H3DRootNode : H3DNode = 1;

enum H3DOptions	
{
  MaxLogLevel = 1,
  MaxNumMessages,
  TrilinearFiltering,
  MaxAnisotropy,
  TexCompression,
  SRGBLinearization,
  LoadTextures,
  FastAnimation,
  ShadowMapSize,
  SampleCount,
  WireframeMode,
  DebugViewMode,
  DumpFailedShaders,
  GatherTimeStats
}
enum H3DStats
{
  TriCount = 100,
  BatchCount,
  LightPassCount,
  FrameTime,
  AnimationTime,
  GeoUpdateTime,
  ParticleSimTime,
  FwdLightsGPUTime,
  DefLightsGPUTime,
  ShadowsGPUTime,
  ParticleGPUTime,
  TextureVMem,
  GeometryVMem
}
/*
enum H3dResTypes	{
  Undefined = 0,
  SceneGraph,
  Geometry,
  Animation,
  Material,
  Code,
  Shader,
  Texture,
  ParticleEffect,
  Pipeline
}
enum H3DShaderRes	{
  ContextElem = 600,
  SamplerElem,
  UniformElem,
  ContNameStr,
  SampNameStr,
  SampDefTexResI,
  UnifNameStr,
  UnifSizeI,
  UnifDefValueF4
}
enum H3DMatRes	{
  MaterialElem = 400,
  SamplerElem,
  UniformElem,
  MatClassStr,
  MatLinkI,
  MatShaderI,
  SampNameStr,
  SampTexResI,
  UnifNameStr,
  UnifValueF4
}
enum H3DAnimRes	{
  EntityElem = 300,
  EntFrameCountI
}
enum H3DGeoRes	{
  GeometryElem = 200,
  GeoIndexCountI,
  GeoVertexCountI,
  GeoIndices16I,
  GeoIndexStream,
  GeoVertPosStream,
  GeoVertTanStream,
  GeoVertStaticStream
}
enum H3DFormats	{
  Unknown = 0,
  TEX_BGRA8,
  TEX_DXT1,
  TEX_DXT3,
  TEX_DXT5,
  TEX_RGBA16F,
  TEX_RGBA32F
}
enum H3DResFlags	{
  NoQuery = 1,
  NoTexCompression = 2,
  NoTexMipmaps = 4,
  TexCubemap = 8,
  TexDynamic = 16,
  TexRenderable = 32,
  TexSRGB = 64
}
enum H3DPartEffRes	{
  ParticleElem = 800,
  ChanMoveVelElem,
  ChanRotVelElem,
  ChanSizeElem,
  ChanColRElem,
  ChanColGElem,
  ChanColBElem,
  ChanColAElem,
  PartLifeMinF,
  PartLifeMaxF,
  ChanStartMinF,
  ChanStartMaxF,
  ChanEndRateF,
  ChanDragElem
}
enum H3DPipeRes	{
  StageElem = 900,
  StageNameStr,
  StageActivationI
}
enum H3DNodeTypes	{
  Undefined = 0,
  Group,
  Model,
  Mesh,
  Joint,
  Light,
  Camera,
  Emitter
}
enum H3DNodeFlags	{
  NoDraw = 1,
  NoCastShadow = 2,
  NoRayQuery = 4,
  Inactive = 7  // NoDraw | NoCastShadow | NoRayQuery
}
enum H3DNodeParams	{
  NameStr = 1,
  AttachmentStr
}
enum H3DModel	{
  GeoResI = 200,
  SWSkinningI,
  LodDist1F,
  LodDist2F,
  LodDist3F,
  LodDist4F,
  AnimCountI
}
enum H3DMesh	{
  MatResI = 300,
  BatchStartI,
  BatchCountI,
  VertRStartI,
  VertREndI,
  LodLevelI
}
enum H3DLight	{
  MatResI = 500,
  RadiusF,
  FovF,
  ColorF3,
  ColorMultiplierF,
  ShadowMapCountI,
  ShadowSplitLambdaF,
  ShadowMapBiasF,
  LightingContextStr,
  ShadowContextStr
}
enum H3DJoint	{
  JointIndexI = 400
}
enum H3DCamera	{
  PipeResI = 600,
  OutTexResI,
  OutBufIndexI,
  LeftPlaneF,
  RightPlaneF,
  BottomPlaneF,
  TopPlaneF,
  NearPlaneF,
  FarPlaneF,
  ViewportXI,
  ViewportYI,
  ViewportWidthI,
  ViewportHeightI,
  OrthoI,
  OccCullingI
}
enum H3DEmitter	{
  MatResI = 700,
  PartEffResI,
  MaxCountI,
  RespawnCountI,
  DelayF,
  EmissionRateF,
  SpreadAngleF,
  ForceF3
}
enum H3DModelUpdateFlags{
  Animation = 1,
  Geometry = 2
}
*/
