require 'ffi'

module Hyperdrive
  extend FFI::Library
  ffi_lib 'hyperdrive'

  attach_function(:init, :hyperdrive_init, [], :void)
  init

  attach_function(:trace_count, :hyperdrive_trace_count, [], :uint64)
end
