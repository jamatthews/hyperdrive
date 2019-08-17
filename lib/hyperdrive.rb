require 'ffi'

module Hyperdrive
  extend FFI::Library
  ffi_lib 'hyperdrive'

  attach_function(:init, :hyperdrive_init, [], :void)
  init

  # attach_function(:begin_trace, :hyperdrive_begin_trace, [], :void)
  # attach_function(:stop_recording, :hyperdrive_stop_recording, [], :void)
  # attach_function(:dump_trace, :hyperdrive_dump_trace, [], :void)
  # attach_function(:recording, :hyperdrive_recording, [], :int64)

  def self.recording?
    return true if recording == 1
    false
  end
end
