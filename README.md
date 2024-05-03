# xgpu

xgpu is a wrapper around vulkan and directx12 that provides a unified interface for both APIs.
For performance reasons, xgpu does not use dynamic dispatch and allow for switching between APIs at runtime.
The user must choose the API at compile time.
The user can however compile two versions of their engine and choose between those at runtime.

## Status

xgpu is currently in development and is not yet ready for production use.
Many api decisions are still being made to minimize the amount of overhead and added indirection to the underlying APIs.