require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("bytebuffer") do |bb|
  bb.profile = ENV.fetch("RB_SYS_CARGO_PROFILE", :dev).to_sym
  bb.ext_dir = "."
  bb.clean_after_install = true
  bb.auto_install_rust_toolchain = true
end
